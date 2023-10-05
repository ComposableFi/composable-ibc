use ethers::abi::Token;
use ibc::core::ics24_host::identifier::ConnectionId;

use super::IntoToken;

pub mod conn_open_try;

impl IntoToken for ConnectionId {
	fn into_token(self) -> Token {
		Token::String(self.to_string())
	}
}
