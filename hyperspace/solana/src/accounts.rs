use anchor_client::anchor_lang::prelude::*;

/// Accounts required for calling the deliver method on smart contract
pub struct LocalDeliver {
	/// Signer
	pub sender: AccountMeta,
	/// Storage Account for all the IBC data
	pub storage: AccountMeta,
	/// Sealable trie which stores the proof
	pub trie: AccountMeta,
	/// System Program (`11111111111111111111111111111111`) used to create the above 2 accounts if
	/// not already created
	pub system_program: AccountMeta,
}

impl LocalDeliver {
	pub fn new(
		sender_key: Pubkey,
		storage_key: Pubkey,
		trie_key: Pubkey,
		system_program_key: Pubkey,
	) -> Self {
		let sender = AccountMeta::new(sender_key, true);
		let storage = AccountMeta::new(storage_key, false);
		let trie = AccountMeta::new(trie_key, false);
		let system_program = AccountMeta::new_readonly(system_program_key, false);
		Self { sender, storage, trie, system_program }
	}
}

impl ToAccountMetas for LocalDeliver {
	fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
		let mut accounts = Vec::new();
		accounts.push(self.sender.clone());
		accounts.push(self.storage.clone());
		accounts.push(self.trie.clone());
		accounts.push(self.system_program.clone());
		accounts
	}
}
