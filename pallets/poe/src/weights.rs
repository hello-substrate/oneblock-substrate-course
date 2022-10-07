// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-07, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `macbookdeMacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_poe
// --extrinsic=*
// --steps=20
// --repeat=10
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/poe/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_poe.
pub trait WeightInfo {
	fn create_claim(d: u32, ) -> Weight;
	fn revoke_claim(d: u32, ) -> Weight;
	fn transfer_claim(d: u32, ) -> Weight;
}

/// Weights for pallet_poe using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(d: u32, ) -> Weight {
		Weight::from_ref_time(17_169_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(d as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		Weight::from_ref_time(23_825_000 as u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(d as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(_d: u32, ) -> Weight {
		Weight::from_ref_time(26_496_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(d: u32, ) -> Weight {
		Weight::from_ref_time(17_169_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(d as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		Weight::from_ref_time(23_825_000 as u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(d as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(_d: u32, ) -> Weight {
		Weight::from_ref_time(26_496_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
