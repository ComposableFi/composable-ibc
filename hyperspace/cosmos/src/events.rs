use crate::TimeoutHeight;
use core::{
	convert::TryFrom,
	fmt::{Display, Error as FmtError, Formatter},
};
use ibc::protobuf::Protobuf;
use ibc::{
	// clients::ics07_tendermint::header::decode_header as tm_decode_header,
	core::{
		ics02_client::{
			error::Error as ClientError,
			events::{self as client_events, Attributes as ClientAttributes},
			// client_message::Header,
			height::{Height, HeightErrorDetail},
		},
		ics03_connection::{
			error::Error as ConnectionError,
			events::{self as connection_events, Attributes as ConnectionAttributes},
		},
		ics04_channel::{
			error::Error as ChannelError,
			events::{self as channel_events, Attributes as ChannelAttributes},
			packet::Packet,
			// timeout::TimeoutHeight,
		},
	},
	events::{Error as IbcEventError, IbcEvent, IbcEventType},
};
use ics07_tendermint::client_message::{decode_header as tm_decode_header, Header};
use serde::Serialize;
use tendermint::abci::Event as AbciEvent;

pub const HEADER_ATTRIBUTE_KEY: &str = "header";

#[derive(Clone, Debug, Serialize)]
pub struct IbcEventWithHeight {
	pub event: IbcEvent,
	pub height: Height,
}

impl IbcEventWithHeight {
	pub fn new(event: IbcEvent, height: Height) -> Self {
		Self { event, height }
	}

	pub fn with_height(self, height: Height) -> Self {
		Self { event: self.event, height }
	}
}

impl Display for IbcEventWithHeight {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(f, "{} at height {}", self.event, self.height)
	}
}

pub fn event_is_type_client(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::CreateClient(_) |
			IbcEvent::UpdateClient(_) |
			IbcEvent::UpgradeClient(_) |
			IbcEvent::ClientMisbehaviour(_) |
			IbcEvent::PushWasmCode(_)
	)
}

pub fn event_is_type_connection(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::OpenInitConnection(_) |
			IbcEvent::OpenTryConnection(_) |
			IbcEvent::OpenAckConnection(_) |
			IbcEvent::OpenConfirmConnection(_)
	)
}

pub fn event_is_type_channel(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::OpenInitChannel(_) |
			IbcEvent::OpenTryChannel(_) |
			IbcEvent::OpenAckChannel(_) |
			IbcEvent::OpenConfirmChannel(_) |
			IbcEvent::CloseInitChannel(_) |
			IbcEvent::CloseConfirmChannel(_) |
			IbcEvent::SendPacket(_) |
			IbcEvent::ReceivePacket(_) |
			IbcEvent::WriteAcknowledgement(_) |
			IbcEvent::AcknowledgePacket(_) |
			IbcEvent::TimeoutPacket(_) |
			IbcEvent::TimeoutOnClosePacket(_)
	)
}

/*
[2023-02-06T13:11:05Z INFO  hyperspace] Simulated transaction: events: [Event { r#type: "coin_spent", attributes: [EventAttribute { key: b"spender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }, EventAttribute { key: b"amount", value: b"1000000stake", index: false }] }, Event { r#type: "coin_received", attributes: [EventAttribute { key: b"receiver", value: b"cosmos17xpfvakm2amg962yls6f84z3kell8c5lserqta", index: false }, EventAttribute { key: b"amount", value: b"1000000stake", index: false }] }, Event { r#type: "transfer", attributes: [EventAttribute { key: b"recipient", value: b"cosmos17xpfvakm2amg962yls6f84z3kell8c5lserqta", index: false }, EventAttribute { key: b"sender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }, EventAttribute { key: b"amount", value: b"1000000stake", index: false }] }, Event { r#type: "message", attributes: [EventAttribute { key: b"sender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }] }, Event { r#type: "tx", attributes: [EventAttribute { key: b"fee", value: b"1000000stake", index: false }, EventAttribute { key: b"fee_payer", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }] }, Event { r#type: "tx", attributes: [EventAttribute { key: b"acc_seq", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0/11", index: false }] }, Event { r#type: "tx", attributes: [EventAttribute { key: b"signature", value: b"NoQoXpQgPHW7npGujUt3JCKfDk668t49F81xO1rjcQcCM+X2NU6tnqbvvhaOF4PtKUCScygOZtRj9EOrGfu2ow==", index: false }] }, Event { r#type: "message", attributes: [EventAttribute { key: b"action", value: b"/ibc.core.client.v1.MsgUpdateClient", index: false }] }, Event { r#type: "update_client", attributes: [EventAttribute { key: b"client_id", value: b"08-wasm-0", index: false }, EventAttribute { key: b"client_type", value: b"08-wasm", index: false }, EventAttribute { key: b"consensus_height", value: b"2000-21", index: false }, EventAttribute { key: b"consensus_heights", value: b"2000-21", index: false }, EventAttribute { key: b"header", value: b"0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212d8210ace210a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212a6210aa9160a20de8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76412ba030300000000000000de8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a764600000000cde8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76460000000ad07a8edfc8fd68436558991e57bf23de7a0c71f78ad57a8f60e44a0d08b19a6ad59a51c61302d11c2760a92bc71490813ba5210383df4a6bba63c2d54cb5505439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234fde8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a7646000000000ff1f4b6fff9f456c1444ee9addb5ea5588686f1fda8d89d4ea045e649b9d9d5b820e3a19d77f50f2565fe96dee8c4a770a905aa0dc7339142fe92fd448820388dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0eede8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76460000000f88737c1a4345337eacc554597724178ac8deb6b2d4be16c9c84b7109093cf6dcda30a1ec5c8cbac1175803d91de785f4a7550769737f7cf59ad5bcbaf2f160bd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001adf05d3c96f0a809a65b0fdbf80313601d38522a224a19a23fb2d59e911f25b034f626d013c94361cb40f149592c2a91ea6a6409d10a8f76617ee0ea8d93d15087cfe60dc0a9af821694dac9417b40fbfdd8a12a7d86628a7059384d7c43045724a09683e180642414245b50103020000005e7fa510000000004c0963ff024098d479e2d11981c08777d2474a0ecdc67eb54b4bdfb3f0400f742fbac8840a193b8b10de1737c0945bed487f7714ccda6d63e605c20c7c5ed90b6dfc6f6309d90eedd4e34e128996002d16530d3123c183bff7a0bfdcc43b610504424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe2201000000000000007543bec20c9a19bde289ba4fb027caf1ad5b1e96776eb43557a74d1ffd1e50b00442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb090000000000000004424545468403ae8c82117aeaa8dbd752e2b1557d4f26cee8c9ee277a2227ff960e9ae6c958b70446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f010000000000000000000000054241424501013c0eaeed5120ea46eda2000a9ae351d31ba215279f28a46ef10046a578341a765defff60bf2369fc8e4fadcf6194fd26b95d6f41c95ba54a32f29135733192841ac5025f30b40d8e2e8e822537c1ea80b468ee5795180e9debe58c73351d92c5cc07fa7101b51a0cd53c08f37768345a20bc18daf544c316447224d138e57bf584fa6a57a6932d890a495ec818161bc5efcb080185d2615a979bf1d86b78dbff4efd5b748b0c0642414245b50103010000005f7fa510000000001e4e8d669517df98e1247cd9cac963713342a16fd40ab2589ae1877f8e380c19e3b17adc735f16fd74de0ede21f416f11e7b9dd7f55986de0913c192793ce90de7ff8fb7777a11c63a810555745b152370f0eabcfe9cb8ea8d73138f01668c0604424545468403b7fbe19743803f15da5a181841af9e93bea97c8b43bd0b4d32ef8a109c6cbda705424142450101fc963fc2547a2fe85ceeec0b0d495902afd24162e29863a9f8b0cf14302d7d07b15b4c040a695367b0122e837dbdc3e942c914d2f32f06adf0e0471c28baf6841ac502abb36d9d8358e0982559fe201e05b62bcfde6e05190239c792df266e84a56092750175d80e406bcfdbd038502be6a47ef6b30bbe2ca6a8520d028ebcc3cc55adba584f82b95fac0cb97aa93cb629f725763f9ce944b01ac6258a068ef944527fb5880c0642414245b5010102000000607fa510000000001054f852cb2967dd3202414ff7f70ead28b2d9591d591000ca260c17c6afc7169fbd5f46f76e937b1f04292ab7aba2b24473fcf73e77a17276a6df59633e3c0c474cf4a2d70ade14c44c4039777d8c76e56f061a78df882285fb84a7e2b0ab0204424545468403efbb141b1301a43b598f2533daa3a0873fac3b9dcd92cafd5df6514ddbc1913505424142450101523c33df9b6b752a1a1feeb0f84de10cdb758dab091afa445027a7325a591e14be631a3c89080cb7d8362bd5cc0ca21f8ed08316350276b4973ef49940ade1871ac502cd14bfe447807e4ed788d6a74735816c2354c6cea7f67cf6fe7869ccd3f959057901aaa2163f44f8ede60f1b6589a763d4dc6201e883f1e0641054cc0b74874539aa5b2e0157ca3e3125b78a9cd9d2f40b302975a17d008a5097b3d1d3a1bb9fe7d20c0642414245b5010300000000617fa51000000000e48817ee21c6eaf74ed7b18e2c7c53e19cb1944e065c7934fdcedb87035189566ef784f7a53f2e3c752b6252426d5a29280f5ff9cde6555abd499cd6592c1e0b2325aa6193ec8629cfcd054ad70caa1c79fac6d3ac121d9ecc378568ea26110d04424545468403ca0024665816a7fd0543ee59333b4bab215d8b78138fd95a0e21fd5a2de824220542414245010108465a236a606f0a7029e1c7610a472d5d7a7c48258e8edb7ac6db466ae6f65f37bc5a25dbb7532bff0e281fd5aeeac5cf1dffd92e50d985e4429957a906b4871ac50219e4abaa362ffecb5efea47429fb82d3e10cb151362c6dad09db0e0982958f5e7d0115960d22abee2f24f8785cfe795e6b56e989e0d9a05b9650da1f0cda621531cf0136235457158a0db5d7b6b6f9eddcb8b8194cb3a471931443f5cbb25e325c030c0642414245b5010300000000627fa51000000000fa4712546333adfae2ebf45b1e38d7db705488c119c9af97d8ce5bd3e565da3b3f41cb2b6a32b25caf8e34f14656eb3e03737a3676af0de287be4f5ad9b303037000202a6b173423f7346fd45484f126177f9ad3c0141c573d2de16c86a9260c04424545468403c34d5717e561af3e35000a84f622aa053e25b60997225f15fe21d3cd3fc6724c05424142450101b630a4388a1098d661b6a6f7df7ea0e76d241a7df56a0a5d6526ad4791f1a964687c2fa22a348c277aa79af18c5483b0986c8c9fdeb7495bc327578e2760cd871ac5027af58ddde93cb2105b7cae84e58dfcfafa0a8e554f4a0c71a205772e6101d58d81013ffe7a945830f411bd5896e6453c5459ad8dd0ba0ff594451331376c91d6a65793b19866d2ab3672e6f8147f27ec191681d4dc3b69dcc862b29c3f982149f8df0c0642414245b5010301000000637fa51000000000ecf65faa3ab34021c76a758b3712476deb796a64394758cc8fe249ce4175674a6a2d76cae94ac0121bea2fb85e8922727a213bcda2fcef82d5ec423b29fd0e0fcda12c5c32b4be1a901daabdcc6677e98e9ff51ed30490c9dff00204ba3e84070442454546840386eca99994a1e01efd73cb69f830bb0f90e678c80967d294ff68a189d541076f05424142450101365f5c24f5fd229a357df3578c496a26d1e11ff8a567d06eb98eb75fe49e9e55204c930c785a5dd18bacb8f06e64a442966cbae0859b55819e9bec6fc801918512f70a0a207af58ddde93cb2105b7cae84e58dfcfafa0a8e554f4a0c71a205772e6101d58d12d20a0a2d3703f5a4efb16ffa83d007000083a874f5953ebc28142cb2184f2227e8fc1a2a930665b6269dcbfd64da0b179b0a870180046480f6cbcf8517e448ae891923f1d9aebdd7a1e435a18d593074ff6cf873ccb4ac4580f6f6801e4b41e2e6d8ec194dba122bfb9eb33feb2545ef5144cea79551f7cc52802962bfbdbc4c3fee805d547f8fb08db560b03ec56721ce05f7fdffe7d6892ffa80b8497c75f80a57982ff29ed2c42c347b5158f40ab78079f506a3f0e10ad0ce630a930480ffff80a244132166cca1b9cb319f1362aa46c025824965ca24af5fbf2ca3f4a762e08c806cb588798afe499ef4fad06a5d547a4b9baf7cc471ce81cf40d6ec5743d23b6d805bfb7edd5c0eaf177acc68eeb1bb05f86803769f0c113500515551f2859e30aa80c35647d5ef63e87b7c4d9ed2b88c9b78553b1019e8c2a47b7c1e65fb973df75b80b872082faf350984dc3173561d7f7e1e37dd90fdeda6f929181bef3457632654800b9e5fde7411eb39fbe31354d1f8c092c52812c7e351ca8824bf07e42572533680ae3a3246726e8113f3dafe3f3a895d9cc0b21085974ea332ad3dca85fec21656805b682132c52908705526057f73ab7fccab4af6d72a9805634dd8d3cc53f130d180c2d44d371e5fc1f50227d7491ad65ad049630361cefb4ab1844831237609f0838033ef217f33a7bf2497bd3b532d5196363603b0a685cd5e253b1b7ef6ef3186c2800cdc2830c3697b7128e33d81118472b04783e76b01177c26cb5f29a23f5713c980330a556b0c7ee12c7ba20d775b9dc705b5b2779282d0170e384744768427f4ad802f664b4cf1449a239a94171acdb7039035923e9e5644530aa837f575f94569a1804c59925a4e422dc87a33d8d2bcca62efa7fb2fd985e6c281675e9b2df0eeec79806bfdbbf0e0bedcb993b65c9cea1e929a56d78a3b7bc53d1b7ca6fc488e2295ee80f49ac3396db69998b2428f2de31fd9ca4acff6663b4a42ea7aa3f6c1ae8355d40a95029e710b30bd2eab0352ddcc26417aa1945f43803b3441f15daa8a53147d69c48eac75356fab581febbb8030520b248c5942a14880a404a091241950948d6e2399cc5949818a2c0bab7d33c1aa2a496d1cdc2a5d72802e2e0716043a02f2f29fdd52922704af194b98545ce5ca832255e8ec41bcdb648015306063e6f316b68ee811a531146c8f64e0a490cc0fbca9b83afce9d7699799505f0e7b9012096b41c4eb3aaf947f6ea4290800004c5f03c716fb8fff3de61a883bb76adb34a2040080ca356caacc573edab93da993e39702d2cff8a467e3074b14a8b2c4db9a5f4a1b4c5f0f4993f016e2d2f8e5f43be7bb259486040080b842c13f848818d7d2869ede474bb616f1c39752fc94636aed5706e4872e00c10a559f0b3c252fcb29d88eff4f3de5de4476c3500080a735ba4368d8d5a4b0733aca7dbdb8df42ef6c48b425623e92d3aca67a2fd1a0808d61f62a0339287be1b499bfdb18bf74ae128504f3b6ac946b263cca88de73ed0ab901dd02ab28bfb62ceb4e02ae2e255a88c42a134e74b848bdc0efbe58fef1596ed8e14e54c409c1b2897ad2b437395fb17bc30a92898095e5e3d8bb2fe5c14214b4a12f5298afd7cad48bcf25e3e1b9e6f054c6c3d5dceb810b8a58bb5d0e71825b401bd908066175726120b0bf5208000000000561757261010144b9d39fe5c4f4808eebb547fab57a10a3a336e3a94a6ae0b0bf79fdf9dd964cbea565b0ae654412e01d4121b01be9ef32a4e20cf252b7201174bfc0a9917f81120b280401000b1a5ad92686011a49810011010840008064f9a32e7466b08244144e633bbb1d122b6a7e3f66555ef62baedc9146b26ee0800b9e7d3145822242b5a382127b49323d44c0b9ac1003dc93f2216ed0f7e7a246120508d00f1060", index: false }] }, Event { r#type: "message", attributes: [EventAttribute { key: b"module", value: b"ibc_client", index: false }] }]
	logs: [{"msg_index":0,"events":[{"type":"message","attributes":[{"key":"action","value":"/ibc.core.client.v1.MsgUpdateClient"},{"key":"module","value":"ibc_client"}]},{"type":"update_client","attributes":[{"key":"client_id","value":"08-wasm-0"},{"key":"client_type","value":"08-wasm"},{"key":"consensus_height","value":"2000-21"},{"key":"consensus_heights","value":"2000-21"},{"key":"header","value":"0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212d8210ace210a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212a6210aa9160a20de8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76412ba030300000000000000de8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a764600000000cde8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76460000000ad07a8edfc8fd68436558991e57bf23de7a0c71f78ad57a8f60e44a0d08b19a6ad59a51c61302d11c2760a92bc71490813ba5210383df4a6bba63c2d54cb5505439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234fde8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a7646000000000ff1f4b6fff9f456c1444ee9addb5ea5588686f1fda8d89d4ea045e649b9d9d5b820e3a19d77f50f2565fe96dee8c4a770a905aa0dc7339142fe92fd448820388dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0eede8ed3490dbf7158f82351a5fd0c4fe6dd6be791df0a2a8aa073d6221e18a76460000000f88737c1a4345337eacc554597724178ac8deb6b2d4be16c9c84b7109093cf6dcda30a1ec5c8cbac1175803d91de785f4a7550769737f7cf59ad5bcbaf2f160bd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001adf05d3c96f0a809a65b0fdbf80313601d38522a224a19a23fb2d59e911f25b034f626d013c94361cb40f149592c2a91ea6a6409d10a8f76617ee0ea8d93d15087cfe60dc0a9af821694dac9417b40fbfdd8a12a7d86628a7059384d7c43045724a09683e180642414245b50103020000005e7fa510000000004c0963ff024098d479e2d11981c08777d2474a0ecdc67eb54b4bdfb3f0400f742fbac8840a193b8b10de1737c0945bed487f7714ccda6d63e605c20c7c5ed90b6dfc6f6309d90eedd4e34e128996002d16530d3123c183bff7a0bfdcc43b610504424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe2201000000000000007543bec20c9a19bde289ba4fb027caf1ad5b1e96776eb43557a74d1ffd1e50b00442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb090000000000000004424545468403ae8c82117aeaa8dbd752e2b1557d4f26cee8c9ee277a2227ff960e9ae6c958b70446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f010000000000000000000000054241424501013c0eaeed5120ea46eda2000a9ae351d31ba215279f28a46ef10046a578341a765defff60bf2369fc8e4fadcf6194fd26b95d6f41c95ba54a32f29135733192841ac5025f30b40d8e2e8e822537c1ea80b468ee5795180e9debe58c73351d92c5cc07fa7101b51a0cd53c08f37768345a20bc18daf544c316447224d138e57bf584fa6a57a6932d890a495ec818161bc5efcb080185d2615a979bf1d86b78dbff4efd5b748b0c0642414245b50103010000005f7fa510000000001e4e8d669517df98e1247cd9cac963713342a16fd40ab2589ae1877f8e380c19e3b17adc735f16fd74de0ede21f416f11e7b9dd7f55986de0913c192793ce90de7ff8fb7777a11c63a810555745b152370f0eabcfe9cb8ea8d73138f01668c0604424545468403b7fbe19743803f15da5a181841af9e93bea97c8b43bd0b4d32ef8a109c6cbda705424142450101fc963fc2547a2fe85ceeec0b0d495902afd24162e29863a9f8b0cf14302d7d07b15b4c040a695367b0122e837dbdc3e942c914d2f32f06adf0e0471c28baf6841ac502abb36d9d8358e0982559fe201e05b62bcfde6e05190239c792df266e84a56092750175d80e406bcfdbd038502be6a47ef6b30bbe2ca6a8520d028ebcc3cc55adba584f82b95fac0cb97aa93cb629f725763f9ce944b01ac6258a068ef944527fb5880c0642414245b5010102000000607fa510000000001054f852cb2967dd3202414ff7f70ead28b2d9591d591000ca260c17c6afc7169fbd5f46f76e937b1f04292ab7aba2b24473fcf73e77a17276a6df59633e3c0c474cf4a2d70ade14c44c4039777d8c76e56f061a78df882285fb84a7e2b0ab0204424545468403efbb141b1301a43b598f2533daa3a0873fac3b9dcd92cafd5df6514ddbc1913505424142450101523c33df9b6b752a1a1feeb0f84de10cdb758dab091afa445027a7325a591e14be631a3c89080cb7d8362bd5cc0ca21f8ed08316350276b4973ef49940ade1871ac502cd14bfe447807e4ed788d6a74735816c2354c6cea7f67cf6fe7869ccd3f959057901aaa2163f44f8ede60f1b6589a763d4dc6201e883f1e0641054cc0b74874539aa5b2e0157ca3e3125b78a9cd9d2f40b302975a17d008a5097b3d1d3a1bb9fe7d20c0642414245b5010300000000617fa51000000000e48817ee21c6eaf74ed7b18e2c7c53e19cb1944e065c7934fdcedb87035189566ef784f7a53f2e3c752b6252426d5a29280f5ff9cde6555abd499cd6592c1e0b2325aa6193ec8629cfcd054ad70caa1c79fac6d3ac121d9ecc378568ea26110d04424545468403ca0024665816a7fd0543ee59333b4bab215d8b78138fd95a0e21fd5a2de824220542414245010108465a236a606f0a7029e1c7610a472d5d7a7c48258e8edb7ac6db466ae6f65f37bc5a25dbb7532bff0e281fd5aeeac5cf1dffd92e50d985e4429957a906b4871ac50219e4abaa362ffecb5efea47429fb82d3e10cb151362c6dad09db0e0982958f5e7d0115960d22abee2f24f8785cfe795e6b56e989e0d9a05b9650da1f0cda621531cf0136235457158a0db5d7b6b6f9eddcb8b8194cb3a471931443f5cbb25e325c030c0642414245b5010300000000627fa51000000000fa4712546333adfae2ebf45b1e38d7db705488c119c9af97d8ce5bd3e565da3b3f41cb2b6a32b25caf8e34f14656eb3e03737a3676af0de287be4f5ad9b303037000202a6b173423f7346fd45484f126177f9ad3c0141c573d2de16c86a9260c04424545468403c34d5717e561af3e35000a84f622aa053e25b60997225f15fe21d3cd3fc6724c05424142450101b630a4388a1098d661b6a6f7df7ea0e76d241a7df56a0a5d6526ad4791f1a964687c2fa22a348c277aa79af18c5483b0986c8c9fdeb7495bc327578e2760cd871ac5027af58ddde93cb2105b7cae84e58dfcfafa0a8e554f4a0c71a205772e6101d58d81013ffe7a945830f411bd5896e6453c5459ad8dd0ba0ff594451331376c91d6a65793b19866d2ab3672e6f8147f27ec191681d4dc3b69dcc862b29c3f982149f8df0c0642414245b5010301000000637fa51000000000ecf65faa3ab34021c76a758b3712476deb796a64394758cc8fe249ce4175674a6a2d76cae94ac0121bea2fb85e8922727a213bcda2fcef82d5ec423b29fd0e0fcda12c5c32b4be1a901daabdcc6677e98e9ff51ed30490c9dff00204ba3e84070442454546840386eca99994a1e01efd73cb69f830bb0f90e678c80967d294ff68a189d541076f05424142450101365f5c24f5fd229a357df3578c496a26d1e11ff8a567d06eb98eb75fe49e9e55204c930c785a5dd18bacb8f06e64a442966cbae0859b55819e9bec6fc801918512f70a0a207af58ddde93cb2105b7cae84e58dfcfafa0a8e554f4a0c71a205772e6101d58d12d20a0a2d3703f5a4efb16ffa83d007000083a874f5953ebc28142cb2184f2227e8fc1a2a930665b6269dcbfd64da0b179b0a870180046480f6cbcf8517e448ae891923f1d9aebdd7a1e435a18d593074ff6cf873ccb4ac4580f6f6801e4b41e2e6d8ec194dba122bfb9eb33feb2545ef5144cea79551f7cc52802962bfbdbc4c3fee805d547f8fb08db560b03ec56721ce05f7fdffe7d6892ffa80b8497c75f80a57982ff29ed2c42c347b5158f40ab78079f506a3f0e10ad0ce630a930480ffff80a244132166cca1b9cb319f1362aa46c025824965ca24af5fbf2ca3f4a762e08c806cb588798afe499ef4fad06a5d547a4b9baf7cc471ce81cf40d6ec5743d23b6d805bfb7edd5c0eaf177acc68eeb1bb05f86803769f0c113500515551f2859e30aa80c35647d5ef63e87b7c4d9ed2b88c9b78553b1019e8c2a47b7c1e65fb973df75b80b872082faf350984dc3173561d7f7e1e37dd90fdeda6f929181bef3457632654800b9e5fde7411eb39fbe31354d1f8c092c52812c7e351ca8824bf07e42572533680ae3a3246726e8113f3dafe3f3a895d9cc0b21085974ea332ad3dca85fec21656805b682132c52908705526057f73ab7fccab4af6d72a9805634dd8d3cc53f130d180c2d44d371e5fc1f50227d7491ad65ad049630361cefb4ab1844831237609f0838033ef217f33a7bf2497bd3b532d5196363603b0a685cd5e253b1b7ef6ef3186c2800cdc2830c3697b7128e33d81118472b04783e76b01177c26cb5f29a23f5713c980330a556b0c7ee12c7ba20d775b9dc705b5b2779282d0170e384744768427f4ad802f664b4cf1449a239a94171acdb7039035923e9e5644530aa837f575f94569a1804c59925a4e422dc87a33d8d2bcca62efa7fb2fd985e6c281675e9b2df0eeec79806bfdbbf0e0bedcb993b65c9cea1e929a56d78a3b7bc53d1b7ca6fc488e2295ee80f49ac3396db69998b2428f2de31fd9ca4acff6663b4a42ea7aa3f6c1ae8355d40a95029e710b30bd2eab0352ddcc26417aa1945f43803b3441f15daa8a53147d69c48eac75356fab581febbb8030520b248c5942a14880a404a091241950948d6e2399cc5949818a2c0bab7d33c1aa2a496d1cdc2a5d72802e2e0716043a02f2f29fdd52922704af194b98545ce5ca832255e8ec41bcdb648015306063e6f316b68ee811a531146c8f64e0a490cc0fbca9b83afce9d7699799505f0e7b9012096b41c4eb3aaf947f6ea4290800004c5f03c716fb8fff3de61a883bb76adb34a2040080ca356caacc573edab93da993e39702d2cff8a467e3074b14a8b2c4db9a5f4a1b4c5f0f4993f016e2d2f8e5f43be7bb259486040080b842c13f848818d7d2869ede474bb616f1c39752fc94636aed5706e4872e00c10a559f0b3c252fcb29d88eff4f3de5de4476c3500080a735ba4368d8d5a4b0733aca7dbdb8df42ef6c48b425623e92d3aca67a2fd1a0808d61f62a0339287be1b499bfdb18bf74ae128504f3b6ac946b263cca88de73ed0ab901dd02ab28bfb62ceb4e02ae2e255a88c42a134e74b848bdc0efbe58fef1596ed8e14e54c409c1b2897ad2b437395fb17bc30a92898095e5e3d8bb2fe5c14214b4a12f5298afd7cad48bcf25e3e1b9e6f054c6c3d5dceb810b8a58bb5d0e71825b401bd908066175726120b0bf5208000000000561757261010144b9d39fe5c4f4808eebb547fab57a10a3a336e3a94a6ae0b0bf79fdf9dd964cbea565b0ae654412e01d4121b01be9ef32a4e20cf252b7201174bfc0a9917f81120b280401000b1a5ad92686011a49810011010840008064f9a32e7466b08244144e633bbb1d122b6a7e3f66555ef62baedc9146b26ee0800b9e7d3145822242b5a382127b49323d44c0b9ac1003dc93f2216ed0f7e7a246120508d00f1060"}]}]}]

 */

/// Note: This function, as well as other helpers, are needed as a workaround to
/// Rust's orphan rule. That is, we want the AbciEvent -> IbcEvent to be defined
/// in the relayer crate, but can't because neither AbciEvent nor IbcEvent are
/// defined in this crate. Hence, we are forced to make an ad-hoc function for
/// it.
pub fn ibc_event_try_from_abci_event(
	abci_event: &AbciEvent,
	height: Height,
) -> Result<IbcEvent, IbcEventError> {
	// println!("Got IBC event type: {}", abci_event.kind);

	match &abci_event.kind.parse() {
		Ok(IbcEventType::CreateClient) => Ok(IbcEvent::CreateClient(
			create_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::UpdateClient) => Ok(IbcEvent::UpdateClient(
			update_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::UpgradeClient) => Ok(IbcEvent::UpgradeClient(
			upgrade_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::ClientMisbehaviour) => Ok(IbcEvent::ClientMisbehaviour(
			client_misbehaviour_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::PushWasmCode) =>
			Ok(IbcEvent::PushWasmCode(push_wasm_code_try_from_abci_event(abci_event)?)),
		Ok(IbcEventType::OpenInitConnection) => Ok(IbcEvent::OpenInitConnection(
			connection_open_init_try_from_abci_event(abci_event)
				.map_err(IbcEventError::connection)?,
		)),
		Ok(IbcEventType::OpenTryConnection) => Ok(IbcEvent::OpenTryConnection(
			connection_open_try_try_from_abci_event(abci_event)
				.map_err(IbcEventError::connection)?,
		)),
		Ok(IbcEventType::OpenAckConnection) => Ok(IbcEvent::OpenAckConnection(
			connection_open_ack_try_from_abci_event(abci_event)
				.map_err(IbcEventError::connection)?,
		)),
		Ok(IbcEventType::OpenConfirmConnection) => Ok(IbcEvent::OpenConfirmConnection(
			connection_open_confirm_try_from_abci_event(abci_event)
				.map_err(IbcEventError::connection)?,
		)),
		Ok(IbcEventType::OpenInitChannel) => Ok(IbcEvent::OpenInitChannel(
			channel_open_init_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenTryChannel) => Ok(IbcEvent::OpenTryChannel(
			channel_open_try_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenAckChannel) => Ok(IbcEvent::OpenAckChannel(
			channel_open_ack_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenConfirmChannel) => Ok(IbcEvent::OpenConfirmChannel(
			channel_open_confirm_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::CloseInitChannel) => Ok(IbcEvent::CloseInitChannel(
			channel_close_init_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::CloseConfirmChannel) => Ok(IbcEvent::CloseConfirmChannel(
			channel_close_confirm_try_from_abci_event(abci_event)
				.map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::SendPacket) => Ok(IbcEvent::SendPacket(
			send_packet_try_from_abci_event(abci_event, height).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::WriteAck) => Ok(IbcEvent::WriteAcknowledgement(
			write_acknowledgement_try_from_abci_event(abci_event, height)
				.map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::AckPacket) => Ok(IbcEvent::AcknowledgePacket(
			acknowledge_packet_try_from_abci_event(abci_event, height)
				.map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::Timeout) => Ok(IbcEvent::TimeoutPacket(
			timeout_packet_try_from_abci_event(abci_event, height)
				.map_err(IbcEventError::channel)?,
		)),
		_ => {
			println!("IBC event type not recognized: {}", abci_event.kind);
			Err(IbcEventError::unsupported_abci_event(abci_event.kind.to_owned()))
		},
	}
}

pub fn create_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<client_events::CreateClient, ClientError> {
	client_extract_attributes_from_tx(abci_event).map(client_events::CreateClient)
}

pub fn update_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<client_events::UpdateClient, ClientError> {
	client_extract_attributes_from_tx(abci_event).map(|attributes| client_events::UpdateClient {
		common: attributes,
		header: extract_header_from_tx(abci_event)
			.ok()
			.map(|h| h.encode_vec().expect("header should encode")),
	})
}

pub fn upgrade_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<client_events::UpgradeClient, ClientError> {
	client_extract_attributes_from_tx(abci_event).map(client_events::UpgradeClient)
}

pub fn client_misbehaviour_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<client_events::ClientMisbehaviour, ClientError> {
	client_extract_attributes_from_tx(abci_event).map(client_events::ClientMisbehaviour)
}

pub fn push_wasm_code_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<client_events::PushWasmCode, IbcEventError> {
	let mut code_id = None;
	for tag in &abci_event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		match key {
			client_events::WASM_CODE_ID_ATTRIBUTE_KEY =>
				code_id = Some(hex::decode(value).map_err(IbcEventError::from_hex_error)?),
			_ => {},
		}
	}

	Ok(client_events::PushWasmCode(code_id.ok_or_else(|| {
		IbcEventError::missing_key(client_events::WASM_CODE_ID_ATTRIBUTE_KEY.to_owned())
	})?))
}

pub fn connection_open_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<connection_events::OpenInit, ConnectionError> {
	connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenInit)
}

pub fn connection_open_try_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<connection_events::OpenTry, ConnectionError> {
	connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenTry)
}

pub fn connection_open_ack_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<connection_events::OpenAck, ConnectionError> {
	connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenAck)
}

pub fn connection_open_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<connection_events::OpenConfirm, ConnectionError> {
	connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenConfirm)
}

pub fn channel_open_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::OpenInit, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::OpenInit::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn channel_open_try_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::OpenTry, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::OpenTry::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn channel_open_ack_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::OpenAck, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::OpenAck::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn channel_open_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::OpenConfirm, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::OpenConfirm::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn channel_close_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::CloseInit, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::CloseInit::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn channel_close_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<channel_events::CloseConfirm, ChannelError> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => channel_events::CloseConfirm::try_from(attrs)
			.map_err(|e| ChannelError::implementation_specific(e.to_string())),
		Err(e) => Err(e),
	}
}

pub fn send_packet_try_from_abci_event(
	abci_event: &AbciEvent,
	height: Height,
) -> Result<channel_events::SendPacket, ChannelError> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			channel_events::SendPacket { packet, height }
		})
		.map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn write_acknowledgement_try_from_abci_event(
	abci_event: &AbciEvent,
	height: Height,
) -> Result<channel_events::WriteAcknowledgement, ChannelError> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| channel_events::WriteAcknowledgement {
			packet,
			ack: write_ack,
			height,
		})
		.map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn acknowledge_packet_try_from_abci_event(
	abci_event: &AbciEvent,
	height: Height,
) -> Result<channel_events::AcknowledgePacket, ChannelError> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			channel_events::AcknowledgePacket { height, packet }
		})
		.map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn timeout_packet_try_from_abci_event(
	abci_event: &AbciEvent,
	height: Height,
) -> Result<channel_events::TimeoutPacket, ChannelError> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			channel_events::TimeoutPacket { packet, height }
		})
		.map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

fn client_extract_attributes_from_tx(event: &AbciEvent) -> Result<ClientAttributes, ClientError> {
	let mut attr = ClientAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		match key {
			client_events::CLIENT_ID_ATTRIBUTE_KEY =>
				attr.client_id = value.parse().map_err(ClientError::invalid_client_identifier)?,
			client_events::CLIENT_TYPE_ATTRIBUTE_KEY =>
				attr.client_type = value
					.parse()
					.map_err(|_| ClientError::unknown_client_type(value.to_string()))?,
			client_events::CONSENSUS_HEIGHT_ATTRIBUTE_KEY =>
				attr.consensus_height = value
					.parse()
					.map_err(|e| ClientError::invalid_string_as_height(value.to_string(), e))?,
			client_events::HEIGHT_ATTRIBUTE_KEY => attr.height = value.parse().unwrap(),
			_ => {},
		}
	}

	Ok(attr)
}

pub fn extract_header_from_tx(event: &AbciEvent) -> Result<Header, ClientError> {
	for tag in &event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		if key == HEADER_ATTRIBUTE_KEY {
			let header_bytes: Vec<u8> =
				hex::decode(value).map_err(|_| ClientError::malformed_header())?;
			return decode_header(&header_bytes)
		}
	}
	Err(ClientError::missing_raw_header())
}

fn connection_extract_attributes_from_tx(
	event: &AbciEvent,
) -> Result<ConnectionAttributes, ConnectionError> {
	let mut attr = ConnectionAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		match key {
			connection_events::CONN_ID_ATTRIBUTE_KEY => {
				attr.connection_id = value.parse().ok();
			},
			connection_events::CLIENT_ID_ATTRIBUTE_KEY => {
				attr.client_id = value.parse().map_err(ConnectionError::invalid_identifier)?;
			},
			connection_events::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY => {
				attr.counterparty_connection_id = value.parse().ok();
			},
			connection_events::COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY => {
				attr.counterparty_client_id =
					value.parse().map_err(ConnectionError::invalid_identifier)?;
			},
			connection_events::HEIGHT_ATTRIBUTE_KEY => attr.height = value.parse().unwrap(),
			_ => {},
		}
	}

	Ok(attr)
}

fn channel_extract_attributes_from_tx(
	event: &AbciEvent,
) -> Result<ChannelAttributes, ChannelError> {
	let mut attr = ChannelAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		match key {
			channel_events::PORT_ID_ATTRIBUTE_KEY =>
				attr.port_id = value.parse().map_err(ChannelError::identifier)?,
			channel_events::CHANNEL_ID_ATTRIBUTE_KEY => {
				attr.channel_id = value.parse().ok();
			},
			channel_events::CONNECTION_ID_ATTRIBUTE_KEY => {
				attr.connection_id = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY => {
				attr.counterparty_port_id = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY => {
				attr.counterparty_channel_id = value.parse().ok();
			},
			channel_events::HEIGHT_ATTRIBUTE_KEY => attr.height = value.parse().unwrap(),
			_ => {},
		}
	}

	Ok(attr)
}

fn extract_packet_and_write_ack_from_tx(
	event: &AbciEvent,
) -> Result<(Packet, Vec<u8>), ChannelError> {
	let mut packet = Packet::default();
	let mut write_ack: Vec<u8> = Vec::new();
	for tag in &event.attributes {
		let key = tag.key.as_str();
		let value = tag.value.as_str();
		match key {
			channel_events::PKT_SRC_PORT_ATTRIBUTE_KEY => {
				packet.source_port = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::PKT_SRC_CHANNEL_ATTRIBUTE_KEY => {
				packet.source_channel = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::PKT_DST_PORT_ATTRIBUTE_KEY => {
				packet.destination_port = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::PKT_DST_CHANNEL_ATTRIBUTE_KEY => {
				packet.destination_channel = value.parse().map_err(ChannelError::identifier)?;
			},
			channel_events::PKT_SEQ_ATTRIBUTE_KEY =>
				packet.sequence = value
					.parse::<u64>()
					.map_err(|e| ChannelError::invalid_string_as_sequence(value.to_string(), e))?
					.into(),
			channel_events::PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY => {
				packet.timeout_height =
					parse_timeout_height(value)?.expect("timeout height is set");
			},
			channel_events::PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY => {
				packet.timeout_timestamp = value.parse().unwrap();
			},
			channel_events::PKT_DATA_ATTRIBUTE_KEY => {
				packet.data = Vec::from(value.as_bytes());
			},
			channel_events::PKT_ACK_ATTRIBUTE_KEY => {
				write_ack = Vec::from(value.as_bytes());
			},
			_ => {},
		}
	}

	Ok((packet, write_ack))
}

/// Parse a string into a timeout height expected to be stored in
/// `Packet.timeout_height`. We need to parse the timeout height differently
/// because of a quirk introduced in ibc-go. See comment in
/// `TryFrom<RawPacket> for Packet`.
pub fn parse_timeout_height(s: &str) -> Result<TimeoutHeight, ChannelError> {
	match s.parse::<Height>() {
		Ok(height) => Ok(Some(height)),
		Err(e) => match e.into_detail() {
			HeightErrorDetail::HeightConversion(x) if x.height == "0" => Ok(None),
			_ => Err(ChannelError::invalid_timeout_height()),
		},
	}
}

/// Decodes an encoded header into a known `Header` type,
pub fn decode_header(header_bytes: &[u8]) -> Result<Header, ClientError> {
	let header = tm_decode_header(header_bytes)?;
	Ok(header)
}

// fn extract_block_events(
// 	height: Height,
// 	block_events: &HashMap<String, Vec<String>>,
// ) -> Vec<IbcEvent> {
// 	#[inline]
// 	fn extract_events<'a, T: TryFrom<RawObject<'a>>>(
// 		height: Height,
// 		block_events: &'a HashMap<String, Vec<String>>,
// 		event_type: &str,
// 		event_field: &str,
// 	) -> Vec<T> {
// 		block_events
// 			.get(&format!("{}.{}", event_type, event_field))
// 			.unwrap_or(&vec![])
// 			.iter()
// 			.enumerate()
// 			.filter_map(|(i, _)| {
// 				let raw_obj = RawObject::new(height, event_type.to_owned(), i, block_events);
// 				T::try_from(raw_obj).ok()
// 			})
// 			.collect()
// 	}

// 	#[inline]
// 	fn append_events<T: Into<IbcEvent>>(
// 		events: &mut Vec<IbcEventWithHeight>,
// 		chan_events: Vec<T>,
// 		height: Height,
// 	) {
// 		events.append(
// 			&mut chan_events
// 				.into_iter()
// 				.map(|ev| IbcEventWithHeight::new(ev.into(), height))
// 				.collect(),
// 		);
// 	}

// 	let mut events: Vec<IbcEventWithHeight> = vec![];
// 	append_events::<ChannelEvents::OpenInit>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_init", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenTry>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_try", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenAck>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_ack", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenConfirm>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_confirm", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::SendPacket>(
// 		&mut events,
// 		extract_events(height, block_events, "send_packet", "packet_data"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::CloseInit>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_close_init", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::CloseConfirm>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_close_confirm", "channel_id"),
// 		height,
// 	);
// 	events
// }
