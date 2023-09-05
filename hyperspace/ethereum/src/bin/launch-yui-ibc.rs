#[path = "../../tests/utils.rs"]
mod utils;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path, "contracts/core");
	let diamond_project_output = utils::compile_yui(&path, "contracts/diamond");
	let (anvil, client) = utils::spawn_anvil();

	println!("address: {:?}", anvil.endpoint());
	println!("chain-id: {:?}", anvil.chain_id());

	let yui_ibc = utils::deploy_yui_ibc(&project_output, &diamond_project_output, client).await;

	println!("diamond address: {:?}", yui_ibc.diamond.address());

	let _ = tokio::signal::ctrl_c().await;
	drop(anvil);
}
