use anchor_client::anchor_lang::prelude::*;

pub struct LocalDeliver {
  pub sender: AccountMeta,
  pub storage: AccountMeta,
  pub trie: AccountMeta,
  pub system_program: AccountMeta,
}

impl LocalDeliver {
  pub fn new(sender_key: Pubkey, storage_key: Pubkey, trie_key: Pubkey, system_program_key: Pubkey) -> Self {
      let sender = AccountMeta::new(sender_key, true);
      let storage = AccountMeta::new(storage_key, false);
      let trie = AccountMeta::new(trie_key, false);
      let system_program = AccountMeta::new_readonly(system_program_key, false);
      Self {
          sender,
          storage,
          trie,
          system_program
      }
  }
}

impl ToAccountMetas for LocalDeliver {
  fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
      let mut accounts = Vec::new();
      accounts.push(self.sender.clone());
      accounts.push(self.storage.clone());
      accounts.push(self.trie.clone());
      accounts.push(self.system_program.clone());
      accounts
  }
}