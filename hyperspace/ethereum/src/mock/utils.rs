#![allow(dead_code)]

use elliptic_curve::pkcs8::der::pem;
use std::{path::PathBuf, sync::Arc, time::Duration};

use crate::{
	config::EthereumClientConfig,
	contract::UnwrapContractError,
	utils::{DeployYuiIbc, ProviderImpl},
};
use ethers::{
	abi::{Detokenize, Tokenize},
	core::utils::Anvil,
	middleware::SignerMiddleware,
	prelude::*,
	providers::{Http, Middleware, Provider},
	signers::{LocalWallet, Signer},
	utils::AnvilInstance,
};
use ethers_solc::Artifact;
use futures::SinkExt;
use ibc::core::ics24_host::identifier::ClientId;

pub const USE_GETH: bool = true;

pub const ETH_NODE_PORT: u16 = 8545;
pub const ETH_NODE_PORT_WS: u16 = 8546;
pub const BEACON_NODE_PORT: u16 = 3500;
pub const PRIVATE_KEY: &str = "/Users/vmark/work/centauri-private/hyperspace/ethereum/keys/0x73db010c3275eb7a92e5c38770316248f4c644ee";

#[track_caller]
pub fn yui_ibc_solidity_path() -> PathBuf {
	let base = env!("CARGO_MANIFEST_DIR");
	let default = PathBuf::from(base).join("yui-ibc-solidity");

	if let Ok(path) = std::env::var("YUI_IBC_SOLIDITY_PATH") {
		path.into()
	} else {
		default
	}
}

#[track_caller]
pub async fn spawn_anvil() -> (AnvilInstance, Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) {
	#[cfg(not(feature = "no_beacon"))]
	let anvil = Anvil::new().spawn();
	#[cfg(feature = "no_beacon")]
	let anvil = Anvil::new().port(8545u16).spawn();

	println!("{:?}", std::env::current_dir().unwrap());
	let wallet: LocalWallet = if USE_GETH {
		LocalWallet::decrypt_keystore(
			PRIVATE_KEY,
			std::env::var("KEY_PASS").expect("KEY_PASS not set"),
		)
		.unwrap()
		.into()
	} else {
		anvil.keys()[0].clone().into()
	};

	let endpoint =
		if USE_GETH { format!("http://localhost:{}", ETH_NODE_PORT) } else { anvil.endpoint() };
	let provider = Provider::<Http>::try_from(endpoint)
		.unwrap()
		.interval(Duration::from_millis(10u64));
	let chain_id =
		if USE_GETH { provider.get_chainid().await.unwrap().as_u64() } else { anvil.chain_id() };
	let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
	let client = Arc::new(client);

	(anvil, client)
}

pub mod mock {
	use ethers::abi::Token;
	use prost::Message;

	#[derive(Clone, PartialEq, ::prost::Message)]
	pub struct ClientState {
		#[prost(message, required, tag = "1")]
		pub height: ibc_proto::ibc::core::client::v1::Height,
	}

	#[derive(Clone, PartialEq, ::prost::Message)]
	pub struct ConsensusState {
		#[prost(uint64, tag = "1")]
		pub timestamp: u64,
	}

	pub fn create_client_msg(kind: &str) -> Token {
		let client_state_bytes = client_state_bytes();

		let consensus_state_bytes = ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ConsensusState".into(),
			value: ConsensusState { timestamp: 1 }.encode_to_vec(),
		}
		.encode_to_vec();

		Token::Tuple(vec![
			Token::String(kind.into()),
			Token::Bytes(client_state_bytes),
			Token::Bytes(consensus_state_bytes),
		])
	}

	pub fn client_state_bytes() -> Vec<u8> {
		ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ClientState".into(),
			value: ClientState {
				height: ibc_proto::ibc::core::client::v1::Height {
					revision_number: 0,
					revision_height: 1,
				},
			}
			.encode_to_vec(),
		}
		.encode_to_vec()
	}
}

pub async fn hyperspace_ethereum_client_fixture(
	anvil: &AnvilInstance,
	yui_ibc: DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
	db_url: &str,
	redis_url: &str,
) -> EthereumClientConfig {
	let endpoint =
		if USE_GETH { format!("http://localhost:{}", ETH_NODE_PORT) } else { anvil.endpoint() };
	let wallet_path = if USE_GETH { Some(PRIVATE_KEY.to_string()) } else { None };

	let wallet = if !USE_GETH {
		Some(
			anvil.keys()[0]
				.clone()
				.to_sec1_pem(pem::LineEnding::CR)
				.unwrap()
				.as_str()
				.to_owned()
				.to_string(),
		)
	} else {
		None
	};

	let jwt_secret_path = if !USE_GETH {
		None
	} else {
		Some("/Users/vmark/work/eth-pos-devnet/execution/jwtsecret".to_string())
	};

	EthereumClientConfig {
		http_rpc_url: endpoint.parse().unwrap(),
		ws_rpc_url: format!("ws://127.0.0.1:{}", ETH_NODE_PORT_WS).parse().unwrap(),
		beacon_rpc_url: format!("http://localhost:{}", BEACON_NODE_PORT).parse().unwrap(),
		mnemonic: None,
		max_block_weight: 1,
		private_key: wallet,
		private_key_path: wallet_path,
		name: "ethereum-client".into(),
		client_id: Some(ClientId::new("07-tendermint", 0).unwrap()),
		connection_id: None,
		channel_whitelist: vec![],
		commitment_prefix: "696263".into(),
		// commitment_prefix: "424242".into(),
		wasm_code_id: None,
		ics20_transfer_bank_address: yui_ibc.ics20_transfer_bank.clone().map(|b| b.address()),
		ics20_bank_address: yui_ibc.ics20_bank.clone().map(|b| b.address()),
		diamond_address: Some(yui_ibc.diamond.address()),
		tendermint_address: yui_ibc.tendermint.clone().map(|x| x.address()),
		diamond_facets: yui_ibc
			.deployed_facets
			.iter()
			.map(|f| (f.abi_name(), f.contract().address()))
			.collect(),
		yui: Some(yui_ibc),
		client_type: "07-tendermint".into(),
		jwt_secret_path,
		indexer_pg_url: db_url.parse().unwrap(),
		indexer_redis_url: redis_url.parse().unwrap(),
		anvil: None,
	}
}
