fn main() -> std::io::Result<()> {
	prost_build::Config::new()
		// .enable_type_names()
		// .type_name_domain(["."], "")
		.include_file("messages.rs")
		.compile_protos(&["proto/guest.proto"], &["proto/"])
}
