#[allow(unused)]
pub fn compress_rle(data: &[u8]) -> Vec<u8> {
	let mut compressed = Vec::with_capacity(data.len() * 2); // max size

	let mut i = 0;
	while i < data.len() {
		let current = data[i];
		let mut count = 1;

		while i + count < data.len() && count < 255 && data[i + count] == current {
			count += 1;
		}

		compressed.push(count as u8);
		compressed.push(current);

		i += count;
	}

	println!("RLE compressed: {} -> {}", data.len(), compressed.len());
	compressed.shrink_to_fit();
	compressed
}

#[allow(unused)]
pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
	assert!(data.len() % 2 == 0, "Invalid compressed data");

	let mut decompressed = Vec::with_capacity(data.len() * 255); // max size

	let mut i = 0;
	while i < data.len() {
		let count = data[i] as usize;
		let value = data[i + 1];

		for _ in 0..count {
			decompressed.push(value);
		}

		i += 2;
	}

	decompressed.shrink_to_fit();
	decompressed
}

#[test]
fn foo() {
	let n = 1000u16;
	// generate a byte array with `n` u16 values. The first u16 value should be the number of the
	// elements (n)
	let mut data = vec![0; n as usize * 2 + 2];
	data[0..2].copy_from_slice(&n.to_be_bytes());
	for i in 0..n {
		data[i as usize * 2 + 2..i as usize * 2 + 4].copy_from_slice(&i.to_be_bytes());
	}
	println!("Data: 0x{}", hex::encode(&data));
}

#[test]
fn test_decode() {
	let nums = (0..20)
		.flat_map(|x| {
			let n = 1 << x;
			vec![n, n - 1, n + 1]
		})
		.collect::<Vec<_>>();
	let mut data = vec![];
	for i in &nums {
		let encoded = encode_compact_u32(*i);
		data.extend(&encoded);
		assert_eq!(*i, decode_compact_u32(&mut &encoded[..]).unwrap());
	}
	for i in nums.iter().rev() {
		let encoded = encode_compact_u32(*i);
		data.extend(&encoded);
	}
	let data_ref = &mut &data[..];
	for i in nums.iter().chain(nums.iter().rev()) {
		let n = decode_compact_u32(data_ref).unwrap();
		assert_eq!(*i, n);
	}
}
