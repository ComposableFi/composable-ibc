use ethers::abi::Token;
use ibc::core::ics02_client::height::Height;

use super::IntoToken;

impl IntoToken for Height {
	fn into_token(self) -> Token {
		Token::Tuple(vec![
			Token::Uint(self.revision_number.into()),
			Token::Uint(self.revision_height.into()),
		])
	}
}
