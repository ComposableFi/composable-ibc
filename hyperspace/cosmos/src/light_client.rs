//! This section mainly has been ported from `InformalSystems/hermes/relayer/src/light_client`
use crate::error::Error;
use ibc::Height;
use ics07_tendermint::{client_state::ClientState, ProdVerifier};
use pallet_ibc::light_clients::HostFunctionsManager;
use tendermint::trust_threshold::TrustThresholdFraction;
use tendermint_light_client::{
	components::{
		self,
		io::{AtHeight, ProdIo},
	},
	light_client::LightClient as TmLightClient,
	state::State as LightClientState,
	store::{memory::MemoryStore, LightStore},
};
use tendermint_light_client_verifier::{
	options::Options as TmOptions,
	types::{Height as TMHeight, LightBlock, PeerId, Status},
	PredicateVerifier,
};
use tendermint_rpc::{Client, HttpClient, Url};

#[derive(Clone, Debug)]
pub struct LightClient {
	pub peer_id: PeerId,
	pub io: ProdIo,
}

impl LightClient {
	pub async fn init_light_client(rpc_url: Url) -> Result<Self, Error> {
		let rpc_client = HttpClient::new(rpc_url).map_err(|e| Error::from(e.to_string()))?;
		let peer_id: PeerId = rpc_client
			.status()
			.await
			.map(|s| s.node_info.id)
			.map_err(|e| Error::from(e.to_string()))?;
		let io = ProdIo::new(peer_id, rpc_client, None);
		Ok(Self { peer_id, io })
	}

	pub fn prepare_tendermint_light_client(
		&self,
		client_state: &ClientState<HostFunctionsManager>,
	) -> Result<TmLightClient, Error> {
		let params = TmOptions {
			trust_threshold: TrustThresholdFraction::new(
				client_state.trust_level.numerator(),
				client_state.trust_level.denominator(),
			)?,
			trusting_period: client_state.trusting_period,
			clock_drift: client_state.max_clock_drift,
		};
		let clock = components::clock::SystemClock;
		let scheduler = components::scheduler::basic_bisecting_schedule;
		let verifier: ProdVerifier<HostFunctionsManager> = PredicateVerifier::default();

		Ok(TmLightClient::new(self.peer_id, params, clock, scheduler, verifier, self.io.clone()))
	}

	pub fn prepare_state(&self, trusted: Height) -> Result<LightClientState, Error> {
		let trusted_height =
			TMHeight::try_from(trusted.revision_height).map_err(|e| Error::from(e.to_string()))?;

		use tendermint_light_client::components::io::Io;
		let trusted_block = self
			.io
			.fetch_light_block(AtHeight::At(trusted_height))
			.map_err(|e| Error::from(e.to_string()))?;
		let mut store = MemoryStore::new();
		store.insert(trusted_block, Status::Trusted);
		Ok(LightClientState::new(store))
	}

	/// Perform forward verification with bisection.
	pub async fn verify(
		&self,
		trusted: Height,
		target: Height,
		client_state: &ClientState<HostFunctionsManager>,
	) -> Result<LightBlock, Error> {
		let target_height =
			TMHeight::try_from(target.revision_height).map_err(|e| Error::from(e.to_string()))?;

		let client = self.prepare_tendermint_light_client(client_state)?;
		let mut state = self.prepare_state(trusted)?;

		// Verify the target header
		let target = client
			.verify_to_target(target_height, &mut state)
			.map_err(|e| Error::from(e.to_string()))?;
		Ok(target)
	}
}
