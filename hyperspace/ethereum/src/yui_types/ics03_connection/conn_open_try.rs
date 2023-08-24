use ethers::abi::Token;
use ethers::prelude::EthAbiType;
use ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
use primitives::mock::LocalClientTypes;
use ethers::abi::Detokenize;
use ethers::abi::Tokenize;

#[derive(Clone, Debug, PartialEq, Eq, EthAbiType)]
pub struct YuiMsgConnectionOpenTry {
	pub counterparty: YuiCounterparty,
	pub delay_period: u64,
	pub client_id: String, 						// clientID of chainA
	pub client_state_bytes: Vec<u8>, 			// clientState that chainA has for chainB
	pub counterparty_versions: Vec<YuiVersion>, // supported versions of chain A
	pub proof_init: Vec<u8>, 					// proof that chainA stored connectionEnd in state (on ConnOpenInit)
	pub proof_client: Vec<u8>, 					// proof that chainA stored a light client of chainB
	pub proof_consensus: Vec<u8>,				// proof that chainA stored chainB's consensus state at consensus height
	pub proof_height: YuiHeight, 				// height at which relayer constructs proof of A storing connectionEnd in state
	pub consensus_height: YuiHeight, 			// latest height of chain B which chain A has stored in its chain B client
}
// , client_state: ClientState<H>
impl From<MsgConnectionOpenTry::<LocalClientTypes>> for YuiMsgConnectionOpenTry{
    fn from(value: MsgConnectionOpenTry::<LocalClientTypes>) -> Self {
        unimplemented!();
    }
}

impl YuiMsgConnectionOpenTry{
    pub fn new() -> Self{
        unimplemented!();
    }

    pub fn token(self) -> Token{
        Token::Tuple(Self::into_tokens(self))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EthAbiType)]
pub struct YuiCounterparty {
	pub client_id: String,
	pub connection_id: String,
	pub prefix: YuiCommitmentPrefix,
}

#[derive(Clone, Debug, PartialEq, Eq, EthAbiType)]
pub struct YuiCommitmentPrefix {
	pub key_prefix: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, EthAbiType)]
pub struct YuiHeight {
	pub revision_number: u64,
	pub revision_height: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, EthAbiType)]
pub struct YuiVersion {
	pub identifier: String,
	pub features: Vec<String>,
}

