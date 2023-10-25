use anchor_client::{anchor_lang, anchor_lang::prelude::*};

#[derive(Debug, Clone, PartialEq, AnchorSerialize, AnchorDeserialize)]
/// Arguments to the instruction
pub struct Deliver {
	pub messages: Vec<AnyCheck>,
}

/// 8 byte unique identifier for Deliver Type
impl anchor_lang::Discriminator for Deliver {
	const DISCRIMINATOR: [u8; 8] = [250, 131, 222, 57, 211, 229, 209, 147];
}
impl anchor_lang::InstructionData for Deliver {}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct AnyCheck {
	pub type_url: String,
	pub value: Vec<u8>,
}
