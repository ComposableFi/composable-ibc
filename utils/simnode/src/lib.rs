use parachain_inherent::ParachainInherentData;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use sp_runtime::generic::Era;
use std::sync::Arc;
use substrate_simnode::{ChainInfo, FullClientFor, RpcHandlerArgs, SignatureVerificationOverride};

pub struct ParachainCli;

impl substrate_simnode::SimnodeCli for ParachainCli {
	type CliConfig = sc_cli::RunCmd;
	type SubstrateCli = parachain_node::cli::Cli;

	fn cli_config(cli: &Self::SubstrateCli) -> &Self::CliConfig {
		&cli.run.base
	}

	fn log_filters(_cli_config: &Self::CliConfig) -> Result<String, sc_cli::Error> {
		Ok("pallet_ibc=trace".to_string())
	}
}

pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions =
		(frame_benchmarking::benchmarking::HostFunctions, SignatureVerificationOverride);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		parachain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		parachain_runtime::native_version()
	}
}

pub struct ParachainRuntimeChainInfo;
impl ChainInfo for ParachainRuntimeChainInfo {
	type Block = parachain_runtime::opaque::Block;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = parachain_runtime::Runtime;
	type RuntimeApi = parachain_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	type BlockImport = Arc<FullClientFor<Self>>;
	type InherentDataProviders = (
		SlotTimestampProvider,
		sp_consensus_aura::inherents::InherentDataProvider,
		ParachainInherentData,
	);
	type SignedExtras = parachain_runtime::SignedExtra;
	type Cli = ParachainCli;

	fn create_rpc_io_handler<SC>(deps: RpcHandlerArgs<Self, SC>) -> jsonrpsee::RpcModule<()>
	where
		<<Self as ChainInfo>::RuntimeApi as sp_api::ConstructRuntimeApi<
			Self::Block,
			FullClientFor<Self>,
		>>::RuntimeApi: sp_api::Core<Self::Block>
			+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Self::Block>,
	{
		let full_deps = parachain_node::rpc::FullDeps {
			client: deps.client,
			pool: deps.pool,
			deny_unsafe: deps.deny_unsafe,
			chain_props: Default::default(),
		};
		parachain_node::rpc::create_full(full_deps).expect("Rpc to be initialized")
	}

	fn signed_extras(
		from: <Self::Runtime as frame_system::pallet::Config>::AccountId,
	) -> Self::SignedExtras {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(from);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}

#[cfg(test)]
mod tests {
	use crate::ParachainRuntimeChainInfo;
	use frame_benchmarking::frame_support::codec::Encode;
	use grandpa_client_primitives::{justification::GrandpaJustification, Commit, FinalityProof};
	use ibc::{
		core::{
			ics02_client::{
				context::ClientTypes,
				msgs::{create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient},
			},
			ics24_host::identifier::ClientId,
		},
		protobuf::Protobuf,
		signer::Signer,
		timestamp::Timestamp,
		tx_msg::Msg,
	};
	use ics10_grandpa::client_message::RelayChainHeader;
	use pallet_ibc::{
		light_clients::{AnyClient, AnyClientMessage, AnyClientState, AnyConsensusState},
		Any,
	};
	use sp_runtime::{testing::H256, SaturatedConversion};
	use std::str::FromStr;
	use substrate_simnode::ChainInfo;

	#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
	pub struct LocalClientTypes;

	impl ClientTypes for LocalClientTypes {
		type AnyClientMessage = AnyClientMessage;
		type AnyClientState = AnyClientState;
		type AnyConsensusState = AnyConsensusState;
		type ClientDef = AnyClient;
	}

	#[test]
	fn check_client_expiry() {
		substrate_simnode::parachain_node::<ParachainRuntimeChainInfo, _, _>(|node| async move {
			let sudo = node
				.with_state(
					None,
					sudo::Pallet::<<ParachainRuntimeChainInfo as ChainInfo>::Runtime>::key,
				)
				.unwrap();

			let client_state = ics10_grandpa::client_state::ClientState {
				relay_chain: Default::default(),
				latest_relay_height: 100,
				latest_relay_hash: Default::default(),
				frozen_height: None,
				latest_para_height: 10,
				para_id: 100,
				current_set_id: 1,
				current_authorities: Default::default(),
				_phantom: Default::default(),
			};

			let time = core::time::Duration::from_millis(1_000_000_000u64.saturating_mul(1000));
			let consensus_state = ics10_grandpa::consensus_state::ConsensusState {
				timestamp: Timestamp::from_nanoseconds(time.as_nanos().saturated_into::<u64>())
					.unwrap()
					.into_tm_time()
					.unwrap(),
				root: H256::zero().as_bytes().to_vec().into(),
			};

			let justification = GrandpaJustification::<RelayChainHeader> {
				round: 1,
				commit: Commit::<RelayChainHeader> {
					target_hash: Default::default(),
					target_number: Default::default(),
					precommits: Default::default(),
				},
				votes_ancestries: vec![],
			};
			let grandpa_header = ics10_grandpa::client_message::Header {
				finality_proof: FinalityProof {
					block: Default::default(),
					justification: justification.encode(),
					unknown_headers: Default::default(),
				},
				parachain_headers: Default::default(),
			};
			let msg_create_client = MsgCreateAnyClient::<LocalClientTypes> {
				client_state: AnyClientState::Grandpa(client_state),
				signer: Signer::from_str("relayer").unwrap(),
				consensus_state: AnyConsensusState::Grandpa(consensus_state),
			};

			let msg_create_client = Any {
				type_url: msg_create_client.type_url().as_bytes().to_vec(),
				value: msg_create_client.encode_vec(),
			};
			let msg_update_client = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: ClientId::new("10-grandpa", 0).unwrap(),
				client_message: AnyClientMessage::Grandpa(
					ics10_grandpa::client_message::ClientMessage::Header(grandpa_header),
				),
				signer: Signer::from_str("relayer").unwrap(),
			};

			let msg_update_client = Any {
				type_url: msg_update_client.type_url().as_bytes().to_vec(),
				value: msg_update_client.encode_vec(),
			};
			let call =
				pallet_ibc::Call::<<ParachainRuntimeChainInfo as ChainInfo>::Runtime>::deliver {
					messages: vec![msg_create_client],
				};
			node.submit_extrinsic(call, sudo.clone()).await?;
			let relaychain = light_client_common::RelayChain::default();
			// Fast forward to a time beyond the trusting period of the client
			let blocks_to_seal = (relaychain.trusting_period().as_secs() / 12).saturating_add(100);
			node.seal_blocks(blocks_to_seal as usize).await;
			let call =
				pallet_ibc::Call::<<ParachainRuntimeChainInfo as ChainInfo>::Runtime>::deliver {
					messages: vec![msg_update_client],
				};

			node.submit_extrinsic(call, sudo).await?;
			node.seal_blocks(1).await;
			let events = node.events(None);
			assert!(events.into_iter().any(|ev| {
				match ev.event {
					parachain_runtime::Event::Ibc(pallet_ibc::Event::Events { events }) => {
						for ev in events {
							if let Err(pallet_ibc::errors::IbcError::Ics02Client { message }) = ev {
								let err_str = String::from_utf8(message).unwrap();
								// Assert that client expiry was validated
								return err_str.contains("HeaderNotWithinTrustPeriod")
							}
						}
						return false
					},
					_ => false,
				}
			}));
			Ok(())
		})
		.unwrap();
	}
}
