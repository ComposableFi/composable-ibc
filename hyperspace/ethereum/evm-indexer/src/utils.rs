use ethers::types::{Bytes, H160, H256, H64, U256, U64};

use ethers::contract::abigen;

abigen!(
	IbcClientAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-client-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcChannelAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-channel-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcConnectionAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-connection-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcPacketAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-packet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcQuerierAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-querier-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	Ics20TransferBankAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ics20-transfer-bank-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	Ics20BankAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ics20-bank-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	TendermintClientAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/tendermint-client-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/diamond-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondCutFacetAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/diamond-cut-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondLoupeFacetAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/diamond-loupe-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	OwnershipFacetAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ownership-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);
);

pub fn format_nonce(h: H64) -> String {
	return format!("{:?}", h)
}

pub fn format_bool(h: U64) -> bool {
	let data = format!("{:?}", h);
	return data == "1"
}

pub fn format_hash(h: H256) -> String {
	return format!("{:?}", h)
}

pub fn format_address(h: H160) -> String {
	return format!("{:?}", h)
}

pub fn format_bytes(b: &Bytes) -> String {
	return format!("{}", serde_json::to_string(b).unwrap().replace("\"", ""))
}

pub fn format_bytes_slice(b: &[u8]) -> String {
	return format!("0x{}", hex::encode(b))
}

pub fn format_number(n: U256) -> String {
	return format!("{}", n.to_string())
}

pub fn format_small_number(n: U64) -> String {
	return format!("{}", n.to_string())
}
