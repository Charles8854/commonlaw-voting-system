use hdk::prelude::*;

#[hdk_entry]
#[derive(Clone)]
pub struct AgentProfile {
    pub name: String,
    pub email: String,
    pub joined_at: Timestamp,
}
