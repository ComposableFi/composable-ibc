use anchor_client::anchor_lang::prelude::*;
use anchor_client::anchor_lang;

#[derive(Debug, Clone, PartialEq)]
pub struct Deliver {
  pub messages: Vec<AnyCheck>,
}
impl borsh::ser::BorshSerialize for Deliver
where
  Vec<AnyCheck>: borsh::ser::BorshSerialize,
{
  fn serialize<W: borsh::maybestd::io::Write>(
      &self,
      writer: &mut W,
  ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
      borsh::BorshSerialize::serialize(&self.messages, writer)?;
      Ok(())
  }
}
impl borsh::de::BorshDeserialize for Deliver
where
  Vec<AnyCheck>: borsh::BorshDeserialize,
{
  fn deserialize(
      reader: &mut &[u8],
  ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
      Ok(Self {
          messages: borsh::BorshDeserialize::deserialize(reader)?,
      })
  }
}
impl anchor_lang::Discriminator for Deliver {
  const DISCRIMINATOR: [u8; 8] = [250, 131, 222, 57, 211, 229, 209, 147];
}
impl anchor_lang::InstructionData for Deliver {}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct AnyCheck {
  pub type_url: String,
  pub value: Vec<u8>,
}