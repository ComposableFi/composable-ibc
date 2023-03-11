#[macro_export]
macro_rules! define_id {
	(
		$name: ident,
		$id_type: path
	) => {
		pub struct $name($id_type);

		impl From<u32> for $name {
			fn from(value: u32) -> Self {
				$name($id_type(value))
			}
		}
	};
}

#[macro_export]
macro_rules! define_head_data {
	(
		$name: ident,
		$head_data_type: ty,
	) => {
		#[derive(Decode)]
		pub struct $name($head_data_type);

		impl AsRef<[u8]> for $name {
			fn as_ref(&self) -> &[u8] {
				self.0 .0.as_ref()
			}
		}
	};
}

#[macro_export]
macro_rules! define_pallet_params {
	(
		$name: ident,
		$pallet_params_type: ty,
		$raw_pallet_params_type: ty
	) => {
		pub struct $name($pallet_params_type);

		impl From<$name> for $raw_pallet_params_type {
			fn from(value: $name) -> Self {
				let params = value.0;
				Self { send_enabled: params.send_enabled, receive_enabled: params.receive_enabled }
			}
		}
	};
}

#[macro_export]
macro_rules! define_any_wrapper {
	(
		$name: ident,
		$raw_any_type: ty
	) => {
		pub struct $name($raw_any_type);

		impl From<$name> for Any {
			fn from(value: $name) -> Self {
				Any {
					type_url: String::from_utf8(value.0.type_url).expect("valid string"),
					value: value.0.value,
				}
			}
		}
	};
}

#[macro_export]
macro_rules! define_ibc_event_wrapper {
	(
		$name: ident,
		$meta_ibc_event_type: ty
	) => {
		pub struct $name(pub $meta_ibc_event_type);

		impl From<$name> for RawIbcEvent {
			fn from(event: $name) -> Self {
				let event = event.0;
				match event {
					MetadataIbcEvent::NewBlock { revision_height, revision_number } =>
						RawIbcEvent::NewBlock { revision_height, revision_number },
					MetadataIbcEvent::OpenInitConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					} => RawIbcEvent::OpenInitConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					},
					MetadataIbcEvent::OpenTryConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					} => RawIbcEvent::OpenTryConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					},
					MetadataIbcEvent::OpenAckConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					} => RawIbcEvent::OpenAckConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					},
					MetadataIbcEvent::OpenConfirmConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					} => RawIbcEvent::OpenConfirmConnection {
						revision_height,
						revision_number,
						connection_id,
						counterparty_connection_id,
						client_id,
						counterparty_client_id,
					},
					MetadataIbcEvent::OpenInitChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::OpenInitChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::OpenTryChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::OpenTryChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::OpenAckChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::OpenAckChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::OpenConfirmChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::OpenConfirmChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::CloseInitChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::CloseInitChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::CloseConfirmChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					} => RawIbcEvent::CloseConfirmChannel {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						connection_id,
						counterparty_port_id,
						counterparty_channel_id,
					},
					MetadataIbcEvent::SendPacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					} => RawIbcEvent::SendPacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					},
					MetadataIbcEvent::WriteAcknowledgement {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					} => RawIbcEvent::WriteAcknowledgement {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					},
					MetadataIbcEvent::TimeoutPacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					} => RawIbcEvent::TimeoutPacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					},
					MetadataIbcEvent::TimeoutOnClosePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					} => RawIbcEvent::TimeoutOnClosePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					},
					MetadataIbcEvent::CreateClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					} => RawIbcEvent::CreateClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					},
					MetadataIbcEvent::UpdateClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					} => RawIbcEvent::UpdateClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					},
					MetadataIbcEvent::UpgradeClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					} => RawIbcEvent::UpgradeClient {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					},
					MetadataIbcEvent::ClientMisbehaviour {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					} => RawIbcEvent::ClientMisbehaviour {
						client_id,
						client_type,
						revision_height,
						revision_number,
						consensus_height,
						consensus_revision_number,
					},
					MetadataIbcEvent::ReceivePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					} => RawIbcEvent::ReceivePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						dest_port,
						dest_channel,
						sequence,
					},
					MetadataIbcEvent::AcknowledgePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					} => RawIbcEvent::AcknowledgePacket {
						revision_height,
						revision_number,
						port_id,
						channel_id,
						sequence,
					},
					MetadataIbcEvent::AppModule { kind, module_id } =>
						RawIbcEvent::AppModule { kind, module_id },
					MetadataIbcEvent::Empty => RawIbcEvent::Empty,
					MetadataIbcEvent::ChainError => RawIbcEvent::ChainError,
				}
			}
		}
	};
}

#[macro_export]
macro_rules! define_send_ping_params {
	(
		$name: ident,
		$send_ping_params_type: ty,
		$raw_send_ping_params_type: ty,
	) => {
		pub struct $name($send_ping_params_type);

		impl From<$name> for $raw_send_ping_params_type {
			fn from(value: $name) -> Self {
				let params = value.0;
				Self {
					data: params.data,
					timeout_height_offset: params.timeout_height_offset,
					timeout_timestamp_offset: params.timeout_timestamp_offset,
					channel_id: params.channel_id,
				}
			}
		}
	};
}

#[macro_export]
macro_rules! define_transfer_params {
	(
		$name: ident,
		$transfer_params_type: ty,
		$raw_transfer_params_type: ty,
		$timeout_type: ty,
		$address_type: ty,
	) => {
		pub struct $name($transfer_params_type);

		impl<T> From<$name> for $raw_transfer_params_type
		where
			T: From<[u8; 32]>,
		{
			fn from(value: $name) -> Self {
				let params = value.0;
				Self {
					to: match params.to {
						MultiAddress::Id(id) => {
							let id: [u8; 32] = id.into();
							<$address_type>::Id(id.into())
						},
						MultiAddress::Raw(raw) => <$address_type>::Raw(raw),
					},

					source_channel: params.source_channel,
					timeout: match params.timeout {
						Timeout::Offset { timestamp, height } =>
							RawTimeout::Offset { timestamp, height },
						Timeout::Absolute { timestamp, height } =>
							RawTimeout::Absolute { timestamp, height },
					},
				}
			}
		}
	};
}
