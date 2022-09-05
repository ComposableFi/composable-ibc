use alloc::collections::{BTreeMap, BTreeSet};
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::voter_set::VoterSet;
use primitives::{error, Commit, HostFunctions};
use sp_finality_grandpa::{
	AuthorityId, AuthorityList, ConsensusLog, ScheduledChange, GRANDPA_ENGINE_ID,
};
use sp_runtime::{
	generic::OpaqueDigestItemId,
	traits::{Block as BlockT, Header as HeaderT, NumberFor},
};
use sp_std::prelude::*;

/// A GRANDPA justification for block finality, it includes a commit message and
/// an ancestry proof including all headers routing all precommit target blocks
/// to the commit target block. Due to the current voting strategy the precommit
/// targets should be the same as the commit target, since honest voters don't
/// vote past authority set change blocks.
///
/// This is meant to be stored in the db and passed around the network to other
/// nodes, and are used by syncing nodes to prove authority set handoffs.
#[derive(Clone, Encode, Decode, PartialEq, Eq)]
pub struct GrandpaJustification<Block: BlockT> {
	pub round: u64,
	pub commit: Commit<Block>,
	pub votes_ancestries: Vec<Block::Header>,
}

impl<Block: BlockT> GrandpaJustification<Block> {
	/// Validate the commit and the votes' ancestry proofs.
	pub fn verify<H>(&self, set_id: u64, authorities: &AuthorityList) -> Result<(), error::Error>
	where
		H: HostFunctions,
		NumberFor<Block>: finality_grandpa::BlockNumberOps,
	{
		let voters =
			VoterSet::new(authorities.iter().cloned()).ok_or(anyhow!("Invalid AuthoritiesSet"))?;

		self.verify_with_voter_set::<H>(set_id, &voters)
	}

	/// Validate the commit and the votes' ancestry proofs.
	pub(crate) fn verify_with_voter_set<H>(
		&self,
		set_id: u64,
		voters: &VoterSet<AuthorityId>,
	) -> Result<(), error::Error>
	where
		H: HostFunctions,
		NumberFor<Block>: finality_grandpa::BlockNumberOps,
	{
		use finality_grandpa::Chain;

		let ancestry_chain = AncestryChain::<Block>::new(&self.votes_ancestries);

		match finality_grandpa::validate_commit(&self.commit, voters, &ancestry_chain) {
			Ok(ref result) if result.is_valid() => {},
			err => {
				let result = err?;
				Err(anyhow!("invalid commit in grandpa justification: {result:?}"))?
			},
		}

		// we pick the precommit for the lowest block as the base that
		// should serve as the root block for populating ancestry (i.e.
		// collect all headers from all precommit blocks to the base)
		let base_hash = self
			.commit
			.precommits
			.iter()
			.map(|signed| &signed.precommit)
			.min_by_key(|precommit| precommit.target_number)
			.map(|precommit| precommit.target_hash.clone())
			.expect(
				"can only fail if precommits is empty; \
				 commit has been validated above; \
				 valid commits must include precommits; \
				 qed.",
			);

		let mut buf = Vec::new();
		let mut visited_hashes = BTreeSet::new();
		for signed in self.commit.precommits.iter() {
			let message = finality_grandpa::Message::Precommit(signed.precommit.clone());
			// clear the buffer
			buf.clear();
			(message, self.round, set_id).encode_to(&mut buf);
			if !H::ed25519_verify(signed.signature.as_ref(), &buf, signed.id.as_ref()) {
				Err(anyhow!("invalid signature for precommit in grandpa justification"))?
			}

			if base_hash == signed.precommit.target_hash {
				continue
			}

			let route = ancestry_chain.ancestry(base_hash, signed.precommit.target_hash)?;
			// ancestry starts from parent hash but the precommit target hash has been
			// visited
			visited_hashes.insert(signed.precommit.target_hash);
			for hash in route {
				visited_hashes.insert(hash);
			}
		}

		let ancestry_hashes: BTreeSet<_> =
			self.votes_ancestries.iter().map(|h: &Block::Header| h.hash()).collect();

		if visited_hashes != ancestry_hashes {
			Err(anyhow!(
				"invalid precommit ancestries in grandpa justification with unused headers",
			))?
		}

		Ok(())
	}

	/// The target block number and hash that this justifications proves finality for.
	pub fn target(&self) -> (NumberFor<Block>, Block::Hash) {
		(self.commit.target_number, self.commit.target_hash)
	}
}

/// A utility trait implementing `finality_grandpa::Chain` using a given set of headers.
/// This is useful when validating commits, using the given set of headers to
/// verify a valid ancestry route to the target commit block.
pub struct AncestryChain<Block: BlockT> {
	ancestry: BTreeMap<Block::Hash, Block::Header>,
}

impl<Block: BlockT> AncestryChain<Block> {
	pub fn new(ancestry: &[Block::Header]) -> AncestryChain<Block> {
		let ancestry: BTreeMap<_, _> =
			ancestry.iter().cloned().map(|h: Block::Header| (h.hash(), h)).collect();

		AncestryChain { ancestry }
	}

	pub fn header(&self, hash: &Block::Hash) -> Option<&Block::Header> {
		self.ancestry.get(hash)
	}
}

impl<Block: BlockT> finality_grandpa::Chain<Block::Hash, NumberFor<Block>> for AncestryChain<Block>
where
	NumberFor<Block>: finality_grandpa::BlockNumberOps,
{
	fn ancestry(
		&self,
		base: Block::Hash,
		block: Block::Hash,
	) -> Result<Vec<Block::Hash>, finality_grandpa::Error> {
		let mut route = Vec::new();
		let mut current_hash = block;
		loop {
			if current_hash == base {
				break
			}
			match self.ancestry.get(&current_hash) {
				Some(current_header) => {
					current_hash = *current_header.parent_hash();
					route.push(current_hash);
				},
				_ => return Err(finality_grandpa::Error::NotDescendent),
			}
		}
		route.pop(); // remove the base

		Ok(route)
	}
}

/// Checks the given header for a consensus digest signalling a **standard** scheduled change and
/// extracts it.
pub fn find_scheduled_change<B: BlockT>(
	header: &B::Header,
) -> Option<ScheduledChange<NumberFor<B>>> {
	let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

	let filter_log = |log: ConsensusLog<NumberFor<B>>| match log {
		ConsensusLog::ScheduledChange(change) => Some(change),
		_ => None,
	};

	// find the first consensus digest with the right ID which converts to
	// the right kind of consensus log.
	header.digest().convert_first(|l| l.try_to(id).and_then(filter_log))
}

/// Checks the given header for a consensus digest signalling a **forced** scheduled change and
/// extracts it.
pub fn find_forced_change<B: BlockT>(
	header: &B::Header,
) -> Option<(NumberFor<B>, ScheduledChange<NumberFor<B>>)> {
	let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

	let filter_log = |log: ConsensusLog<NumberFor<B>>| match log {
		ConsensusLog::ForcedChange(delay, change) => Some((delay, change)),
		_ => None,
	};

	// find the first consensus digest with the right ID which converts to
	// the right kind of consensus log.
	header.digest().convert_first(|l| l.try_to(id).and_then(filter_log))
}

#[cfg(test)]
mod tests {
	use crate::{
		justification::{
			find_forced_change, find_scheduled_change, AuthorityList, FinalityProof,
			GrandpaJustification,
		},
		kusama,
	};
	use codec::{Decode, Encode};
	use finality_grandpa_rpc::GrandpaApiClient;
	use futures::StreamExt;
	use polkadot_core_primitives::{Block, Header};
	use serde::{Deserialize, Serialize};
	use sp_core::H256;
	use std::mem::size_of_val;
	use subxt::{
		rpc::{rpc_params, ClientT},
		DefaultConfig,
	};

	pub type Justification = GrandpaJustification<Block>;

	/// An encoded justification proving that the given header has been finalized
	#[derive(Clone, Serialize, Deserialize)]
	pub struct JustificationNotification(sp_core::Bytes);

	#[tokio::test]
	async fn follow_grandpa_justifications() {
		let url = std::env::var("NODE_ENDPOINT")
			.unwrap_or("wss://kusama-rpc.polkadot.io:443".to_string());
		let client = subxt::ClientBuilder::new()
			.set_url(url)
			.build::<DefaultConfig>()
			.await
			.expect("Failed to initialize subxt");

		let api = client
			.clone()
			.to_runtime_api::<kusama::api::RuntimeApi<DefaultConfig, subxt::PolkadotExtrinsicParams<_>>>(
			);

		let mut subscription = client
			.rpc()
			.subscribe_finalized_blocks()
			.await
			.expect("Failed to subscribe finalized blocks");

		let mut current_set_id = api
			.storage()
			.grandpa()
			.current_set_id(None)
			.await
			.expect("Failed to fetch current set id");

		let mut authorities = {
			let bytes = client
				.rpc()
				.client
				.request::<String>(
					"state_call",
					rpc_params!(
						"GrandpaApi_grandpa_authorities",
						"0x",
						format!("{:?}", header.hash())
					),
				)
				.await
				.map(|res| hex::decode(&res[2..]))
				.expect("Failed to fetch authorities")
				.expect("Failed to hex decode authorities");

			AuthorityList::decode(&mut &bytes[..]).expect("Failed to scale decode authorities")
		};

		while let Some(Ok(header)) = subscription.next().await {
			let header =
				Header::decode(&mut &header.encode()[..]).expect("Failed to decode header");
			println!("========= New Header =========");
			println!("Got header: Hash({:?}), Number({})", header.hash(), header.number);
			println!("current_set_id: {current_set_id}");

			let encoded = GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
				&*client.rpc().client,
				header.number,
			)
			.await
			.expect("Failed to fetch finality proof")
			.expect("Failed to fetch finality proof")
			.0
			 .0;

			println!("justification size: {}kb", size_of_val(&*encoded) / 1000);
			let finality_proof = FinalityProof::<Header>::decode(&mut &encoded[..])
				.expect("Failed to decode finality proof");
			let unknown_headers =
				finality_proof.unknown_headers.iter().map(|h| h.number).collect::<Vec<_>>();
			println!("unknown_headers: {unknown_headers:#?}",);

			let mut justification = Justification::decode(&mut &finality_proof.justification[..])
				.expect("Failed to decode justification");

			justification
				.verify(current_set_id, &authorities)
				.expect("Failed to verify proof");

			let pre_commits = justification.commit.precommits.drain(..).collect::<Vec<_>>();

			println!("{justification:#?}");
			println!("Signatures: {:#?}", pre_commits.len());

			if let Some(scheduled_authority_set) = find_scheduled_change::<Block>(&header) {
				println!(
					"Found scheduled authority set with delay: {} blocks",
					scheduled_authority_set.delay
				);
				current_set_id += 1;
				authorities = scheduled_authority_set.next_authorities;
			}
			if let Some((delay, forced_authority_set)) = find_forced_change::<Block>(&header) {
				println!("Found forced authority set: block import delay: {delay} blocks");
				println!("forced_authority_set.delay: {} blocks", forced_authority_set.delay);
			}
			println!("========= Successfully verified grandpa justification =========");
		}
	}
}
