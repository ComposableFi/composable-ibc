use ethers::abi::Token;
use ibc::core::{ics04_channel::{channel::{State, Order, Counterparty, ChannelEnd}, Version, msgs::chan_open_init::MsgChannelOpenInit}, ics24_host::identifier::{ChannelId, PortId}};


use super::IntoToken;

impl IntoToken for State{
    fn into_token(self) -> Token {
        match self {
            State::Init => Token::Uint(1.into()),
            State::TryOpen => Token::Uint(2.into()),
            State::Open => Token::Uint(3.into()),
            State::Closed => Token::Uint(4.into()),
        }
    }
}

impl IntoToken for Order{
    fn into_token(self) -> Token {
        match self {
            Order::Unordered => Token::Uint(0.into()),
            Order::Ordered => Token::Uint(1.into()),
        }
    }
}

impl IntoToken for Counterparty{
    fn into_token(self) -> Token {
        let channel_id = match &self.channel_id {
            Some(channel_id) => channel_id.to_string(),
            None => String::new(),
        };
        Token::Tuple(vec![
            self.port_id.as_str().into_token(),
            channel_id.into_token(),
        ])
    }
}

impl IntoToken for PortId{
    fn into_token(self) -> Token {
        Token::String(self.to_string())
    }
}

impl IntoToken for Version{
    fn into_token(self) -> Token {
        Token::String(self.to_string())
    }
}

impl IntoToken for ChannelEnd{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.state.into_token(),
            self.ordering.into_token(),
            self.remote.into_token(),
            Token::Array(self.connection_hops.into_iter().map(IntoToken::into_token).collect()),
            self.version.into_token(),
        ])
    }
}

impl IntoToken for MsgChannelOpenInit{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel.into_token(),
        ])
    }
}