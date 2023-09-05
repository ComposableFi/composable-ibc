#![allow(dead_code)]

use cast::hashbrown::HashSet;
use std::{
	collections::HashMap,
	fs::File,
	iter::once,
	path::{Path, PathBuf},
	sync::Arc,
	time::Duration,
};

use ethers::{
	abi::{Detokenize, Token, Tokenize},
	contract::ContractFactory,
	core::{rand::rngs::ThreadRng, utils::Anvil},
	middleware::SignerMiddleware,
	prelude::{ContractInstance, *},
	providers::{Http, Middleware, Provider},
	signers::{LocalWallet, Signer},
	utils::AnvilInstance,
};
use ethers_solc::{
	artifacts::{
		output_selection::OutputSelection, DebuggingSettings, Libraries, Optimizer,
		OptimizerDetails, RevertStrings, Settings, SettingsMetadata, StorageLayout,
	},
	Artifact, ConfigurableContractArtifact, EvmVersion, Project, ProjectCompileOutput,
	ProjectPathsConfig, SolcConfig,
};
use futures::SinkExt;
use hyperspace_ethereum::{
	contract::UnwrapContractError,
	utils::{DeployYuiIbc, FacetCut, FacetCutAction},
};
use ibc::{
	core::{
		ics04_channel::packet::Packet,
		ics24_host::identifier::{ChannelId, PortId},
	},
	timestamp::Timestamp,
	Height,
};
use tracing::log;

pub const USE_GETH: bool = true;

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
pub fn spawn_anvil() -> (AnvilInstance, Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) {
	let anvil = Anvil::new().spawn();
	let wallet: LocalWallet = if USE_GETH {
		LocalWallet::decrypt_keystore(
			"keys/0x73db010c3275eb7a92e5c38770316248f4c644ee",
			option_env!("KEY_PASS").expect("KEY_PASS not set"),
		)
		.unwrap()
		.into()
	} else {
		anvil.keys()[0].clone().into()
	};

	let endpoint = if USE_GETH { "http://localhost:6001".to_string() } else { anvil.endpoint() };
	let provider = Provider::<Http>::try_from(endpoint)
		.unwrap()
		.interval(Duration::from_millis(10u64));
	let chain_id = if USE_GETH { 4242u64 } else { anvil.chain_id() };
	let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
	let client = Arc::new(client);

	(anvil, client)
}

#[track_caller]
pub fn compile_solc(project_paths: ProjectPathsConfig) -> ProjectCompileOutput {
	// custom solc config to solve Yul-relatated compilation errors
	let mut selection = OutputSelection::default_output_selection();
	selection
		.0
		.get_mut("*")
		.unwrap()
		.get_mut("*")
		.unwrap()
		.push("storageLayout".to_string());
	let solc_config = SolcConfig {
		settings: Settings {
			stop_after: None,
			remappings: vec![],
			optimizer: Optimizer {
				enabled: Some(true),
				runs: Some(256),
				details: Some(OptimizerDetails {
					peephole: Some(true),
					inliner: Some(true),
					jumpdest_remover: Some(true),
					order_literals: Some(true),
					deduplicate: Some(true),
					cse: Some(true),
					constant_optimizer: Some(true),
					yul: Some(false),
					yul_details: None,
				}),
			},
			model_checker: None,
			metadata: None,
			output_selection: selection,
			evm_version: Some(EvmVersion::Paris),
			via_ir: Some(false),
			// debug: Some(DebuggingSettings {
			// 	revert_strings: Some(RevertStrings::Debug),
			// 	debug_info: vec!["location".to_string()],
			// }),
			debug: None,
			libraries: Libraries { libs: Default::default() },
		},
	};

	let mut project = Project::builder()
		.paths(project_paths)
		.ephemeral()
		.solc_config(solc_config)
		.build()
		.expect("project build failed");

	// TODO: figure out how to enable it in the config
	// project.artifacts.additional_values.storage_layout = true;
	project.artifacts.additional_files.abi = true;
	// project.solc.args.push("--storage-layout".to_string());

	let project_output = project.compile().expect("compilation failed");

	if project_output.has_compiler_errors() {
		for err in project_output.output().errors {
			eprintln!("{}", err);
		}
		panic!("compiler errors");
	}

	return project_output
}

/// Uses solc to compile the yui-ibc-solidity contracts.
///
/// first argument is the path to the yui-ibc-solidity repo.
/// the second argument is the path to the solidity sources, relative to the first argument.
///
/// so if you have the yui-ibc-solidity as the path to yui then sources should be "contracts/core"
/// for IBCHandler or "contracts/clients" for the clients.
#[track_caller]
pub fn compile_yui(path_to_yui: &Path, sources: &str) -> ProjectCompileOutput {
	assert!(
		path_to_yui.exists(),
		"path to yui-ibc-solidity does not exist: {}",
		path_to_yui.display()
	);

	let project_paths = ProjectPathsConfig::builder()
		.root(&path_to_yui)
		.sources(path_to_yui.join(sources))
		.build()
		.unwrap();

	compile_solc(project_paths)
}

#[allow(dead_code)]
pub async fn deploy_contract<M, T>(
	contract: &ConfigurableContractArtifact,
	constructor_args: T,
	client: Arc<M>,
) -> ContractInstance<Arc<M>, M>
where
	M: Middleware,
	T: abi::Tokenize,
{
	let (abi, bytecode, _) = contract.clone().into_parts();
	let mut factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	factory.deploy(constructor_args).unwrap().send().await.unwrap()
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

fn get_selectors<M>(contract: &ContractInstance<Arc<M>, M>) -> Vec<(String, [u8; 4])>
where
	M: Middleware,
{
	let signatures = contract.abi().functions.keys().cloned().collect::<Vec<_>>();
	signatures
		.into_iter()
		.filter(|val| val != "init(bytes)")
		.map(|val| (val.clone(), contract.abi().function(&val).unwrap().short_signature()))
		.collect()
}

pub async fn deploy_yui_ibc<M>(
	project_output: &ProjectCompileOutput,
	diamond_project_output: &ProjectCompileOutput,
	client: Arc<M>,
) -> DeployYuiIbc<Arc<M>, M>
where
	M: Middleware,
{
	let facet_names = [
		"IBCClient",
		"IBCConnection",
		"IBCChannelHandshake",
		"IBCPacket",
		"IBCQuerier",
		"DiamondCutFacet",
		"DiamondLoupeFacet",
		"OwnershipFacet",
	];

	project_output.artifacts().for_each(|(name, artifact)| {
		let size = artifact.bytecode.as_ref().unwrap().object.as_bytes().unwrap().len();
		let max = 24 * 1024;
		if size > max {
			panic!("{} size is too big: {}/{}", name, size, max);
		}
		log::info!("{} size: {}/{}", name, size, max);
	});
	diamond_project_output.artifacts().for_each(|(name, artifact)| {
		let size = artifact.bytecode.as_ref().unwrap().object.as_bytes().unwrap().len();
		let max = 24 * 1024;
		if size > max {
			panic!("{} size is too big: {}/{}", name, size, max);
		}
		log::info!("{} size: {}/{}", name, size, max);
	});

	let acc = client.default_sender().unwrap();

	println!("Sender account: {acc:?}");

	let contract = diamond_project_output.find_first("DiamondInit").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let diamond_init = factory.deploy(()).unwrap().send().await.unwrap();
	println!("Diamond init address: {:?}", diamond_init.address());

	let mut sigs = HashMap::<[u8; 4], (String, String)>::new();
	let mut facet_cuts = vec![];
	let mut deployed_facets = vec![];
	for facet_name in facet_names {
		let contract = project_output
			.find_first(facet_name)
			.or_else(|| diamond_project_output.find_first(facet_name))
			.unwrap();
		let (abi, bytecode, _) = contract.clone().into_parts();
		let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
		let facet = factory.deploy(()).unwrap().send().await.unwrap();
		let facet_address = facet.address();
		println!("Deployed {facet_name} on {facet_address:?}");
		deployed_facets.push(facet.clone());
		let selectors = get_selectors(&facet);

		for (name, selector) in &selectors {
			if sigs.contains_key(selector) {
				let (contract_name, fn_name) = &sigs[selector];
				panic!(
					"duplicate selector: {}:{} and {}:{}",
					contract_name, fn_name, facet_name, name
				);
			}
			sigs.insert(*selector, (facet_name.to_owned(), name.clone()));
		}

		let facet_cut = FacetCut { address: facet_address, action: FacetCutAction::Add, selectors };
		facet_cuts.push(facet_cut);
	}
	let init_calldata = diamond_init.method::<_, ()>("init", ()).unwrap().calldata().unwrap();

	let contract = diamond_project_output.find_first("Diamond").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let diamond = factory
		.deploy(Token::Tuple(vec![
			Token::Array(facet_cuts.clone().into_iter().map(|x| x.into_token()).collect()),
			Token::Tuple(vec![
				Token::Address(acc),
				Token::Address(diamond_init.address()),
				Token::Bytes(init_calldata.0.into()),
			]),
		]))
		.unwrap()
		.send()
		.await
		.unwrap();

	// std::fs::ReadDir::new().
	let predefined_layout = serde_json::from_reader::<_, StorageLayout>(
		File::open("/Users/vmark/work/centauri-private/hyperspace/ethereum/src/storage_layout/ibc_storage.json").unwrap(),
	)
	.expect("failed to read predefined storage layout");

	let storage_layout = project_output
		.compiled_artifacts()
		.iter()
		.chain(diamond_project_output.compiled_artifacts())
		.flat_map(|(_, artifact)| {
			artifact.into_iter().flat_map(|(an, artifact)| {
				println!("artifact name {an}");
				artifact
			})
		})
		.filter_map(|ar| ar.artifact.storage_layout.clone())
		.chain(once(predefined_layout))
		.fold(StorageLayout { storage: vec![], types: Default::default() }, |mut acc, layout| {
			// let mut len0 = acc.storage.len();
			// let mut len1 = layout.storage.len();
			acc.storage.extend(layout.storage);
			// assert_eq!(acc.storage.len(), len0 + len1, "duplicated storage");

			let len0 = acc.types.len();
			let len1 = layout.types.len();
			acc.types.extend(layout.types);
			assert_eq!(acc.types.len(), len0 + len1, "duplicated type");
			acc
		});

	let tendermint_client = diamond.clone();

	DeployYuiIbc {
		diamond,
		facet_cuts,
		deployed_facets,
		storage_layout,
		tendermint: tendermint_client,
	}
}
