use std::io::Result;

fn main() -> Result<()> {
	// build protoc compiler
	std::env::set_var("PROTOC", protobuf_src::protoc());
	// compile our proto files
	prost_build::compile_protos(&["src/proto/beefy.proto"], &["src/"])?;
	Ok(())
}
