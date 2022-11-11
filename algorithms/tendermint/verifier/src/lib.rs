//! This section mainly has been ported from `InformalSystems/hermes/relayer/src/light_client`
use tendermint_light_client::components::{self, io::AsyncIo, io::AtHeight, io::RpcIo};
use tendermint_light_client_verifier::types::{LightBlock, PeerId, Status};
use tendermint_primitives::error::Error;
use tendermint_rpc::{Client, HttpClient, Url};

#[derive(Clone, Debug)]
pub struct LightClient {
	peer_id: PeerId,
	rpc_handler: RpcIo,
}

impl LightClient {
	pub async fn init_light_client(rpc_url: Url) -> Result<Self, Error> {
		let rpc_client = HttpClient::new(rpc_url).map_err(|e| Error::from(e.to_string()))?;
		let peer_id: PeerId = rpc_client
			.status()
			.await
			.map(|s| s.node_info.id)
			.map_err(|e| Error::from(e.to_string()))?;
		let rpc_handler = RpcIo::new(peer_id, rpc_client, None);
		Ok(Self { peer_id, rpc_handler })
	}
}
