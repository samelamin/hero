//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use paid_chain_runtime::{Hash, opaque::Block, AccountId, Balance, Index};
use sc_client_api::backend::{Backend, StateBackend, StorageProvider};

// Imports to support Ethereum RPC.
use sc_transaction_pool::{ChainApi, Pool};
use std::collections::BTreeMap;
use fc_rpc::{
	EthBlockDataCache, OverrideHandle, RuntimeApiStorageOverride,
};
use sp_runtime::traits::BlakeTwo256;
use fc_rpc_core::types::FeeHistoryCache;

use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sc_network::NetworkService;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Full client dependencies
pub struct FullDeps<C, P, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Maximum fee history cache size.
	pub fee_history_limit: u64,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, BE, A>(deps: FullDeps<C, P, A>) -> RpcExtension
where
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C: StorageProvider<Block, BE>,
	C: StorageProvider<Block, BE>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: BlockBuilder<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	A: ChainApi<Block = Block> + 'static,
{
	use fc_rpc::{EthApi, EthApiServer, NetApi, NetApiServer, Web3Api, Web3ApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		 client,
		 pool,
		 deny_unsafe,
		 graph,
		 network ,
		 backend,
		 is_authority,
		 fee_history_limit,
		 fee_history_cache,
		} = deps;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe)));
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));
	io.extend_with(NetApiServer::to_delegate(NetApi::new(
		client.clone(),
		network.clone(),
		// Whether to format the `peer_count` response as Hex (default) or not.
		true,
	)));

    io.extend_with(
    		Web3ApiServer::to_delegate(Web3Api::new(
    			client.clone(),
    		))
    	);

	// We won't use the override feature
	let overrides = Arc::new(OverrideHandle {
		schemas: BTreeMap::new(),
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	});

	// Nor any signers
	let signers = Vec::new();

	// Limit the number of queryable logs. Temporarily hard-coded.
	// Best practice is to have this be a CLI input.
	let max_past_logs = 1024;

	// Reasonable default caching inspired by the frontier template
	let block_data_cache = Arc::new(EthBlockDataCache::new(50, 50));

	io.extend_with(EthApiServer::to_delegate(EthApi::new(
		client.clone(),
		pool.clone(),
		graph,
		paid_chain_runtime::TransactionConverter,
		network.clone(),
		signers,
		overrides.clone(),
		backend.clone(),
		is_authority,
		max_past_logs,
		block_data_cache.clone(),
		fc_rpc::format::Geth,
		fee_history_limit,
		fee_history_cache,
	)));

	io
}
