use ibc_proto::ibc::core::client::v1::Height;

pub fn incerement_proof_height(height: Option<Height>) -> Option<Height> {
    match height {
        Some(height) => Some(Height {
            revision_number: height.revision_number,
            revision_height: height.revision_height + 1,
        }),
        None => None,
    }
}