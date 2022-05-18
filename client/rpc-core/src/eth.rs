// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Eth rpc interface.

use ethereum_types::{H160, H256, H64, U256, U64};
use jsonrpsee::{core::async_trait, core::RpcResult as Result, proc_macros::rpc};

use crate::types::*;

/// Eth rpc interface.
#[rpc(server)]
#[async_trait]
pub trait EthApi {
	// ########################################################################
	// Client
	// ########################################################################

	/// Returns protocol version encoded as a string (quotes are necessary).
	#[method(name = "eth_protocolVersion")]
	fn protocol_version(&self) -> Result<u64>;

	/// Returns an object with data about the sync status or false. (wtf?)
	#[method(name = "eth_syncing")]
	fn syncing(&self) -> Result<SyncStatus>;

	/// Returns block author.
	#[method(name = "eth_coinbase")]
	fn author(&self) -> Result<H160>;

	/// Returns accounts list.
	#[method(name = "eth_accounts")]
	fn accounts(&self) -> Result<Vec<H160>>;

	/// Returns highest block number.
	#[method(name = "eth_blockNumber")]
	fn block_number(&self) -> Result<U256>;

	/// Returns the chain ID used for transaction signing at the
	/// current best block. None is returned if not
	/// available.
	#[method(name = "eth_chainId")]
	fn chain_id(&self) -> Result<Option<U64>>;

	// ########################################################################
	// Block
	// ########################################################################

	/// Returns block with given hash.
	#[method(name = "eth_getBlockByHash")]
	async fn block_by_hash(&self, hash: H256, full: bool) -> Result<Option<RichBlock>>;

	/// Returns block with given number.
	#[method(name = "eth_getBlockByNumber")]
	async fn block_by_number(&self, number: BlockNumber, full: bool) -> Result<Option<RichBlock>>;

	/// Returns the number of transactions in a block with given hash.
	#[method(name = "eth_getBlockTransactionCountByHash")]
	fn block_transaction_count_by_hash(&self, hash: H256) -> Result<Option<U256>>;

	/// Returns the number of transactions in a block with given block number.
	#[method(name = "eth_getBlockTransactionCountByNumber")]
	fn block_transaction_count_by_number(&self, number: BlockNumber) -> Result<Option<U256>>;

	/// Returns the number of uncles in a block with given hash.
	#[method(name = "eth_getUncleCountByBlockHash")]
	fn block_uncles_count_by_hash(&self, hash: H256) -> Result<U256>;

	/// Returns the number of uncles in a block with given block number.
	#[method(name = "eth_getUncleCountByBlockNumber")]
	fn block_uncles_count_by_number(&self, number: BlockNumber) -> Result<U256>;

	/// Returns an uncles at given block and index.
	#[method(name = "eth_getUncleByBlockHashAndIndex")]
	fn uncle_by_block_hash_and_index(&self, hash: H256, index: Index) -> Result<Option<RichBlock>>;

	/// Returns an uncles at given block and index.
	#[method(name = "eth_getUncleByBlockNumberAndIndex")]
	fn uncle_by_block_number_and_index(
		&self,
		number: BlockNumber,
		index: Index,
	) -> Result<Option<RichBlock>>;

	// ########################################################################
	// Transaction
	// ########################################################################

	/// Get transaction by its hash.
	#[method(name = "eth_getTransactionByHash")]
	async fn transaction_by_hash(&self, hash: H256) -> Result<Option<Transaction>>;

	/// Returns transaction at given block hash and index.
	#[method(name = "eth_getTransactionByBlockHashAndIndex")]
	async fn transaction_by_block_hash_and_index(
		&self,
		hash: H256,
		index: Index,
	) -> Result<Option<Transaction>>;

	/// Returns transaction by given block number and index.
	#[method(name = "eth_getTransactionByBlockNumberAndIndex")]
	async fn transaction_by_block_number_and_index(
		&self,
		number: BlockNumber,
		index: Index,
	) -> Result<Option<Transaction>>;

	/// Returns transaction receipt by transaction hash.
	#[method(name = "eth_getTransactionReceipt")]
	async fn transaction_receipt(&self, hash: H256) -> Result<Option<Receipt>>;

	// ########################################################################
	// State
	// ########################################################################

	/// Returns balance of the given account.
	#[method(name = "eth_getBalance")]
	fn balance(&self, address: H160, number: Option<BlockNumber>) -> Result<U256>;

	/// Returns content of the storage at given address.
	#[method(name = "eth_getStorageAt")]
	fn storage_at(&self, address: H160, index: U256, number: Option<BlockNumber>) -> Result<H256>;

	/// Returns the number of transactions sent from given address at given time (block number).
	#[method(name = "eth_getTransactionCount")]
	fn transaction_count(&self, address: H160, number: Option<BlockNumber>) -> Result<U256>;

	/// Returns the code at given address at given time (block number).
	#[method(name = "eth_getCode")]
	fn code_at(&self, address: H160, number: Option<BlockNumber>) -> Result<Bytes>;

	// ########################################################################
	// Execute
	// ########################################################################

	/// Call contract, returning the output data.
	#[method(name = "eth_call")]
	fn call(&self, request: CallRequest, number: Option<BlockNumber>) -> Result<Bytes>;

	/// Estimate gas needed for execution of given contract.
	#[method(name = "eth_estimateGas")]
	async fn estimate_gas(&self, request: CallRequest, number: Option<BlockNumber>)
		-> Result<U256>;

	// ########################################################################
	// Fee
	// ########################################################################

	/// Returns current gas_price.
	#[method(name = "eth_gasPrice")]
	fn gas_price(&self) -> Result<U256>;

	/// Introduced in EIP-1159 for getting information on the appropriate priority fee to use.
	#[method(name = "eth_feeHistory")]
	fn fee_history(
		&self,
		block_count: U256,
		newest_block: BlockNumber,
		reward_percentiles: Option<Vec<f64>>,
	) -> Result<FeeHistory>;

	/// Introduced in EIP-1159, a Geth-specific and simplified priority fee oracle.
	/// Leverages the already existing fee history cache.
	#[method(name = "eth_maxPriorityFeePerGas")]
	fn max_priority_fee_per_gas(&self) -> Result<U256>;

	// ########################################################################
	// Mining
	// ########################################################################

	/// Returns true if client is actively mining new blocks.
	#[method(name = "eth_mining")]
	fn is_mining(&self) -> Result<bool>;

	/// Returns the number of hashes per second that the node is mining with.
	#[method(name = "eth_hashrate")]
	fn hashrate(&self) -> Result<U256>;

	/// Returns the hash of the current block, the seedHash, and the boundary condition to be met.
	#[method(name = "eth_getWork")]
	fn work(&self) -> Result<Work>;

	/// Used for submitting mining hashrate.
	#[method(name = "eth_submitHashrate")]
	fn submit_hashrate(&self, hashrate: U256, id: H256) -> Result<bool>;

	/// Used for submitting a proof-of-work solution.
	#[method(name = "eth_submitWork")]
	fn submit_work(&self, nonce: H64, pow_hash: H256, mix_digest: H256) -> Result<bool>;

	// ########################################################################
	// Submit
	// ########################################################################

	/// Sends transaction; will block waiting for signer to return the
	/// transaction hash.
	#[method(name = "eth_sendTransaction")]
	fn send_transaction(&self, request: TransactionRequest) -> Result<H256>;

	/// Sends signed transaction, returning its hash.
	#[method(name = "eth_sendRawTransaction")]
	fn send_raw_transaction(&self, bytes: Bytes) -> Result<H256>;
}

/// Eth filters rpc api (polling).
#[rpc(server)]
pub trait EthFilterApi {
	/// Returns id of new filter.
	#[method(name = "eth_newFilter")]
	fn new_filter(&self, filter: Filter) -> Result<U256>;

	/// Returns id of new block filter.
	#[method(name = "eth_newBlockFilter")]
	fn new_block_filter(&self) -> Result<U256>;

	/// Returns id of new block filter.
	#[method(name = "eth_newPendingTransactionFilter")]
	fn new_pending_transaction_filter(&self) -> Result<U256>;

	/// Returns filter changes since last poll.
	#[method(name = "eth_getFilterChanges")]
	fn filter_changes(&self, index: Index) -> Result<FilterChanges>;

	/// Returns all logs matching given filter (in a range 'from' - 'to').
	#[method(name = "eth_getFilterLogs")]
	fn filter_logs(&self, index: Index) -> Result<Vec<Log>>;

	/// Uninstalls filter.
	#[method(name = "eth_uninstallFilter")]
	fn uninstall_filter(&self, index: Index) -> Result<bool>;

	/// Returns logs matching given filter object.
	#[method(name = "eth_getLogs")]
	fn logs(&self, filter: Filter) -> Result<Vec<Log>>;
}
