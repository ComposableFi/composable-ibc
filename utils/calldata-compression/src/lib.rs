use std::{env, fs::File, io::Read, process::Command};

/// compress calldata stores in a file (calldata_path) and returns
/// the hex representation of it, as a String
pub fn compress(script_path: String, calldata_path: String) -> String {
	// due to memory constraints, need to increase limits for NodeJS
	env::set_var("NODE_OPTIONS", "--max_old_space_size=409600");

	let _ = Command::new("node").arg(script_path).arg(calldata_path).output().unwrap();

	// we are storing the result on a file. This is because for now, I'm not being able to
	// decode the hex in Rust in the same way it is stored in the file. Once we fix this,
	// it will be possible to consume the data directly from the stdout
	let mut file = File::open("test.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	return contents
}

#[test]
fn test() {
	let result = compress(
		"./utils/calldata-compression/scripts/decompress-script.js".to_string(),
		"./utils/calldata-compression/data/tx-calldata.json".to_string(),
	);

	assert_eq!(&result[..13], "0x60200061800")
}
