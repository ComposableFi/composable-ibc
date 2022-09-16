use ibc::core::ics02_client::header::Header;
use ibc::Height;
use tendermint_proto::Protobuf;

use super::types::LightClientBlockView;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NearHeader {
    inner: LightClientBlockView,
}

impl NearHeader {
    pub fn get_light_client_block_view(&self) -> &LightClientBlockView {
        &self.inner
    }
}

impl Header for NearHeader {
    fn encode_to_vec(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn height(&self) -> Height {
        todo!()
    }
}

impl Protobuf<()> for NearHeader {}

impl From<NearHeader> for () {
    fn from(_: NearHeader) -> Self {
        todo!()
    }
}

impl From<()> for NearHeader {
    fn from(_: ()) -> Self {
        todo!()
    }
}
