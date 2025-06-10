use hdk::prelude::*;

#[hdk_entry]
#[derive(Clone)]
pub struct VoteItem {
    pub ballot_hash: ActionHash,
    pub option_text: String,
}
