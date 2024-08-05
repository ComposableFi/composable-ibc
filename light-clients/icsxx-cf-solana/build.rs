fn main() -> std::io::Result<()> {
	prost_build::Config::new()
		.enable_type_names()
		.include_file("messages.rs")
		.compile_protos(&["src/proto/cf-solana.proto"], &["src/proto/"])
}
