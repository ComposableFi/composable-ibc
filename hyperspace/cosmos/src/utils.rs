use crate::error::Error;
use ibc::core::ics02_client::events::Attributes as ClientAttributes;
use tendermint_rpc::abci::Event as AbciEvent;

pub fn client_extract_attributes_from_tx(event: &AbciEvent) -> Result<ClientAttributes, Error> {
	let mut attr = ClientAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		match key {
			"client_id" => {
				attr.client_id = value
					.parse()
					.map_err(|e| Error::from(format!("Failed to parse client id {:?}", e)))?
			},
			"client_type" => {
				attr.client_type = value
					.parse()
					.map_err(|e| Error::from(format!("Failed to parse client type {:?}", e)))?
			},
			"consensus_height" => {
				attr.consensus_height = value
					.parse()
					.map_err(|e| Error::from(format!("Failed to parse consensus height {:?}", e)))?
			},
			_ => {},
		}
	}

	Ok(attr)
}
