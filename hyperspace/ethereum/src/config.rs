use std::str::FromStr;

use ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

fn uri_de<'de, D>(de: D) -> Result<http::uri::Uri, D::Error>
where
	D: Deserializer<'de>,
{
	struct FromStr;

	impl Visitor<'_> for FromStr {
		type Value = http::uri::Uri;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(formatter, "a string that can parse into a http URI")
		}

		fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			http::uri::Uri::from_str(&v).map_err(serde::de::Error::custom)
		}
	}

	de.deserialize_str(FromStr)
}

fn uri_se<S>(uri: &http::uri::Uri, ser: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	ser.serialize_str(&format!("{uri}"))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
	/// HTTP URL for RPC
	#[serde(deserialize_with = "uri_de", serialize_with = "uri_se")]
	pub rpc_url: http::uri::Uri,
	/// Websocket URL for RPC
	#[serde(deserialize_with = "uri_de", serialize_with = "uri_se")]
	pub ws_url: http::uri::Uri,
	/// Name of the chain
	pub name: String,
	/// Cosmos chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Whitelisted channels
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
}

#[cfg(test)]
mod test;
