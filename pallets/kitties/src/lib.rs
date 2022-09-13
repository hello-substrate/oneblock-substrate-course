#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::Bounded;
	use sp_std::fmt::Debug;

	// ----------------------------------------------------------------
	type KittyDna = [u8; 16];
	/// 定义余额类型
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	/// 小猫的信息
	#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub KittyDna);

	/// 定义账户类型
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	/// web 小猫的信息
	#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct WebKitty<T: Config> {
		pub id: T::KittyIndex,
		pub dna: KittyDna,
		pub owner: AccountIdOf<T>,
	}
	// ----------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// 随机数
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		/// 可质押的货币
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		/// 作业2: 小猫 id 在 runtime 中指定具体类型
		type KittyIndex: Parameter
			+ Member
			+ sp_runtime::traits::AtLeast32BitUnsigned
			+ codec::Codec
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaxEncodedLen
			+ TypeInfo;
		/// 最大拥有的小猫数量
		#[pallet::constant]
		type MaxOwnerKitty: Get<u32>;
		/// 小猫的质押金额
		#[pallet::constant]
		type StakeAmount: Get<BalanceOf<Self>>;
	}

	/// kitty 的自增 id
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	/// kitty 的信息
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;
	/// 前端需要读取的 kitty 信息
	#[pallet::storage]
	#[pallet::getter(fn web_kitties)]
	pub type WebKitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, WebKitty<T>>;
	/// kitty 的主人
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;
	/// 作业3: 获取账户下所有的小猫
	#[pallet::storage]
	#[pallet::getter(fn user_kitties)]
	pub type UserKitties<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::KittyIndex, T::MaxOwnerKitty>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { who: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty },
		Bred { who: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: T::KittyIndex },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// kitty_id 无效/溢出
		InvalidKittyId,
		/// kitty_id 相同
		SameKittyId,
		/// 不是小猫的主人
		NotOwner,
		/// 超过最大拥有小猫数量
		OverflowMaxOwnerKitty,
		/// 质押金额不足
		InsufficientStakeAmount,
		/// 质押金额不足
		DuplicateKitty,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// 创建一只小猫
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// 生成小猫
			let dna = Self::gen_dna(&who);
			Self::mint_kitty(who.clone(), dna)?;
			Ok(())
		}

		/// 繁育一只小猫
		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::breed_kitty(who.clone(), kitty_id_1, kitty_id_2)?;
			Ok(())
		}

		/// 转让一只小猫
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::transfer_kitty(who, to, kitty_id)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// 创建一只小猫
		fn mint_kitty(who: T::AccountId, dna: KittyDna) -> Result<T::KittyIndex, DispatchError> {
			// 获取小猫自增 id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty = Kitty(dna);
			// 入库
			Kitties::<T>::insert(&kitty_id, &kitty);
			KittyOwner::<T>::insert(&kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1u32.into());
			UserKitties::<T>::try_append(&who, kitty_id)
				.map_err(|_| Error::<T>::OverflowMaxOwnerKitty)?;
			// 作业4: 质押金额
			T::Currency::reserve(&who, T::StakeAmount::get())
				.map_err(|_| Error::<T>::InsufficientStakeAmount)?;

			WebKitties::<T>::insert(&kitty_id, &WebKitty { id: kitty_id, dna, owner: who.clone() });

			Self::deposit_event(Event::<T>::Created { who, kitty_id, kitty });
			Ok(kitty_id)
		}
		// 繁育一只小猫
		fn breed_kitty(
			who: T::AccountId,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> Result<T::KittyIndex, DispatchError> {
			// 父母不能相同
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			// 检查父母是否存在
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;
			// 获取小猫自增 id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
			// 生成小猫 dna
			let rand_dna = Self::gen_dna(&who);
			// dna 依靠父母重构
			let mut new_dna = KittyDna::default();
			for i in 0..kitty_1.0.len() {
				// 0 choose kitty2, and 1 choose kitty1
				new_dna[i] = (kitty_1.0[i] & rand_dna[i]) | (kitty_2.0[i] & !rand_dna[i]);
			}
			let kitty = Kitty(new_dna);
			// 入库
			Kitties::<T>::insert(&kitty_id, &kitty);
			KittyOwner::<T>::insert(&kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1u32.into());
			UserKitties::<T>::try_append(&who, kitty_id)
				.map_err(|_| Error::<T>::OverflowMaxOwnerKitty)?;
			// 质押金额
			T::Currency::reserve(&who, T::StakeAmount::get())
				.map_err(|_| Error::<T>::InsufficientStakeAmount)?;

			WebKitties::<T>::insert(
				&kitty_id,
				&WebKitty { id: kitty_id, dna: new_dna, owner: who.clone() },
			);

			Self::deposit_event(Event::<T>::Bred { who, kitty_id, kitty });
			Ok(kitty_id)
		}

		// 转让一只小猫
		fn transfer_kitty(
			from: T::AccountId,
			to: T::AccountId,
			kitty_id: T::KittyIndex,
		) -> DispatchResult {
			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
			// 检查小猫是否自己的
			let owner = Self::kitty_owner(kitty_id);
			ensure!(owner == Some(from.clone()), Error::<T>::NotOwner);
			KittyOwner::<T>::set(kitty_id, Some(to.clone()));
			// 将之前的小猫数组清除
			UserKitties::<T>::try_mutate(&from, |vec| -> DispatchResult {
				// 获取索引
				if let Some(index) = vec.iter().position(|&x| x == kitty_id) {
					vec.swap_remove(index);
					Ok(())
				} else {
					Err(Error::<T>::InvalidKittyId.into())
				}
			})?;
			// kitty 信息更新
			let kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
			WebKitties::<T>::insert(
				&kitty_id,
				&WebKitty { id: kitty_id, dna: kitty.0, owner: to.clone() },
			);

			UserKitties::<T>::try_mutate(&to, |vec| vec.try_push(kitty_id))
				.map_err(|_| Error::<T>::OverflowMaxOwnerKitty)?;

			// 新的主人质押金额
			T::Currency::reserve(&to, T::StakeAmount::get())
				.map_err(|_| Error::<T>::InsufficientStakeAmount)?;
			// 之前的主人解除质押金额. 由于这个方法不会失败,放在最后执行。
			let _ = T::Currency::unreserve(&from, T::StakeAmount::get());

			Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
			Ok(())
		}
		/// 获取小猫的 id
		fn get_next_id() -> Result<T::KittyIndex, ()> {
			let max = T::KittyIndex::max_value();
			let kitty_id = Self::next_kitty_id();
			if kitty_id < max {
				Ok(kitty_id)
			} else {
				Err(())
			}
		}
		/// 获取小猫的 信息
		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}
		/// 生成小猫的 dna
		fn gen_dna(who: &T::AccountId) -> KittyDna {
			let payload = (
				T::Randomness::random(&b"dna"[..]),
				who,
				frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
				frame_system::Pallet::<T>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
