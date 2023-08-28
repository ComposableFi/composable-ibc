use ethers::abi::Token;
use ibc::core::{ics04_channel::{channel::{State, Order, Counterparty, ChannelEnd}, Version, msgs::{chan_open_init::MsgChannelOpenInit, chan_open_try::MsgChannelOpenTry, chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm, chan_close_init::MsgChannelCloseInit, chan_close_confirm::MsgChannelCloseConfirm}}, ics24_host::identifier::{ChannelId, PortId}};


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

impl IntoToken for ChannelId{
    fn into_token(self) -> Token {
        Token::String(self.to_string())
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

impl IntoToken for MsgChannelOpenTry{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel.into_token(),
            self.counterparty_version.to_string().into_token(),
            self.proofs.object_proof().as_bytes().into_token(),
            self.proofs.height().into_token()
        ])
    }
}
impl IntoToken for MsgChannelOpenAck{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel_id.into_token(),
            self.counterparty_version.to_string().into_token(),
            self.counterparty_channel_id.into_token(),
            self.proofs.object_proof().as_bytes().into_token(),
            self.proofs.height().into_token()
        ])
    }
}

impl IntoToken for MsgChannelOpenConfirm{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel_id.into_token(),
            self.proofs.object_proof().as_bytes().into_token(),
            self.proofs.height().into_token()
        ])
    }
}

impl IntoToken for MsgChannelCloseInit{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel_id.into_token(),
        ])
    }
}

impl IntoToken for MsgChannelCloseConfirm{
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            self.port_id.into_token(),
            self.channel_id.into_token(),
            self.proofs.object_proof().as_bytes().into_token(),
            self.proofs.height().into_token()
        ])
    }
}
