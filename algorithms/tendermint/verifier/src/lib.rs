//! This section mainly has been ported from `InformalSystems/hermes/relayer/src/light_client`
use futures_util::future::TryFutureExt;
use ibc::Height as ICSHeight;
use ics07_tendermint::client_state::ClientState as TmClientState;
use pallet_ibc::light_clients::HostFunctionsManager;
use tendermint_light_client::{
	components::{self, io::AsyncIo, io::AtHeight, io::RpcIo},
	light_client::LightClient as TmLightClient,
	state::State as LightClientState,
	store::{memory::MemoryStore, LightStore},
};
use tendermint_light_client_verifier::{
	host_functions::CryptoProvider,
	operations::{ProdCommitValidator, ProdVotingPowerCalculator},
	options::Options as TmOptions,
	predicates::ProdPredicates,
	types::{Height as TMHeight, LightBlock, PeerId, Status},
	PredicateVerifier, ProdVerifier,
};
use tendermint_primitives::error::Error;
use tendermint_rpc::{Client, HttpClient, Url};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verified<H> {
	/// Verified target
	pub target: H,
	/// Supporting headers needed to verify `target`
	pub supporting: Vec<H>,
}

#[derive(Clone, Debug)]
pub struct LightClient {
	peer_id: PeerId,
	io: RpcIo,
}

impl LightClient {
	pub async fn init_light_client(rpc_url: Url) -> Result<Self, Error> {
		let rpc_client = HttpClient::new(rpc_url).map_err(|e| Error::from(e.to_string()))?;
		let peer_id: PeerId = rpc_client
			.status()
			.await
			.map(|s| s.node_info.id)
			.map_err(|e| Error::from(e.to_string()))?;
		let io = RpcIo::new(peer_id, rpc_client, None);
		Ok(Self { peer_id, io })
	}

	pub async fn prepare_tendermint_light_client<HostFunctions>(
		&self,
		client_state: &TmClientState<HostFunctions>,
	) -> Result<TmLightClient<HostFunctions>, Error>
	where
		HostFunctions: CryptoProvider,
	{
		let clock = components::clock::SystemClock;
		let verifier: PredicateVerifier<
			ProdPredicates<HostFunctions>,
			ProdVotingPowerCalculator<HostFunctions>,
			ProdCommitValidator<HostFunctions>,
			HostFunctions,
		> = ProdVerifier::default();
		let scheduler = components::scheduler::basic_bisecting_schedule;
		let params = TmOptions {
			trust_threshold: client_state.trust_level.try_into().unwrap(),
			trusting_period: client_state.trusting_period,
			clock_drift: client_state.max_clock_drift,
		};

		Ok(TmLightClient::new(self.peer_id, params, clock, scheduler, verifier, self.io.clone()))
	}

	pub async fn prepare_state(&self, trusted: ICSHeight) -> Result<LightClientState, Error> {
		let trusted_height =
			TMHeight::try_from(trusted.revision_height).map_err(|e| Error::from(e.to_string()))?;

		let trusted_block = self.fetch_light_block(AtHeight::At(trusted_height)).await?;
		let mut store = MemoryStore::new();
		store.insert(trusted_block, Status::Trusted);

		Ok(LightClientState::new(store))
	}

	async fn fetch_light_block(&self, height: AtHeight) -> Result<LightBlock, Error> {
		self.io.fetch_light_block(height).map_err(|e| Error::from(e.to_string())).await
	}

	/// Perform forward verification with bisection.
	pub async fn verify<HostFuntions>(
		&self,
		trusted: ICSHeight,
		target: ICSHeight,
		client_state: &TmClientState<HostFunctionsManager>,
	) -> Result<LightBlock, Error> {
		let target_height =
			TMHeight::try_from(target.revision_height).map_err(|e| Error::from(e.to_string()))?;

		let client = self
			.prepare_tendermint_light_client::<HostFunctionsManager>(client_state)
			.await?;
		let mut state = self.prepare_state(trusted).await?;

		// Verify the target header
		let target = client
			.verify_to_target(target_height, &mut state)
			.map_err(|e| Error::from(e.to_string()))
			.await?;
		Ok(target)
	}
}
