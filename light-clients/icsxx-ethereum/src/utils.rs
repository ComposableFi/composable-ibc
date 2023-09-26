use tiny_keccak::Hasher;

pub fn keccak256<T: AsRef<[u8]>>(x: T) -> [u8; 32] {
	let mut hasher = tiny_keccak::Keccak::v256();
	hasher.update(x.as_ref());
	let mut output = [0u8; 32];
	hasher.finalize(&mut output);
	output
}
