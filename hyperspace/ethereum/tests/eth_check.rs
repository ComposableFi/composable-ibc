use ethers::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let rpc_url = "http://localhost:7545";
	let provider = Provider::try_from(rpc_url)?;

	let chain_id = provider.get_chainid().await?;
	let block_number = provider.get_block_number().await?;
	let tx_pool_content = provider.txpool_content().await?;

	Ok(())
}

#[tokio::test]
async fn test_eth_environment_is_active() {
	main().await.expect("failure");
}
