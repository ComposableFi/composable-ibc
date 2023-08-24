use std::{sync::Arc, path::{Path, PathBuf}};

use ethers::{
	abi::{Abi, Address, Detokenize, Token},
	prelude::Contract,
	providers::Middleware,
};
use ethers_solc::{ProjectCompileOutput, ProjectPathsConfig, SolcConfig, artifacts::{Settings, Optimizer, OptimizerDetails, output_selection::OutputSelection, DebuggingSettings, RevertStrings, Libraries}, EvmVersion, Project, Artifact};

/// Unwraps a contract error, decoding the revert reason if possible
pub trait UnwrapContractError<T> {
	fn unwrap_contract_error(self) -> T;
}

impl<T, M> UnwrapContractError<T> for Result<T, ethers::prelude::ContractError<M>>
where
	M: Middleware,
{
	/// Unwraps a contract error, decoding the revert reason if possible
	#[track_caller]
	fn unwrap_contract_error(self) -> T {
		match self {
			Ok(t) => t,
			Err(ethers::prelude::ContractError::Revert(bytes)) => {
				// abi decode the bytes after the first 4 bytes (the error selector)
				if bytes.len() < 4 {
					panic!("contract-error: {:?}", bytes);
				}
				let bytes = &bytes[4..];
				let tokens = ethers::abi::decode(&[ethers::abi::ParamType::String], bytes).unwrap();
				panic!("contract-error: {tokens:#?}")
			},
			Err(e) => panic!("contract-error: {:?}", e),
		}
	}
}

pub const IBC_HANDLER_ABI: &str = include_str!("./abi/ibc-handler-abi.json");

/// A wrapper around the IBC handler contract instance
pub struct IbcHandler<M> {
	pub(crate) contract: Contract<M>,
}

impl<M> IbcHandler<M>
where
	M: Middleware,
{	
	pub fn new(contract: Contract<M>) -> Self {
		IbcHandler { contract }
	}

	pub async fn bind_port(&self, port_id: &str, address: Address) {
		let bind_port = self
			.contract
			.method::<_, ()>("bindPort", (Token::String(port_id.into()), Token::Address(address)))
			.unwrap();
		let () = bind_port.call().await.unwrap_contract_error();
		let tx_recp = bind_port.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn connection_open_init(&self, msg: Token) -> String {
		let method = self.contract.method::<_, String>("connectionOpenInit", (msg,)).unwrap();

		let gas_estimate_connection_id = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_id);
		let connection_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		connection_id
	}

	pub async fn connection_open_ack(&self, msg: Token) {
		let method = self.contract.method::<_, ()>("connectionOpenAck", (msg,)).unwrap();

		let gas_estimate_connection_open = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open);
		let _ = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn connection_open_try(&self, msg: Token) -> String {
		let method = self.contract.method::<_, String>("connectionOpenTry", (msg,)).unwrap();

		let gas_estimate_connection_open_try = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open_try);
		let id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		id
	}


	pub async fn register_client(&self, kind: &str, address: Address) {
		let method = self
			.contract
			.method::<_, ()>(
				"registerClient",
				(Token::String(kind.into()), Token::Address(address)),
			)
			.unwrap();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn create_client(&self, msg: Token) -> String {
		let method = self.contract.method::<_, String>("createClient", (msg,)).unwrap();

		let client_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));

		client_id
	}

	pub async fn update_client(&self, msg: Token) {
		let method = self.contract.method::<_, ()>("updateClient", (msg,)).unwrap();

		let gas_estimate_update_client = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_update_client);
		let client_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}
}

#[track_caller]
pub fn get_contract_from_name<M>(address: Address, client: Arc<M>, source: impl AsRef<str>, s: impl AsRef<str>) -> Contract<M>
where
	M: Middleware,
{
	//todo: use the artifact instead of compiling!!!
	//but for now it is more dynamic to compile then to use the artifact
	let path = yui_ibc_solidity_path();
	let project = compile_yui(&path, source.as_ref());
	let contract = project.find_first(s).unwrap();
	let r = contract.clone();
	let (abi, _, _) = r.into_parts();
	Contract::new(address, abi.unwrap(), client)
}

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

#[track_caller]
pub fn compile_solc(project_paths: ProjectPathsConfig) -> ProjectCompileOutput {
	// custom solc config to solve Yul-relatated compilation errors
	let solc_config = SolcConfig {
		settings: Settings {
			stop_after: None,
			remappings: vec![],
			optimizer: Optimizer {
				enabled: Some(false),
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
			output_selection: OutputSelection::default_output_selection(),
			evm_version: Some(EvmVersion::Paris),
			via_ir: Some(false),
			debug: Some(DebuggingSettings {
				revert_strings: Some(RevertStrings::Debug),
				debug_info: vec!["location".to_string()],
			}),
			libraries: Libraries { libs: Default::default() },
		},
	};

	let project = Project::builder()
		.paths(project_paths)
		.ephemeral()
		.no_artifacts()
		.solc_config(solc_config)
		.build()
		.expect("project build failed");

	let project_output = project.compile().expect("compilation failed");

	if project_output.has_compiler_errors() {
		for err in project_output.output().errors {
			eprintln!("{}", err);
		}
		panic!("compiler errors");
	}

	return project_output
}

/// Create a new contract instance from the given address and ABI.
#[track_caller]
pub fn ibc_handler<M>(address: Address, client: Arc<M>) -> Contract<M>
where
	M: Middleware,
{
	let abi: Abi = serde_json::from_str(IBC_HANDLER_ABI).unwrap();
	Contract::new(address, abi, client)
}

pub(crate) struct Counterparty {
	pub(crate) client_id: String,
	pub(crate) connection_id: String,
	pub(crate) prefix: Vec<u8>,
}

pub(crate) struct Version {
	pub(crate) identifier: String,
	pub(crate) features: Vec<String>,
}

pub struct ChannelEnd {
	pub(crate) state: u32,
	pub(crate) ordering: u32,
	pub(crate) counterparty: Counterparty,
	pub(crate) connection_hops: Vec<String>,
	pub(crate) version: String,
}

impl Detokenize for ChannelEnd {
	fn from_tokens(tokens: Vec<ethers::abi::Token>) -> Result<Self, ethers::abi::InvalidOutputType>
	where
		Self: Sized,
	{
		let vec = tokens[3].clone().into_array().unwrap();

		let connection_hops = {
			let mut it = vec.into_iter();
			let mut v = vec![];

			while let Some(connection_id) = it.next() {
				v.push(connection_id.into_string().unwrap())
			}

			v
		};

		Ok(ChannelEnd {
			state: tokens[0].clone().into_uint().unwrap().as_u32(),
			ordering: tokens[1].clone().into_uint().unwrap().as_u32(),
			counterparty: Counterparty {
				client_id: tokens[2].clone().into_string().unwrap(),
				connection_id: tokens[3].clone().into_string().unwrap(),
				prefix: tokens[4].clone().into_bytes().unwrap(),
			},
			connection_hops,
			version: tokens[5].clone().into_string().unwrap(),
		})
	}
}

pub struct ConnectionEnd {
	pub(crate) client_id: String,
	pub(crate) versions: Vec<Version>,
	pub(crate) state: u32,
	pub(crate) counterparty: Counterparty,
	pub(crate) delay_period: u64,
}

impl Detokenize for ConnectionEnd {
	fn from_tokens(tokens: Vec<ethers::abi::Token>) -> Result<Self, ethers::abi::InvalidOutputType>
	where
		Self: Sized,
	{
		let vec = tokens[1].clone().into_array().unwrap();

		let versions = {
			let mut it = vec.into_iter();
			let mut v = vec![];

			while let (Some(identifier), Some(features)) = (it.next(), it.next()) {
				v.push(Version {
					identifier: identifier.into_string().unwrap(),
					features: features
						.into_array()
						.unwrap()
						.into_iter()
						.map(|t| t.into_string().unwrap())
						.collect(),
				})
			}

			v
		};

		Ok(ConnectionEnd {
			client_id: tokens[0].clone().into_string().unwrap(),
			versions,
			state: tokens[2].clone().into_uint().unwrap().as_u32(),
			counterparty: Counterparty {
				client_id: tokens[3].clone().into_string().unwrap(),
				connection_id: tokens[4].clone().into_string().unwrap(),
				prefix: tokens[5].clone().into_bytes().unwrap(),
			},
			delay_period: tokens[4].clone().into_uint().unwrap().as_u64(),
		})
	}
}

pub const LIGHT_CLIENT_ABI_JSON: &str = include_str!("./abi/light-client-abi.json");

/// Create a new contract instance from the given address and ABI.
#[track_caller]
pub fn light_client_contract<M>(address: Address, client: Arc<M>) -> Contract<M>
where
	M: Middleware,
{
	let abi: Abi = serde_json::from_str(LIGHT_CLIENT_ABI_JSON).unwrap();
	Contract::new(address, abi, client)
}
