use crate::{
	client::{ClientError, EthereumClient},
	ibc_provider::{
		CloseConfirmChannelFilter, CreateClientFilter, OpenConfirmChannelFilter,
		OpenConfirmConnectionFilter, UpdateClientFilter,
	},
};
use async_trait::async_trait;
use ethers::prelude::Log;
use ibc::{
	core::{
		ics02_client::events::{Attributes as ClientAttributes, CreateClient, UpdateClient},
		ics03_connection::events::{Attributes, OpenConfirm as ConnectionOpenConfirm},
		ics04_channel::events::{CloseConfirm, OpenConfirm as ChannelOpenConfirm},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
	Height,
};
use primitives::IbcProvider;

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

#[async_trait]
impl TryFromEvent<OpenConfirmConnectionFilter> for IbcEvent {
	async fn try_from_event(
		client: &EthereumClient,
		event: OpenConfirmConnectionFilter,
		_log: Log,
		height: Height,
	) -> Result<Self, ClientError> {
		let OpenConfirmConnectionFilter { connection_id } = event;
		let connection_id: ConnectionId = connection_id.parse()?;
		let resp = client.query_connection_end(height, connection_id.clone()).await?;
		let counterparty = resp.connection.unwrap().counterparty.unwrap();
		Ok(IbcEvent::OpenConfirmConnection(ConnectionOpenConfirm(Attributes {
			height,
			connection_id: Some(connection_id),
			client_id: client.client_id(),
			counterparty_connection_id: Some(counterparty.connection_id.parse()?),
			counterparty_client_id: counterparty.client_id.parse()?,
		})))
	}
}

#[async_trait]
impl TryFromEvent<OpenConfirmChannelFilter> for IbcEvent {
	async fn try_from_event(
		client: &EthereumClient,
		event: OpenConfirmChannelFilter,
		_log: Log,
		height: Height,
	) -> Result<Self, ClientError> {
		let OpenConfirmChannelFilter { port_id, channel_id } = event;
		let port_id: PortId = port_id.parse()?;
		let channel_id: ChannelId = channel_id.parse()?;
		let resp = client.query_channel_end(height, channel_id, port_id.clone()).await?;
		let channel = resp.channel.unwrap();
		let counterparty = channel.counterparty.unwrap();
		Ok(IbcEvent::OpenConfirmChannel(ChannelOpenConfirm {
			height,
			channel_id: Some(channel_id),
			connection_id: channel.connection_hops[0].parse()?,
			counterparty_port_id: counterparty.port_id.parse()?,
			port_id,
			counterparty_channel_id: Some(counterparty.port_id.parse()?),
		}))
	}
}
