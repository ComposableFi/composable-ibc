use anchor_client::{
	solana_client::{
		nonblocking::rpc_client::RpcClient as AsyncRpcClient, rpc_config::RpcSendTransactionConfig,
	},
	solana_sdk::{
		commitment_config::{CommitmentConfig, CommitmentLevel},
		compute_budget::ComputeBudgetInstruction,
		signature::Keypair,
		signer::Signer as AnchorSigner,
	},
	Client as AnchorClient, Cluster, Program,
};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::associated_token::get_associated_token_address;
use core::{str::FromStr, time::Duration};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::{
		ics02_client::client_state::ClientType,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
};
use ibc_new::core::host::types::identifiers::ClientId as ClientIdNew;
use lib::hash::CryptoHash;
use primitives::{CommonClientConfig, CommonClientState, IbcProvider};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashSet,
	ops::Deref,
	result::Result,
	sync::{Arc, Mutex},
	thread::sleep,
};
use tendermint_light_client_verifier_new::types::{TrustedBlockState, UntrustedBlockState};
use tendermint_rpc::Url;

use super::{CHAIN_SEED, SOLANA_IBC_STORAGE_SEED, TRIE_SEED};
use crate::{
	error::Error,
	utils::{new_ed25519_instruction_with_signature, non_absent_vote},
};
use solana_ibc::{chain::ChainData, ix_data_account, storage::PrivateStorage};
use tendermint_new::vote::{SignedVote, ValidatorIndex};

pub enum DeliverIxType {
	UpdateClient {
		client_message: ibc_proto_new::google::protobuf::Any,
		client_id: ClientIdNew,
	},
	PacketTransfer {
		denom: String,
		port_id: ibc_new::core::host::types::identifiers::PortId,
		channel_id: ibc_new::core::host::types::identifiers::ChannelId,
	},
	Normal,
}

/// Implements the [`crate::Chain`] trait for solana
#[derive(Clone)]
pub struct SolanaClient {
	/// Chain name
	pub name: String,
	/// rpc url for solana
	pub rpc_url: String,
	/// websocket url for solana
	pub ws_url: String,
	/// Solana chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Account prefix
	pub account_prefix: String,
	pub fee_denom: String,
	/// The key that signs transactions
	pub keybase: KeyEntry,
	/// Maximun transaction size
	pub max_tx_size: usize,
	pub commitment_level: CommitmentLevel,
	pub program_id: Pubkey,
	pub common_state: CommonClientState,
	pub client_type: ClientType,
	pub last_searched_sig_for_send_packets: Arc<tokio::sync::Mutex<String>>,
	pub last_searched_sig_for_recv_packets: Arc<tokio::sync::Mutex<String>>,
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
	/// Channels cleared for packet relay
	pub channel_whitelist: Arc<Mutex<HashSet<(ChannelId, PortId)>>>,
}

#[derive(std::fmt::Debug, Serialize, Deserialize, Clone)]
pub struct SolanaClientConfig {
	/// Chain name
	pub name: String,
	/// rpc url for solana
	pub rpc_url: Url,
	/// websocket url for solana
	pub ws_url: Url,
	/// Solana chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Account prefix
	pub account_prefix: String,
	/// Fee denom
	pub fee_denom: String,
	/// Fee amount
	pub fee_amount: String,
	/// Fee amount
	pub gas_limit: u64,
	/// Store prefix
	pub store_prefix: String,
	/// Maximun transaction size
	pub max_tx_size: usize,
	/// All the client states and headers will be wrapped in WASM ones using the WASM code ID.
	pub wasm_code_id: Option<String>,
	pub common_state_config: CommonClientConfig,
	/// Reference to commitment
	pub commitment_prefix: Vec<u8>,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	pub commitment_level: String,
	pub private_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum FinalityEvent {
	Tendermint {
		previous_blockhash: String,
		blockhash: String,
		height: u64,
		timestamp: u64,
		slot: u64,
	},
}

#[derive(Clone)]
pub struct KeyEntry {
	pub public_key: Pubkey,
	pub private_key: Vec<u8>,
}

impl KeyEntry {
	pub fn keypair(&self) -> Keypair {
		Keypair::from_bytes(&self.private_key).unwrap()
	}
}

impl From<Vec<u8>> for KeyEntry {
	fn from(value: Vec<u8>) -> Self {
		let keypair = Keypair::from_bytes(&value).unwrap();
		Self { public_key: keypair.pubkey(), private_key: value }
	}
}

impl SolanaClient {
	pub fn get_trie_key(&self) -> Pubkey {
		let trie_seeds = &[TRIE_SEED];
		let trie = Pubkey::find_program_address(trie_seeds, &self.program_id).0;
		trie
	}

	pub fn get_ibc_storage_key(&self) -> Pubkey {
		let storage_seeds = &[SOLANA_IBC_STORAGE_SEED];
		let ibc_storage = Pubkey::find_program_address(storage_seeds, &self.program_id).0;
		ibc_storage
	}

	pub fn get_chain_key(&self) -> Pubkey {
		let chain_seeds = &[CHAIN_SEED];
		let chain = Pubkey::find_program_address(chain_seeds, &self.program_id).0;
		chain
	}

	pub fn get_mint_auth_key(&self) -> Pubkey {
		let mint_auth_seeds = &[solana_ibc::MINT_ESCROW_SEED];
		let mint_auth = Pubkey::find_program_address(mint_auth_seeds, &self.program_id).0;
		mint_auth
	}

	pub async fn get_trie(&self) -> solana_trie::TrieAccount<Vec<u8>> {
		let trie_key = self.get_trie_key();
		let rpc_client = self.rpc_client();
		let trie_account = rpc_client
			.get_account_with_commitment(&trie_key, CommitmentConfig::processed())
			.await
			.unwrap()
			.value
			.unwrap();
		let trie = solana_trie::TrieAccount::new(trie_account.data).unwrap();
		trie
	}

	pub async fn get_ibc_storage(&self) -> PrivateStorage {
		let program = self.program();
		let ibc_storage_key = self.get_ibc_storage_key();
		let storage: PrivateStorage = program.account(ibc_storage_key).await.unwrap();
		// let storage = tokio::task::spawn_blocking(move || {
		// 	program.account(ibc_storage_key).unwrap()
		// }).await.unwrap();
		storage
	}

	pub async fn get_chain_storage(&self) -> ChainData {
		let program = self.program();
		let chain_storage_key = self.get_chain_key();
		let storage = program.account(chain_storage_key).await.unwrap();
		storage
	}

	pub fn rpc_client(&self) -> AsyncRpcClient {
		let program = self.program();
		program.async_rpc()
	}

	pub fn client(&self) -> AnchorClient<Arc<Keypair>> {
		let cluster = Cluster::from_str(&self.rpc_url).unwrap();
		let signer = self.keybase.keypair();
		let authority = Arc::new(signer);
		let client =
			AnchorClient::new_with_options(cluster, authority, CommitmentConfig::processed());
		client
	}

	pub fn program(&self) -> Program<Arc<Keypair>> {
		let anchor_client = self.client();
		anchor_client.program(self.program_id).unwrap()
	}

	pub fn new(config: SolanaClientConfig) -> Result<Self, Error> {
		Ok(Self {
			name: config.name,
			rpc_url: config.rpc_url.to_string(),
			ws_url: config.ws_url.to_string(),
			chain_id: config.chain_id,
			client_id: config.client_id,
			connection_id: config.connection_id,
			account_prefix: config.account_prefix,
			fee_denom: config.fee_denom,
			keybase: config.private_key.into(),
			max_tx_size: config.max_tx_size,
			commitment_level: CommitmentLevel::from_str(&config.commitment_level).unwrap(),
			program_id: solana_ibc::ID,
			common_state: CommonClientState::default(),
			client_type: "07-tendermint".to_string(),
			last_searched_sig_for_send_packets: Arc::new(
				tokio::sync::Mutex::new(String::default()),
			),
			last_searched_sig_for_recv_packets: Arc::new(
				tokio::sync::Mutex::new(String::default()),
			),
			commitment_prefix: CommitmentPrefix::try_from(config.commitment_prefix).unwrap(),
			channel_whitelist: Arc::new(Mutex::new(config.channel_whitelist.into_iter().collect())),
		})
	}

	pub async fn send_deliver(
		&self,
		instruction_type: DeliverIxType,

		chunk_account: Pubkey,
		max_tries: u8,
	) -> Result<<SolanaClient as IbcProvider>::TransactionId, Error> {
		let program = self.program();
		let signer = self.keybase.keypair();
		let authority = Arc::new(signer);
		let rpc = self.rpc_client();
		let mut tries = 0;
		let mut signature = String::new();
		let solana_ibc_storage_key = self.get_ibc_storage_key();
		let trie_key = self.get_trie_key();
		let chain_key = self.get_chain_key();
		while tries < max_tries {
			println!("Try For Tx: {}", tries);
			let mut status = true;
			let sig = match instruction_type {
				DeliverIxType::UpdateClient { ref client_message, ref client_id } => {
					let header = ibc_new::clients::tendermint::types::Header::try_from(
						client_message.clone(),
					)
					.unwrap();
					let trusted_state = {
						let storage = self.get_ibc_storage().await;
						log::info!("This is client ID {:?}", client_id);
						let client_store = storage
							.clients
							.iter()
							.find(|&client| client.client_id.as_str() == client_id.as_str())
							.ok_or("Client not found with the given client id while sending update client message".to_owned())
							.unwrap();
						let serialized_consensus_state = client_store
							.consensus_states
							.deref()
							.get(
								&ibc_new::core::client::types::Height::new(
									header.trusted_height.revision_number(),
									header.trusted_height.revision_height(),
								)
								.unwrap(),
							)
							.ok_or(Error::Custom("No value at given key".to_owned()))
							.unwrap();
						let consensus_state = serialized_consensus_state
							.state()
							.map_err(|_| {
								Error::Custom(
									"Could not
deserialize consensus state"
										.to_owned(),
								)
							})
							.unwrap();
						let trusted_consensus_state = match consensus_state {
							solana_ibc::consensus_state::AnyConsensusState::Tendermint(e) => e,
							_ => panic!(),
						};

						header
							.check_trusted_next_validator_set(trusted_consensus_state.inner())
							.unwrap();

						TrustedBlockState {
							chain_id: &self.chain_id.to_string().try_into().unwrap(),
							header_time: trusted_consensus_state.timestamp(),
							height: header.trusted_height.revision_height().try_into().unwrap(),
							next_validators: &header.trusted_next_validator_set,
							next_validators_hash: trusted_consensus_state.next_validators_hash(),
						}
					};

					let untrusted_state = UntrustedBlockState {
						signed_header: &header.signed_header,
						validators: &header.validator_set,
						// NB: This will skip the
						// VerificationPredicates::next_validators_match check for the
						// untrusted state.
						next_validators: None,
					};
					let signed_header = untrusted_state.signed_header;
					let validator_set = trusted_state.next_validators;
					let signatures = &signed_header.commit.signatures;
					log::info!("These are signatures {:?}", signatures);

					let mut seen_validators = HashSet::new();

					// Get non-absent votes from the signatures
					let non_absent_votes =
						signatures.iter().enumerate().flat_map(|(idx, signature)| {
							non_absent_vote(
								signature,
								ValidatorIndex::try_from(idx).unwrap(),
								&signed_header.commit,
							)
							.map(|vote| (signature, vote))
						});
					let mut pubkeys = Vec::new();
					let mut final_signatures = Vec::new();
					let mut messages = Vec::new();
					for (_signature, vote) in non_absent_votes {
						// Ensure we only count a validator's power once
						if seen_validators.contains(&vote.validator_address) {
							// return Err(VerificationError::duplicate_validator(
							// 	vote.validator_address,
							// ))
							panic!("Duplicate validator");
						} else {
							seen_validators.insert(vote.validator_address);
						}

						let validator = match validator_set.validator(vote.validator_address) {
							Some(validator) => validator,
							None => continue, // Cannot find matching validator, so we skip the vote
						};

						let signed_vote = SignedVote::from_vote(
							vote.clone(),
							signed_header.header.chain_id.clone(),
						)
						.unwrap();

						// Check vote is valid
						let sign_bytes = signed_vote.sign_bytes();
						pubkeys.push(validator.pub_key.to_bytes());
						final_signatures.push(signed_vote.signature().clone().into_bytes());
						messages.push(sign_bytes);
						// if validator
						// 	.verify_signature::<V>(&sign_bytes, signed_vote.signature())
						// 	.is_err()
						// {
						// 	panic!("invalid signature");
						// 	// return Err(VerificationError::invalid_signature(
						// 	// 	signed_vote.signature().as_bytes().to_vec(),
						// 	// 	Box::new(validator),
						// 	// 	sign_bytes,
						// 	// ))
						// }

						// TODO: Break out of the loop when we have enough voting power.
						// See https://github.com/informalsystems/tendermint-rs/issues/235
					}
					program
						.request()
						.instruction(ComputeBudgetInstruction::set_compute_unit_limit(1_000_000u32))
						.instruction(ComputeBudgetInstruction::request_heap_frame(128 * 1024))
						.instruction(ComputeBudgetInstruction::set_compute_unit_price(500000))
						.instruction(new_ed25519_instruction_with_signature(
							pubkeys,
							final_signatures,
							messages,
						))
						.accounts(solana_ibc::accounts::Deliver {
							sender: authority.pubkey(),
							receiver: None,
							storage: solana_ibc_storage_key,
							trie: trie_key,
							chain: chain_key,
							system_program: system_program::ID,
							mint_authority: None,
							token_mint: None,
							escrow_account: None,
							receiver_token_account: None,
							associated_token_program: None,
							token_program: None,
						})
						.accounts(vec![
							AccountMeta {
								pubkey: anchor_lang::solana_program::sysvar::instructions::ID,
								is_signer: false,
								is_writable: true,
							},
							AccountMeta {
								pubkey: chunk_account,
								is_signer: false,
								is_writable: true,
							},
						])
						.args(ix_data_account::Instruction)
						.signer(&*authority)
						.send_with_spinner_and_config(RpcSendTransactionConfig {
							skip_preflight: true,
							..RpcSendTransactionConfig::default()
						})
						.await
						.or_else(|e| {
							println!("This is error {:?}", e);
							status = false;
							ibc::prelude::Err("Error".to_owned())
						})
				},
				DeliverIxType::PacketTransfer { ref denom, ref port_id, ref channel_id } => {
					let mint_authority = self.get_mint_auth_key();
					let hashed_denom = CryptoHash::digest(&denom.as_bytes());
					let escrow_seeds =
						[port_id.as_bytes(), channel_id.as_bytes(), hashed_denom.as_ref()];
					let escrow_account =
						Pubkey::find_program_address(&escrow_seeds, &self.program_id).0;
					let token_mint_seeds = [hashed_denom.as_ref()];
					let token_mint =
						Pubkey::find_program_address(&token_mint_seeds, &self.program_id).0;
					// Check if token exists
					let token_mint_info = rpc.get_token_supply(&token_mint).await;
					if token_mint_info.is_err() {
						// Create token Mint token since token doesnt exist
						let tx = program
							.request()
							.instruction(ComputeBudgetInstruction::set_compute_unit_limit(
								1_000_000u32,
							))
							.instruction(ComputeBudgetInstruction::set_compute_unit_price(500000))
							.accounts(solana_ibc::accounts::InitMint {
								sender: authority.pubkey(),
								mint_authority,
								token_mint,
								associated_token_program: anchor_spl::associated_token::ID,
								token_program: anchor_spl::token::ID,
								system_program: system_program::ID,
							})
							.args(solana_ibc::instruction::InitMint {
								port_id: port_id.clone(),
								channel_id_on_b: channel_id.clone(),
								hashed_base_denom: hashed_denom.clone(),
							})
							.signer(&*authority)
							.send_with_spinner_and_config(RpcSendTransactionConfig {
								skip_preflight: true,
								..RpcSendTransactionConfig::default()
							})
							.await
							.or_else(|e| {
								println!("This is error {:?}", e);
								status = false;
								ibc::prelude::Err("Error".to_owned())
							});
						if status {
							let blockhash = rpc.get_latest_blockhash().await.unwrap();
							// Wait for finalizing the transaction
							let _ = rpc
								.confirm_transaction_with_spinner(
									&tx.clone().unwrap(),
									&blockhash,
									CommitmentConfig::finalized(),
								)
								.await
								.unwrap();
						}
					}
					if !status {
						continue
					}
					let receiver_token_account =
						get_associated_token_address(&authority.pubkey(), &token_mint);
					program
						.request()
						.instruction(ComputeBudgetInstruction::set_compute_unit_limit(1_000_000u32))
						.instruction(ComputeBudgetInstruction::request_heap_frame(128 * 1024))
						.instruction(ComputeBudgetInstruction::set_compute_unit_price(500000))
						.accounts(solana_ibc::ix_data_account::Accounts::new(
							solana_ibc::accounts::Deliver {
								sender: authority.pubkey(),
								receiver: Some(authority.pubkey()),
								storage: solana_ibc_storage_key,
								trie: trie_key,
								chain: chain_key,
								system_program: system_program::ID,
								mint_authority: Some(mint_authority),
								token_mint: Some(token_mint),
								escrow_account: None,
								receiver_token_account: Some(receiver_token_account),
								associated_token_program: Some(anchor_spl::associated_token::ID),
								token_program: Some(anchor_spl::token::ID),
							},
							chunk_account,
						))
						.args(ix_data_account::Instruction)
						.signer(&*authority)
						.send_with_spinner_and_config(RpcSendTransactionConfig {
							skip_preflight: true,
							..RpcSendTransactionConfig::default()
						})
						.await
						.or_else(|e| {
							println!("This is error {:?}", e);
							status = false;
							ibc::prelude::Err("Error".to_owned())
						})
				},
				DeliverIxType::Normal => program
					.request()
					.instruction(ComputeBudgetInstruction::set_compute_unit_limit(1_000_000u32))
					.instruction(ComputeBudgetInstruction::request_heap_frame(128 * 1024))
					.instruction(ComputeBudgetInstruction::set_compute_unit_price(500000))
					.accounts(solana_ibc::ix_data_account::Accounts::new(
						solana_ibc::accounts::Deliver {
							sender: authority.pubkey(),
							receiver: None,
							storage: solana_ibc_storage_key,
							trie: trie_key,
							chain: chain_key,
							system_program: system_program::ID,
							mint_authority: None,
							token_mint: None,
							escrow_account: None,
							receiver_token_account: None,
							associated_token_program: None,
							token_program: None,
						},
						chunk_account,
					))
					.args(ix_data_account::Instruction)
					.signer(&*authority)
					.send_with_spinner_and_config(RpcSendTransactionConfig {
						skip_preflight: true,
						..RpcSendTransactionConfig::default()
					})
					.await
					.or_else(|e| {
						println!("This is error {:?}", e);
						status = false;
						ibc::prelude::Err("Error".to_owned())
					}),
			};

			if status {
				let blockhash = rpc.get_latest_blockhash().await.unwrap();
				// Wait for finalizing the transaction
				let _ = rpc
					.confirm_transaction_with_spinner(
						&sig.clone().unwrap(),
						&blockhash,
						CommitmentConfig::finalized(),
					)
					.await
					.unwrap();
				signature = sig.unwrap().to_string();
				break
			}
			sleep(Duration::from_millis(500));
			tries += 1;
		}
		if tries == max_tries {
			panic!("Max retries reached for normal tx in solana");
		}
		Ok(signature)
	}

	pub async fn send_transfer_inner(
		&self,
		msg: MsgTransfer<PrefixedCoin>,
	) -> Result<<SolanaClient as IbcProvider>::TransactionId, Error> {
		let keypair = self.keybase.keypair();
		println!("submitting tx now, {}", keypair.pubkey());
		let authority = Arc::new(keypair);
		let program = self.program();

		// Build, sign, and send program instruction
		let solana_ibc_storage_key = self.get_ibc_storage_key();
		let trie_key = self.get_trie_key();
		let chain_key = self.get_chain_key();

		let mint_authority = self.get_mint_auth_key();

		let channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(msg.source_channel.sequence());
		let port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(msg.source_port.as_str())
				.unwrap();
		let prefixed_denom = ibc_new::apps::transfer::types::PrefixedDenom {
			trace_path: ibc_new::apps::transfer::types::TracePath::default(),
			base_denom: ibc_new::apps::transfer::types::BaseDenom::from_str(
				msg.token.denom.base_denom.as_str(),
			)
			.unwrap(),
		};
		let denom = msg.token.denom.base_denom.to_string();
		let hashed_denom = CryptoHash::digest(denom.as_bytes());
		let split_denom: Vec<&str> = denom.as_str().split('/').collect();
		let split_denom = split_denom.last().unwrap();
		let token_mint = Pubkey::from_str(split_denom).unwrap();
		let sender_token_address = get_associated_token_address(
			&Pubkey::from_str(msg.sender.as_ref()).unwrap(),
			&token_mint,
		);
		let packet_data = ibc_new::apps::transfer::types::packet::PacketData {
			token: ibc_new::apps::transfer::types::PrefixedCoin {
				denom: prefixed_denom,
				amount: ibc_new::apps::transfer::types::Amount::from(msg.token.amount.as_u256().0),
			},
			sender: ibc_new::primitives::Signer::from(sender_token_address.to_string()),
			receiver: ibc_new::primitives::Signer::from(msg.receiver.as_ref().to_string()),
			memo: ibc_new::apps::transfer::types::Memo::from(msg.memo),
		};

		let token_account = get_associated_token_address(&authority.pubkey(), &token_mint);
		let seeds = [port_id.as_bytes(), channel_id.as_bytes(), hashed_denom.as_ref()];
		let escrow_account = Pubkey::find_program_address(&seeds, &self.program_id).0;

		let new_msg_transfer = ibc_new::apps::transfer::types::msgs::transfer::MsgTransfer {
			port_id_on_a: port_id.clone(),
			chan_id_on_a: channel_id.clone(),
			packet_data,
			timeout_height_on_b: ibc_new::core::channel::types::timeout::TimeoutHeight::At(
				ibc_new::core::client::types::Height::new(
					msg.timeout_height.revision_number,
					msg.timeout_height.revision_height,
				)
				.unwrap(),
			),
			timeout_timestamp_on_b: ibc_new::primitives::Timestamp::from_nanoseconds(
				msg.timeout_timestamp.nanoseconds(),
			)
			.unwrap(),
		};

		let sig = program
			.request()
			.instruction(ComputeBudgetInstruction::set_compute_unit_limit(1_000_000u32))
			.accounts(solana_ibc::accounts::SendTransfer {
				sender: authority.pubkey(),
				receiver: None,
				storage: solana_ibc_storage_key,
				trie: trie_key,
				chain: chain_key,
				system_program: system_program::ID,
				mint_authority: Some(mint_authority),
				token_mint: Some(token_mint),
				escrow_account: Some(escrow_account),
				receiver_token_account: Some(token_account),
				associated_token_program: Some(anchor_spl::associated_token::ID),
				token_program: Some(anchor_spl::token::ID),
			})
			.args(solana_ibc::instruction::SendTransfer {
				port_id,
				channel_id,
				hashed_base_denom: hashed_denom,
				msg: new_msg_transfer,
			})
			// .payer(Arc::new(keypair))
			.signer(&*authority)
			.send_with_spinner_and_config(RpcSendTransactionConfig {
				skip_preflight: true,
				..RpcSendTransactionConfig::default()
			})
			.await
			.unwrap();
		let rpc = program.async_rpc();
		let blockhash = rpc.get_latest_blockhash().await.unwrap();
		// Wait for finalizing the transaction
		let _ = rpc
			.confirm_transaction_with_spinner(&sig, &blockhash, CommitmentConfig::finalized())
			.await
			.unwrap();
		let signature = sig.to_string();
		Ok(signature)
	}
}
