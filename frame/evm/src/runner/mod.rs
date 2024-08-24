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

pub mod stack;

use alloc::vec::Vec;
// Substrate
use sp_core::{H160, H256, U256};
use sp_runtime::DispatchError;
// Frontier
use fp_evm::{CallInfo, CreateInfo, TransactionPov};

use crate::{Config, Weight};

pub trait Runner<T: Config> {
	type Error: Into<DispatchError>;

	fn validate(
		source: H160,
		target: Option<H160>,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		is_transactional: bool,
		transaction_pov: Option<TransactionPov>,
		evm_config: &evm::Config,
	) -> Result<(), RunnerError<Self::Error>>;

	fn call(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		is_transactional: bool,
		validate: bool,
		transaction_pov: Option<TransactionPov>,
		config: &evm::Config,
	) -> Result<CallInfo, RunnerError<Self::Error>>;

	fn create(
		source: H160,
		init: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		is_transactional: bool,
		validate: bool,
		transaction_pov: Option<TransactionPov>,
		config: &evm::Config,
	) -> Result<CreateInfo, RunnerError<Self::Error>>;

	fn create2(
		source: H160,
		init: Vec<u8>,
		salt: H256,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		is_transactional: bool,
		validate: bool,
		transaction_pov: Option<TransactionPov>,
		config: &evm::Config,
	) -> Result<CreateInfo, RunnerError<Self::Error>>;
}

#[derive(Debug, PartialEq)]
pub struct RunnerError<E> {
	pub error: E,
	pub weight: Weight,
}
