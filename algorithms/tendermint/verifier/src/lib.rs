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
}
