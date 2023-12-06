pub fn compress(data: Vec<u8>, chunk_size: usize) -> Vec<u8> {
	assert_eq!(data.len() % chunk_size, 0);
	let mut datas: Vec<Vec<u8>> = vec![vec![]];
	let mut indices = vec![];

	let mut j = 0;
	for chunk in data.chunks_exact(chunk_size) {
		if chunk.iter().all(|x| *x == 0) {
			datas.push(vec![]);
			indices.push(j);
		} else {
			datas.last_mut().unwrap().extend(chunk.to_owned());
			j += 1;
		}
	}

	let out_data = datas.clone().into_iter().flatten().collect::<Vec<_>>();
	let len = indices.len() as u32;

	let indices_encoded = indices
		.into_iter()
		.rev()
		.map(|x| (x as u16).to_be_bytes())
		.flatten()
		.collect::<Vec<_>>();

	[(len as u16).to_be_bytes().to_vec(), indices_encoded, out_data]
		.into_iter()
		.flatten()
		.collect()
}

pub fn decompress(compressed: &mut &[u8], chunk_size: usize) -> Vec<u8> {
	let input = compressed;

	// Decode the number of indices
	let indices_count = u16::from_be_bytes([input[0], input[1]]) as usize;
	*input = &input[2..];
	// Decode the indices
	let mut indices: Vec<usize> = Vec::with_capacity(indices_count);
	for _ in 0..indices_count {
		let index = u16::from_be_bytes([input[0], input[1]]) as usize;
		*input = &input[2..];
		indices.push(index);
	}

	// Collect the remaining compressed data
	let mut data: Vec<u8> = input.to_vec();

	// Reconstruct the original data
	let mut decompressed = vec![0; data.len() + indices_count * chunk_size];

	let mut idx = decompressed.len();
	for i in 0..indices_count {
		let index = indices[i] * chunk_size;
		let non_zero = data.drain(index..).collect::<Vec<_>>();
		idx -= non_zero.len();
		(&mut decompressed[idx..idx + non_zero.len()]).copy_from_slice(&non_zero);
		idx -= chunk_size;
	}

	decompressed
}

mod tests {
	use super::*;

	#[test]
	fn compression() {
		const CHUNK_SIZE: usize = 16;
		let data = hex::decode(include_str!("../data/tx-calldata.txt")).unwrap();
		let vec = decompress(&mut &data[..], CHUNK_SIZE);
		println!("{}", data.len());
	}

	// TODO: add fuzzy-tests
}
