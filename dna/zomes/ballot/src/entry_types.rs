use hdk::prelude::*;

#[hdk_entry]
#[derive(Clone)]
pub struct Ballot {
    pub title: String,
    pub description: String,
    pub created_by: AgentPubKey,
    pub created_at: Timestamp,
}
