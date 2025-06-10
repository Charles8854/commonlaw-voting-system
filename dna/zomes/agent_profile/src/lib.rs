pub mod entry_types;

use hdk::prelude::*;
use entry_types::AgentProfile;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    AgentProfile(AgentProfile),
}

#[hdk_extern]
pub fn create_agent_profile(profile: AgentProfile) -> ExternResult<Record> {
    create_entry(&profile)?;
    let hash = hash_entry(&profile)?;
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_agent_profile(hash: ActionHash) -> ExternResult<Option<Record>> {
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn update_agent_profile(original_hash: ActionHash, profile: AgentProfile) -> ExternResult<Record> {
    update_entry(original_hash, &profile)?;
    get(hash_entry(&profile)?, GetOptions::default())
}
