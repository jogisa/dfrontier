// This file is part of Frontier.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_hotfix_sufficients
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `archlinux`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/frontier-template-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_hotfix_sufficients
// --extrinsic=hotfix_inc_account_sufficients
// --execution=wasm
// --wasm-execution=compiled
// --output=weights.rs
// --header=./.maintain/HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_hotfix_sufficients.
pub trait WeightInfo {
	fn hotfix_inc_account_sufficients(n: u32, ) -> Weight;
}

/// Weights for pallet_hotfix_sufficients using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: System Number (r:1 w:0)
	/// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System ExecutionPhase (r:1 w:0)
	/// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: System EventCount (r:1 w:1)
	/// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Events (r:1 w:1)
	/// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 1000]`.
	fn hotfix_inc_account_sufficients(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (124 ±0)`
		//  Estimated: `6572 + n * (2715 ±0)`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(2_000_000, 6572)
			// Standard Error: 12_231
			.saturating_add(Weight::from_parts(15_224_397, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2715).saturating_mul(n.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: System Number (r:1 w:0)
	/// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System ExecutionPhase (r:1 w:0)
	/// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: System EventCount (r:1 w:1)
	/// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Events (r:1 w:1)
	/// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 1000]`.
	fn hotfix_inc_account_sufficients(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (124 ±0)`
		//  Estimated: `6572 + n * (2715 ±0)`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(2_000_000, 6572)
			// Standard Error: 12_231
			.saturating_add(Weight::from_parts(15_224_397, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2715).saturating_mul(n.into()))
	}
}
