use crate::{
	client::{ClientError, EthereumClient},
	ibc_provider::{CreateClientFilter, UpdateClientFilter},
};
use async_trait::async_trait;
use ethers::prelude::Log;
use ibc::{
	core::ics02_client::events::{Attributes as ClientAttributes, CreateClient, UpdateClient},
	events::IbcEvent,
	Height,
};

#[async_trait]
pub trait TryFromEvent<T>
where
	Self: Sized,
{
	async fn try_from_event(
		client: &EthereumClient,
		event: T,
		log: Log,
		height: Height,
	) -> Result<Self, ClientError>;
}

/*
   NewBlock(NewBlock),

   CreateClient(ClientEvents::CreateClient),
   UpdateClient(ClientEvents::UpdateClient),
   UpgradeClient(ClientEvents::UpgradeClient),
   ClientMisbehaviour(ClientEvents::ClientMisbehaviour),
   PushWasmCode(ClientEvents::PushWasmCode),

   OpenInitConnection(ConnectionEvents::OpenInit),
   OpenTryConnection(ConnectionEvents::OpenTry),
   OpenAckConnection(ConnectionEvents::OpenAck),
   OpenConfirmConnection(ConnectionEvents::OpenConfirm),

   OpenInitChannel(ChannelEvents::OpenInit),
   OpenTryChannel(ChannelEvents::OpenTry),
   OpenAckChannel(ChannelEvents::OpenAck),
   OpenConfirmChannel(ChannelEvents::OpenConfirm),
   CloseInitChannel(ChannelEvents::CloseInit),
   CloseConfirmChannel(ChannelEvents::CloseConfirm),

   SendPacket(ChannelEvents::SendPacket),
   ReceivePacket(ChannelEvents::ReceivePacket),
   WriteAcknowledgement(ChannelEvents::WriteAcknowledgement),
   AcknowledgePacket(ChannelEvents::AcknowledgePacket),
   TimeoutPacket(ChannelEvents::TimeoutPacket),
   TimeoutOnClosePacket(ChannelEvents::TimeoutOnClosePacket),

   AppModule(ModuleEvent),

   Empty(String),      // Special event, signifying empty response
   ChainError(String), // Special event, signifying an error on CheckTx or DeliverTx

*/

#[async_trait]
impl TryFromEvent<CreateClientFilter> for IbcEvent {
	async fn try_from_event(
		client: &EthereumClient,
		event: CreateClientFilter,
		log: Log,
		height: Height,
	) -> Result<Self, ClientError> {
		let CreateClientFilter { client_id, client_type } = event;
		Ok(IbcEvent::CreateClient(CreateClient(ClientAttributes {
			height,
			client_id: client_id.parse()?,
			client_type,
			consensus_height: Default::default(), // TODO: consensus height?
		})))
	}
}

#[async_trait]
impl TryFromEvent<UpdateClientFilter> for IbcEvent {
	async fn try_from_event(
		client: &EthereumClient,
		event: UpdateClientFilter,
		log: Log,
		height: Height,
	) -> Result<Self, ClientError> {
		let UpdateClientFilter { client_id, header } = event;
		Ok(IbcEvent::UpdateClient(UpdateClient {
			common: ClientAttributes {
				height,
				client_id: client_id.parse()?,
				client_type: Default::default(),      // TODO: client type?
				consensus_height: Default::default(), // TODO: consensus height?
			},
			header: Some(header.to_vec()), // TODO: header query
		}))
	}
}
