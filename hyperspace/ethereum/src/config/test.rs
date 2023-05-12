const CONFIG: &'static str = include_str!("test_config.toml");

#[test]
fn parse_config() {
	let config: super::Config = toml::from_str(&CONFIG).expect("parsing error");
	assert_eq!(config.name, "mainnet");
	assert!(config.rpc_url.host().is_some());
	assert_eq!(config.rpc_url.host(), Some("localhost"));
	assert_eq!(config.rpc_url.port_u16(), Some(7890));
	assert_eq!(config.rpc_url.scheme_str(), Some("http"));
	assert!(config.ws_url.host().is_some());
	assert_eq!(config.ws_url.host(), Some("localhost"));
	assert_eq!(config.ws_url.port_u16(), Some(1234));
	assert_eq!(config.ws_url.scheme_str(), Some("ws"));
	assert_eq!(config.mnemonic, "1337");
	assert_eq!(config.channel_whitelist, &[]);
	assert_eq!(config.address, "0xdeadbeef");
}
