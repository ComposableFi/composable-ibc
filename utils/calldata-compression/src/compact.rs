pub fn encode_compact_u32(value: u32) -> Vec<u8> {
	if value <= (1 << 6) - 1 {
		return vec![(value << 2) as u8]
	} else if value <= (1 << 14) - 1 {
		let byte1 = ((value << 2) & 0xFF) as u8;
		let byte2 = ((value << 2) >> 8) as u8;
		return vec![byte1 | 0b01, byte2]
	} else if value <= (1 << 30) - 1 {
		let byte1 = ((value << 2) & 0xFF) as u8;
		let byte2 = ((value << 2) >> 8) as u8;
		let byte3 = ((value << 2) >> 16) as u8;
		let byte4 = ((value << 2) >> 24) as u8;
		return vec![byte1 | 0b10, byte2, byte3, byte4]
	} else {
		let byte1 = (value & 0xFF) as u8;
		let byte2 = (value >> 8) as u8;
		let byte3 = (value >> 16) as u8;
		let byte4 = (value >> 24) as u8;
		return vec![byte1 | 0b11, byte2, byte3, byte4]
	}
}

pub fn decode_compact_u32(data: &mut &[u8]) -> Option<u32> {
	let prefix = data[0];
	// *data = &data[1..];
	let mut result = 0;
	match prefix % 4 {
		0 => {
			result = u32::from(prefix) >> 2;
			*data = &data[1..];
		},
		1 => {
			let x = u16::from_le_bytes([prefix, data[1]]) >> 2;
			if x > 0b0011_1111 && x <= 0b0011_1111_1111_1111 {
				result = u32::from(x);
				*data = &data[2..];
			} else {
				return None
			}
		},
		2 => {
			let x = u32::from_le_bytes([prefix, data[1], data[2], data[3]]) >> 2;
			if x > 0b0011_1111_1111_1111 && x <= u32::MAX >> 2 {
				result = x;
				*data = &data[4..];
			} else {
				return None
			}
		},
		3 | _ => {
			if prefix >> 2 == 0 {
				// just 4 bytes. ok.
				let x = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
				if x > u32::MAX >> 2 {
					result = x;
					*data = &data[5..];
				} else {
					return None
				}
			} else {
				// Out of range for a 32-bit quantity.
				return None
			}
		},
	}
	Some(result)
}

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
