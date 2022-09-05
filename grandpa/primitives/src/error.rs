use derive_more::{Display, From};

#[derive(From, Display)]
pub enum Error {
	/// Anyhow error
	Anyhow(anyhow::Error),
	/// Grandpa finality error
	#[display(fmt = "NotDescendent")]
	Grandpa(finality_grandpa::Error),
	/// scale codec error
	Codec(codec::Error),
}
