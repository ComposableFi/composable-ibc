use std::{path::PathBuf, sync::Arc, time::Duration, ops::Deref};

use ethers::{
	contract::{abigen, ContractFactory},
	core::utils::Anvil,
	middleware::SignerMiddleware,
	prelude::*,
	providers::{Http, Provider},
	signers::{LocalWallet},
	solc::{
		artifacts::{
			output_selection::OutputSelection, DebuggingSettings, Libraries, Optimizer,
			OptimizerDetails, Settings,
		},
		Artifact, Project, ProjectPathsConfig,
	},
};
use hyperspace_ethereum::config::Config;

mod utils;

#[tokio::test]
async fn test() {
	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path);
	let (anvil, client) = utils::spawn_anvil();
	let utils::DeployYuiIbc {
		ibc_client,
		ibc_connection,
		ibc_channel_handshake,
		ibc_packet,
		ownable_ibc_handler,
	} = utils::deploy_yui_ibc(project_output, client).await;

	// let wallet: LocalWallet = anvil.keys()[0].clone().into();

	let client = hyperspace_ethereum::client::Client::new(Config {
		http_rpc_url: anvil.endpoint().parse().unwrap(),
		ws_rpc_url: Default::default(),
		ibc_handler_address: ownable_ibc_handler.address(),
		ibc_packet_address: ibc_packet.address(),
		ibc_client_address: ibc_client.address(),
		ibc_connection_address: ibc_connection.address(),
		ibc_channel_handshake_address: ibc_channel_handshake.address(),
		mnemonic: None,
		private_key: Some(
			anvil.keys()[0]
				.to_sec1_pem(elliptic_curve::pkcs8::LineEnding::LF)
				.unwrap()
				.deref()
				.clone(),
		),
		name: "foo".into(),
		client_id: None,
		connection_id: None,
		channel_whitelist: vec![],
		commitment_prefix: "".into(),
	})
	.await
	.unwrap();
}
