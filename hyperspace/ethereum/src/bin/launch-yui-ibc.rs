use hyperspace_ethereum::mock::utils;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let path = utils::yui_ibc_solidity_path();
	let project_output = hyperspace_ethereum::utils::compile_yui(&path, "contracts/core");
	let diamond_project_output =
		hyperspace_ethereum::utils::compile_yui(&path, "contracts/diamond");
	let (anvil, client) = utils::spawn_anvil().await;

	println!("address: {:?}", anvil.endpoint());
	println!("chain-id: {:?}", anvil.chain_id());

	let yui_ibc = hyperspace_ethereum::utils::deploy_yui_ibc(
		&project_output,
		&diamond_project_output,
		client,
	)
	.await;

	println!("diamond address: {:?}", yui_ibc.ibc_core_diamond.address());

	let _ = tokio::signal::ctrl_c().await;
	drop(anvil);
}
