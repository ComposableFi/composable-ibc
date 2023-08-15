
#[path = "../../tests/utils.rs"]
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let path = utils::yui_ibc_solidity_path();
    let project_output = utils::compile_yui(&path, "contracts/core");
    let (anvil, client) = utils::spawn_anvil();

    println!("address: {:?}", anvil.endpoint());
    println!("chain-id: {:?}", anvil.chain_id());

    let utils::DeployYuiIbc {
        ibc_client,
        ibc_connection,
        ibc_channel_handshake,
        ibc_packet,
        ibc_handler,
    } = utils::deploy_yui_ibc(&project_output, client).await;

    println!("ibc_client: {:?}", ibc_client.address());
    println!("ibc_connection: {:?}", ibc_connection.address());
    println!("ibc_channel_handshake: {:?}", ibc_channel_handshake.address());
    println!("ibc_packet: {:?}", ibc_packet.address());
    println!("ibc_handler: {:?}", ibc_handler.address());

    let _ = tokio::signal::ctrl_c().await;
    drop(anvil);
}