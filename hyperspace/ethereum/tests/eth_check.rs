use ethers::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let rpc_url = "http://localhost:7545";
	let provider = Provider::try_from(rpc_url)?;

	let _chain_id = provider.get_chainid().await?;
	let _block_number = provider.get_block_number().await?;
	let _tx_pool_content = provider.txpool_content().await?;

	Ok(())
}

#[tokio::test]
#[ignore = "interactive test that requires an ethereum node to be running locally"]
async fn test_eth_environment_is_active() {
	main().await.expect("failure");
}
