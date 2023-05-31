use std::{path::PathBuf, sync::Arc, time::Duration};

use ethers::{
	contract::{abigen, ContractFactory},
	core::utils::Anvil,
	middleware::SignerMiddleware,
	prelude::*,
	providers::{Http, Provider},
	signers::{LocalWallet, Signer},
	solc::{
		artifacts::{
			output_selection::OutputSelection, DebuggingSettings, Libraries, Optimizer,
			OptimizerDetails, Settings,
		},
		Artifact, Project, ProjectPathsConfig,
	},
};

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

	todo!("tests");
}
