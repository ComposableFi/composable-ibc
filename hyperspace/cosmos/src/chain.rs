use super::{client::CosmosClient, tx::sign_tx};
use crate::{error::Error, events::client_extract_attributes_from_tx, provider::FinalityEvent};
use futures::{Stream, StreamExt};
use ibc::{
	core::{
		ics02_client::{events::UpdateClient, msgs::ClientMsg},
		ics24_host::identifier::ChainId,
		ics26_routing::msgs::Ics26Envelope,
	},
	events::IbcEvent,
	Height,
};
use ibc_proto::{
	cosmos::{
		base::v1beta1::Coin,
		tx::v1beta1::{service_client::ServiceClient, Fee, GetTxsEventRequest, OrderBy},
	},
	google::protobuf::Any,
};
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::{
	mock::LocalClientTypes, Chain, IbcProvider, LightClientSync, MisbehaviourHandler,
};
use prost::Message;
use std::pin::Pin;
use tendermint_rpc::{
	event::{Event, EventData},
	query::{EventType, Query},
	SubscriptionClient, WebSocketClient,
};

#[async_trait::async_trait]
impl<H> LightClientSync for CosmosClient<H>
where
	H: 'static + Clone + Send + Sync,
{
	async fn is_synced<C: Chain>(&self, _counterparty: &C) -> Result<bool, anyhow::Error> {
		Ok(true)
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		_counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		Ok((vec![], vec![]))
	}
}

#[async_trait::async_trait]
impl<H> Chain for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_tx_size as u64
	}

	async fn estimate_weight(&self, messages: Vec<Any>) -> Result<u64, Self::Error> {
		let account_info = self.query_account().await?;
		let fee = self.get_fee();
		let (_, tx_raw, _) =
			sign_tx(self.keybase.clone(), self.chain_id.clone(), &account_info, vec![], fee)?;

		let total_len = tx_raw.encoded_len();
		let body_bytes_len = tx_raw.body_bytes.len();
		let envelope_len = if body_bytes_len == 0 {
			total_len
		} else {
			total_len - 1 - prost::length_delimiter_len(body_bytes_len) - body_bytes_len
		};
		// Full length of the transaction can then be derived from the length of the invariable
		// envelope and the length of the body field, taking into account the varint encoding
		// of the body field's length delimiter.
		fn tx_len(envelope_len: usize, body_len: usize) -> usize {
			// The caller has at least one message field length added to the body's
			debug_assert!(body_len != 0);
			envelope_len + 1 + prost::length_delimiter_len(body_len) + body_len
		}

		let mut current_count = 0;
		let mut current_len = body_bytes_len;

		for message in messages {
			let message_len = message.encoded_len();

			// The total length the message adds to the encoding includes the
			// field tag (small varint) and the length delimiter.
			let tagged_len = 1 + prost::length_delimiter_len(message_len) + message_len;
			if current_count >= 30 ||
				tx_len(envelope_len, current_len + tagged_len) > self.max_tx_size
			{
				return Err(Error::Custom("Too many messages".to_string()))
			}
			current_count += 1;
			current_len += tagged_len;
		}
		Ok(current_len as u64)
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		let (ws_client, ws_driver) = WebSocketClient::new(self.websocket_url.clone())
			.await
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
			.unwrap();
		tokio::spawn(ws_driver.run());
		let subscription = ws_client
			.subscribe(Query::from(EventType::NewBlock))
			.await
			.map_err(|e| Error::from(format!("failed to subscribe to new blocks {:?}", e)))
			.unwrap()
			.chunks(6);
		log::info!(target: "hyperspace_cosmos", "🛰️ Subscribed to {} listening to finality notifications", self.name);
		let stream = subscription.filter_map(|events| {
			let events = events
				.into_iter()
				.collect::<Result<Vec<_>, _>>()
				.map_err(|e| Error::from(format!("failed to get event {:?}", e)))
				.unwrap();
			let get_height = |event: &Event| {
				let Event { data, events: _, query: _ } = &event;
				let height = match &data {
					EventData::NewBlock { block, .. } =>
						block
							.as_ref()
							.expect("NewBlock event should always have a block; qed")
							.header
							.height,
					_ => unreachable!(),
				};
				height
			};
			futures::future::ready(Some(FinalityEvent::Tendermint {
				from: get_height(events.first().unwrap()),
				to: get_height(events.last().unwrap()),
			}))
		});

		Box::pin(stream)
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		let hash = self.submit_call(messages).await?;
		log::debug!(target: "hyperspace_cosmos", "Submitted. Tx hash: {}", hash);
		Ok(Self::TransactionId { hash })
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		let query_str = Query::eq("update_client.client_id", update.client_id().to_string())
			.and_eq("update_client.client_type", update.client_type())
			.and_eq("update_client.consensus_heights", update.consensus_height().to_string());
		// omit this field since the first three should be enough to identify the update
		// .and_eq("update_client.header", hex::encode(&update.header.unwrap_or_default()))
		use tendermint::abci::Event as AbciEvent;

		let mut client = ServiceClient::connect(self.grpc_url.to_string())
			.await
			.map_err(|e| Error::from(e.to_string()))?;
		let mut resp = client
			.get_txs_event(GetTxsEventRequest {
				events: query_str
					.conditions
					.into_iter()
					.map(|c| c.to_string().replace(" = ", "="))
					.collect(),
				order_by: OrderBy::Desc.into(),
				page: 1,
				limit: 1,
				..Default::default()
			})
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();
		let mut idx = None;
		'l: for l in resp
			.tx_responses
			.pop()
			.ok_or_else(|| {
				Error::from(format!("Failed to find tx for update client: {:?}", update))
			})?
			.logs
		{
			for ev in l.events {
				let e = AbciEvent {
					kind: ev.r#type,
					attributes: ev
						.attributes
						.into_iter()
						.map(|a| tendermint::abci::EventAttribute {
							key: a.key,
							value: a.value,
							index: false,
						})
						.collect(),
				};
				let attr = client_extract_attributes_from_tx(&e).map_err(|e| {
					Error::from(format!("Failed to extract attributes from tx: {}", e))
				})?;
				if attr.client_id == *update.client_id() &&
					attr.client_type == update.client_type() &&
					attr.consensus_height == update.consensus_height()
				{
					idx = Some(l.msg_index);
					break 'l
				}
			}
		}
		let idx =
			idx.ok_or(Error::from("Failed to find matching update client event".to_string()))?;
		let x = resp
			.txs
			.pop()
			.ok_or_else(|| {
				Error::from(format!("Failed to find tx for update client in `txs`: {:?}", update))
			})?
			.body
			.ok_or_else(|| {
				Error::from(format!("Failed to find tx for update client in `body`: {:?}", update))
			})?
			.messages
			.remove(idx as usize);
		let envelope = Ics26Envelope::<LocalClientTypes>::try_from(x);
		match envelope {
			Ok(Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(update_msg))) =>
				return Ok(update_msg.client_message),
			_ => (),
		}

		Err(Error::from("Failed to find matching update client event".to_string()))
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height.increment()
	}
}

impl<H> CosmosClient<H>
where
	H: 'static + Clone + Send + Sync,
{
	pub fn get_fee(&self) -> Fee {
		Fee {
			amount: vec![Coin { denom: self.fee_denom.clone(), amount: self.fee_amount.clone() }],
			gas_limit: self.gas_limit,
			payer: "".to_string(),
			granter: "".to_string(),
		}
	}

	pub fn id(&self) -> &ChainId {
		&self.chain_id
	}
}

#[async_trait::async_trait]
impl<H> MisbehaviourHandler for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		_counterparty: &C,
		_client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		Ok(())
	}
}
