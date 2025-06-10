use hdk::prelude::*;

#[hdk_entry]
#[derive(Clone)]
pub struct VoteCast {
    pub vote_item_hash: ActionHash,
    pub voter: AgentPubKey,
    pub voted_at: Timestamp,
}
