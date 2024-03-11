use alloc::vec::Vec;

use guestchain::{PubKey, Signature};
use lib::hash::CryptoHash;

use crate::proto;

/// The consensus header of the guest blockchain.
///
/// `From` and `TryFrom` conversions define mapping between this Rust object and
/// corresponding Protocol Message [`proto::Header`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header<PK: PubKey> {
    pub genesis_hash: CryptoHash,
    pub block_hash: CryptoHash,
    pub block_header: guestchain::BlockHeader,
    pub epoch_commitment: CryptoHash,
    pub epoch: guestchain::Epoch<PK>,
    pub signatures: Vec<(u16, PK::Signature)>,
}

impl<PK: PubKey> From<Header<PK>> for proto::Header {
    fn from(header: Header<PK>) -> Self { Self::from(&header) }
}

impl<PK: PubKey> From<&Header<PK>> for proto::Header {
    fn from(header: &Header<PK>) -> Self {
        let signatures = header
            .signatures
            .iter()
            .map(|(index, signature)| proto::Signature {
                index: u32::from(*index),
                signature: signature.to_vec(),
            })
            .collect();
        Self {
            genesis_hash: header.genesis_hash.to_vec(),
            block_header: borsh::to_vec(&header.block_header).unwrap(),
            epoch: borsh::to_vec(&header.epoch).unwrap(),
            signatures,
        }
    }
}

impl<PK: PubKey> TryFrom<proto::Header> for Header<PK> {
    type Error = proto::BadMessage;
    fn try_from(msg: proto::Header) -> Result<Self, Self::Error> {
        Self::try_from(&msg)
    }
}

impl<PK: PubKey> TryFrom<&proto::Header> for Header<PK> {
    type Error = proto::BadMessage;
    fn try_from(msg: &proto::Header) -> Result<Self, Self::Error> {
        let genesis_hash =
            lib::hash::CryptoHash::try_from(msg.genesis_hash.as_slice())
                .map_err(|_| proto::BadMessage)?;

        let bytes = msg.block_header.as_slice();
        let block_header = borsh::BorshDeserialize::try_from_slice(bytes)
            .map_err(|_| proto::BadMessage)?;
        let block_hash = CryptoHash::digest(bytes);

        let bytes = msg.epoch.as_slice();
        let epoch = borsh::BorshDeserialize::try_from_slice(bytes)
            .map_err(|_| proto::BadMessage)?;
        let epoch_commitment = CryptoHash::digest(bytes);

        let signatures = msg
            .signatures
            .iter()
            .map(|signature| {
                let index = u16::try_from(signature.index)
                    .map_err(|_| proto::BadMessage)?;
                let signature = PK::Signature::from_bytes(&signature.signature)
                    .map_err(|_| proto::BadMessage)?;
                Ok((index, signature))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            genesis_hash,
            block_hash,
            block_header,
            epoch_commitment,
            epoch,
            signatures,
        })
    }
}

super::any_convert! {
    proto::Header,
    Header<PK: guestchain::PubKey = guestchain::validators::MockPubKey>,
    // TODO(mina86): Add `obj: ...`.
}
