use ethers::abi::Bytes;
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
	pub proof_init: Bytes, 					// proof that chainA stored connectionEnd in state (on ConnOpenInit)
	pub proof_client: Bytes, 					// proof that chainA stored a light client of chainB
	pub proof_consensus: Bytes,				// proof that chainA stored chainB's consensus state at consensus height
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
        
        //by some reason the decode is failing in case there is bytes in the struct
        //this is the reason why we are using the manual token construction
        //https://github.com/gakonst/ethers-rs/blob/master/ethers-contract/ethers-contract-derive/src/lib.rs#L140
        /* Token::Tuple(Self::into_tokens(self)) */

        use ethers::abi::encode as ethers_encode;
        use ethers::abi::Token as EthersToken;

        let client_state_data = EthersToken::Tuple(
            [
                //counterparty
                EthersToken::Tuple(
                    [
                        EthersToken::String(self.counterparty.client_id),
                        EthersToken::String(self.counterparty.connection_id),
                        EthersToken::Tuple(
                            [
                                EthersToken::Bytes(self.counterparty.prefix.key_prefix),
                            ].to_vec()),
                    ].to_vec()),
                //delay_period
                EthersToken::Uint(self.delay_period.into()),
                //client_id
                EthersToken::String(self.client_id),
                //client_state_bytes
                EthersToken::Bytes(self.client_state_bytes),
                //counterparty_versions
                EthersToken::Array(
                    self.counterparty_versions.iter().map(|version| {
                        EthersToken::Tuple(
                            [
                                EthersToken::String(version.identifier.clone()),
                                EthersToken::Array(
                                    version.features.iter().map(|feature| {
                                        EthersToken::String(feature.clone())
                                    }).collect::<Vec<EthersToken>>()
                                )
                            ].to_vec())
                    }).collect::<Vec<EthersToken>>()
                ),
                //proof_init
                EthersToken::Bytes(self.proof_init),
                //proof_client
                EthersToken::Bytes(self.proof_client),
                //proof_consensus
                EthersToken::Bytes(self.proof_consensus),
                //proof_height
                EthersToken::Tuple(
                    [
                        EthersToken::Uint(self.proof_height.revision_number.into()),
                        EthersToken::Uint(self.proof_height.revision_height.into()),
                    ].to_vec()),
                //consensus_height
                EthersToken::Tuple(
                    [
                        EthersToken::Uint(self.consensus_height.revision_number.into()),
                        EthersToken::Uint(self.consensus_height.revision_height.into()),
                    ].to_vec()),
            ].to_vec());
        return client_state_data;
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

