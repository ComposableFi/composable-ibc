/// Errors the prover may encounter.
#[derive(derive_more::From)]
pub enum Error {
	/// subxt error
	Subxt(subxt::BasicError),
	/// subxt rpc error
	SubxtRRpc(subxt::rpc::RpcError),
	/// Codec error
	Codec(codec::Error),
}
