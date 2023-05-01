const CONFIG: &'static str = include_str!("test_config.toml");

#[test]
fn parse_config() {
	let super::Config { rpc_url, .. } = toml::from_str(&CONFIG).expect("parsing error");
	assert!(rpc_url.host().is_some());
	assert_eq!(rpc_url.host(), Some("localhost"));
	assert_eq!(rpc_url.port_u16(), Some(7890));
	assert_eq!(rpc_url.scheme_str(), Some("http"))
}
