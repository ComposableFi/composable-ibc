use ethers::abi::Token;

pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics02_client;

pub trait IntoToken {
    fn into_token(self) -> Token;
}

impl IntoToken for String{
    fn into_token(self) -> Token {
        Token::String(self)
    }
}

impl IntoToken for &str{
    fn into_token(self) -> Token {
        Token::String(self.to_string())
    }
}

impl IntoToken for &[u8]{
    fn into_token(self) -> Token {
        Token::Bytes(self.to_vec())
    }
}