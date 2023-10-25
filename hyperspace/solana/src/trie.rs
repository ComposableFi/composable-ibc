use core::mem::ManuallyDrop;

#[cfg(test)]
use anchor_client::anchor_lang::solana_program;
use lib::hash::CryptoHash;
use memory::Ptr;

/// Discriminants for the data stored in the accounts.
mod magic {
	pub(crate) const UNINITIALISED: u32 = 0;
	pub(crate) const TRIE_ROOT: u32 = 1;
}

const SZ: usize = sealable_trie::nodes::RawNode::SIZE;

/// Trie stored in a Solana account.
#[derive(Debug)]
pub struct AccountTrie<D: DataRef + Sized>(ManuallyDrop<sealable_trie::Trie<Allocator<D>>>);

/// Access to the account data underlying the trie.
pub trait DataRef {
	/// Returns size of the referenced data in bytes.
	fn len(&self) -> usize;

	/// Returns a shared reference to a byte or subslice depending on the type
	/// of index.
	///
	/// Returns `None` if index is out of bounds.
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output>;

	/// Returns a shared reference to a byte or subslice depending on the type
	/// of index.
	///
	/// Returns `None` if index is out of bounds.
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output>;
}

impl<D: DataRef + Sized> AccountTrie<D> {
	/// Creates a new Trie from data in an account.
	///
	/// If the data in the account isn’t initialised (i.e. has zero
	/// discriminant) initialises a new empty trie.
	pub(crate) fn new(data: D) -> Option<Self> {
		let (alloc, root) = Allocator::new(data)?;
		let trie = sealable_trie::Trie::from_parts(alloc, root.0, root.1);
		Some(Self(ManuallyDrop::new(trie)))
	}
}

impl<D: DataRef + Sized> core::ops::Drop for AccountTrie<D> {
	/// Updates the header in the Solana account.
	fn drop(&mut self) {
		// SAFETY: Once we’re done with self.0 we are dropped and no one else is
		// going to have access to self.0.
		let trie = unsafe { ManuallyDrop::take(&mut self.0) };
		let (mut alloc, root_ptr, root_hash) = trie.into_parts();
		let hdr = Header {
			root_ptr,
			root_hash,
			next_block: alloc.next_block,
			first_free: alloc.first_free.map_or(0, |ptr| ptr.get()),
		}
		.encode();
		// Avoid writing data if we’re not changing anything.
		if alloc.data.get(..hdr.len()) != Some(&hdr[..]) {
			alloc.data.get_mut(..hdr.len()).unwrap().copy_from_slice(&hdr);
		}
	}
}

/// Data stored in the first 72-bytes of the account describing the trie.
#[derive(Clone, Debug, PartialEq)]
struct Header {
	root_ptr: Option<Ptr>,
	root_hash: CryptoHash,
	next_block: u32,
	first_free: u32,
}

impl Header {
	/// Size of the encoded header.
	const ENCODED_SIZE: usize = 64;

	/// Decodes the header from given block of memory.
	///
	/// Returns `None` if the block is shorter than length of encoded header or
	/// encoded data is invalid.
	// Encoding:
	//     magic:       u32
	//     version:     u32
	//     root_ptr:    u32
	//     root_hash:   [u8; 32]
	//     next_block:  u32
	//     first_free:  u32
	//     padding:     [u8; 12],
	fn decode(data: &impl DataRef) -> Option<Self> {
		let data = data.get(..Self::ENCODED_SIZE)?.try_into().unwrap();

		// Check magic number.  Zero means the account hasn’t been initialised
		// so return default value, and anything other than magic::TRIE_ROOT
		// means it’s an account storing data different than a trie root.
		let (magic, data) = read::<4, 60, 64, _>(data, u32::from_ne_bytes);
		if magic == magic::UNINITIALISED {
			return Some(Self {
				root_ptr: None,
				root_hash: sealable_trie::trie::EMPTY_TRIE_ROOT,
				next_block: Self::ENCODED_SIZE as u32,
				first_free: 0,
			})
		} else if magic != magic::TRIE_ROOT {
			return None
		}

		// Check version.  This is for future-proofing in case format of the
		// encoding changes.
		let (version, data) = read::<4, 56, 60, _>(data, u32::from_ne_bytes);
		if version != 0 {
			return None
		}

		let (root_ptr, data) = read::<4, 52, 56, _>(data, u32::from_ne_bytes);
		let (root_hash, data) = read::<32, 20, 52, _>(data, CryptoHash);
		let (next_block, data) = read::<4, 16, 20, _>(data, u32::from_ne_bytes);
		let (first_free, _) = read::<4, 12, 16, _>(data, u32::from_ne_bytes);

		let root_ptr = Ptr::new(root_ptr).ok()?;
		Some(Self { root_ptr, root_hash, next_block, first_free })
	}

	/// Returns encoded representation of values in the header.
	fn encode(&self) -> [u8; Self::ENCODED_SIZE] {
		let root_ptr = self.root_ptr.map_or([0; 4], |ptr| ptr.get().to_ne_bytes());

		let mut buf = [0; Self::ENCODED_SIZE];
		let data = &mut buf;
		let data = write::<4, 60, 64>(data, magic::TRIE_ROOT.to_ne_bytes());
		let data = write::<4, 56, 60>(data, [0; 4]);
		let data = write::<4, 52, 56>(data, root_ptr);
		let data = write::<32, 20, 52>(data, self.root_hash.0);
		let data = write::<4, 16, 20>(data, self.next_block.to_ne_bytes());
		write::<4, 12, 16>(data, self.first_free.to_ne_bytes());
		buf
	}
}

#[derive(Debug)]
pub struct Allocator<D> {
	/// Pool of memory to allocate blocks in.
	///
	/// The data is always at least long enough to fit encoded [`Header`].
	data: D,

	/// Position of the next unallocated block.
	///
	/// Blocks which were allocated and then freed don’t count as ‘unallocated’
	/// in this context.  This is position of the next block to return if the
	/// free list is empty.
	next_block: u32,

	/// Pointer to the first freed block; `None` if there were no freed blocks
	/// yet.
	first_free: Option<Ptr>,
}

impl<D: DataRef> Allocator<D> {
	/// Initialises the allocator with data in given account.
	fn new(data: D) -> Option<(Self, (Option<Ptr>, CryptoHash))> {
		let hdr = Header::decode(&data)?;
		let next_block = hdr.next_block;
		let first_free = Ptr::new(hdr.first_free).ok()?;
		let alloc = Self { data, next_block, first_free };
		let root = (hdr.root_ptr, hdr.root_hash);
		Some((alloc, root))
	}

	/// Grabs a block from a free list.  Returns `None` if free list is empty.
	fn alloc_from_freelist(&mut self) -> Option<Ptr> {
		let ptr = self.first_free.take()?;
		let idx = ptr.get() as usize;
		let next = self.data.get(idx..idx + 4).unwrap().try_into().unwrap();
		self.first_free = Ptr::new(u32::from_ne_bytes(next)).unwrap();
		Some(ptr)
	}

	/// Grabs a next available block.  Returns `None` if account run out of
	/// space.
	fn alloc_next_block(&mut self) -> Option<Ptr> {
		let len = u32::try_from(self.data.len()).unwrap_or(u32::MAX);
		let end = self.next_block.checked_add(SZ as u32).filter(|&e| e <= len)?;
		let ptr = Ptr::new(self.next_block).ok().flatten()?;
		self.next_block = end;
		Some(ptr)
	}
}

impl<D: DataRef + Sized> memory::Allocator for Allocator<D> {
	type Value = [u8; SZ];

	fn alloc(&mut self, value: Self::Value) -> Result<Ptr, memory::OutOfMemory> {
		let ptr = self
			.alloc_from_freelist()
			.or_else(|| self.alloc_next_block())
			.ok_or(memory::OutOfMemory)?;
		self.set(ptr, value);
		Ok(ptr)
	}

	#[inline]
	fn get(&self, ptr: Ptr) -> &Self::Value {
		let idx = ptr.get() as usize;
		self.data.get(idx..idx + SZ).unwrap().try_into().unwrap()
	}

	#[inline]
	fn get_mut(&mut self, ptr: Ptr) -> &mut Self::Value {
		let idx = ptr.get() as usize;
		self.data.get_mut(idx..idx + SZ).unwrap().try_into().unwrap()
	}

	#[inline]
	fn free(&mut self, ptr: Ptr) {
		let next = self.first_free.map_or([0; 4], |ptr| ptr.get().to_ne_bytes());
		let idx = ptr.get() as usize;
		self.data.get_mut(idx..idx + 4).unwrap().copy_from_slice(&next);
		self.first_free = Some(ptr);
	}
}

impl<D: DataRef> core::ops::Deref for AccountTrie<D> {
	type Target = sealable_trie::Trie<Allocator<D>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<D: DataRef> core::ops::DerefMut for AccountTrie<D> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl DataRef for [u8] {
	#[inline]
	fn len(&self) -> usize {
		(*self).len()
	}

	#[inline]
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
		self.get(index)
	}

	#[inline]
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output> {
		self.get_mut(index)
	}
}

impl<const N: usize> DataRef for [u8; N] {
	#[inline]
	fn len(&self) -> usize {
		N
	}

	#[inline]
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
		self[..].get(index)
	}

	#[inline]
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output> {
		self[..].get_mut(index)
	}
}

impl DataRef for Vec<u8> {
	#[inline]
	fn len(&self) -> usize {
		(**self).len()
	}

	#[inline]
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
		(**self).get(index)
	}

	#[inline]
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output> {
		(**self).get_mut(index)
	}
}

impl<D: DataRef + ?Sized> DataRef for &'_ mut D {
	#[inline]
	fn len(&self) -> usize {
		(**self).len()
	}

	#[inline]
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
		(**self).get(index)
	}

	#[inline]
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output> {
		(**self).get_mut(index)
	}
}

impl<D: DataRef + ?Sized> DataRef for core::cell::RefMut<'_, D> {
	#[inline]
	fn len(&self) -> usize {
		(**self).len()
	}

	#[inline]
	fn get<I: core::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
		(**self).get(index)
	}

	#[inline]
	fn get_mut<I: core::slice::SliceIndex<[u8]>>(&mut self, index: I) -> Option<&mut I::Output> {
		(**self).get_mut(index)
	}
}

/// Reads fixed-width value from start of the buffer and returns the value and
/// remaining portion of the buffer.
///
/// By working on a fixed-size buffers, this avoids any run-time checks.  Sizes
/// are verified at compile-time.
fn read<const L: usize, const R: usize, const N: usize, T>(
	buf: &[u8; N],
	f: impl Fn([u8; L]) -> T,
) -> (T, &[u8; R]) {
	let (left, right) = stdx::split_array_ref(buf);
	(f(*left), right)
}

/// Writes given fixed-width buffer at the start the buffer and returns the
/// remaining portion of the buffer.
///
/// By working on a fixed-size buffers, this avoids any run-time checks.  Sizes
/// are verified at compile-time.
fn write<const L: usize, const R: usize, const N: usize>(
	buf: &mut [u8; N],
	data: [u8; L],
) -> &mut [u8; R] {
	let (left, right) = stdx::split_array_mut(buf);
	*left = data;
	right
}

#[test]
fn test_header_encoding() {
	const ONE: CryptoHash = CryptoHash([1; 32]);

	assert_eq!(
		Some(Header {
			root_ptr: None,
			root_hash: sealable_trie::trie::EMPTY_TRIE_ROOT,
			next_block: Header::ENCODED_SIZE as u32,
			first_free: 0,
		}),
		Header::decode(&[0; 72])
	);

	let hdr = Header {
		root_ptr: Ptr::new(420).unwrap(),
		root_hash: ONE.clone(),
		next_block: 42,
		first_free: 24,
	};
	let got_bytes = hdr.encode();
	let got_hdr = Header::decode(&got_bytes);

	#[rustfmt::skip]
    assert_eq!([
        /* magic: */     1, 0, 0, 0,
        /* version: */   0, 0, 0, 0,
        /* root_ptr: */  164, 1, 0, 0,
        /* root_hash: */ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        /* next_block: */ 42, 0, 0, 0,
        /* first_free: */ 24, 0, 0, 0,
        /* tail: */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], got_bytes);
	assert_eq!(Some(hdr), got_hdr);
}

#[test]
fn test_trie_sanity() {
	const ONE: CryptoHash = CryptoHash([1; 32]);

	let key = solana_program::pubkey::Pubkey::new_unique();
	let mut lamports: u64 = 10 * solana_program::native_token::LAMPORTS_PER_SOL;
	let mut data = [0; SZ * 1000];
	let owner = solana_program::pubkey::Pubkey::new_unique();
	let account = solana_program::account_info::AccountInfo::new(
		/* key: */ &key,
		/* is signer: */ false,
		/* is writable: */ true,
		/* lamports: */ &mut lamports,
		/* data: */ &mut data[..],
		/* owner: */ &owner,
		/* executable: */ false,
		/* rent_epoch: */ 42,
	);

	{
		let mut trie = AccountTrie::new(account.data.borrow_mut()).unwrap();
		assert_eq!(Ok(None), trie.get(&[0]));

		assert_eq!(Ok(()), trie.set(&[0], &ONE));
		assert_eq!(Ok(Some(ONE.clone())), trie.get(&[0]));
	}

	{
		let mut trie = AccountTrie::new(account.data.borrow_mut()).unwrap();
		assert_eq!(Ok(Some(ONE.clone())), trie.get(&[0]));

		assert_eq!(Ok(()), trie.seal(&[0]));
		assert_eq!(Err(sealable_trie::Error::Sealed), trie.get(&[0]));
	}
}
