pub mod entry_types;

use hdk::prelude::*;
use entry_types::VoteCast;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    VoteCast(VoteCast),
}
#[hdk_extern]
pub fn cast_vote(vote: VoteCast) -> ExternResult<Record> {
    let agent = agent_info()?.agent_latest_pubkey;

    // Create VoteCast
    create_entry(&vote)?;
    let vote_hash = hash_entry(&vote)?;

    // Link from agent to their vote
    create_link(agent, vote_hash.clone(), LinkTag::new("my_vote"))?;

    get(vote_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_vote(hash: ActionHash) -> ExternResult<Option<Record>> {
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn update_vote(original_hash: ActionHash, vote: VoteCast) -> ExternResult<Record> {
    update_entry(original_hash, &vote)?;
    get(hash_entry(&vote)?, GetOptions::default())
}
#[hdk_extern]
pub fn get_my_votes(agent: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(agent, Some(LinkTag::new("my_vote")))?;

    let mut votes = Vec::new();
    for link in links {
        if let Some(record) = get(link.target.clone(), GetOptions::default())? {
            votes.push(record);
        }
    }
    Ok(votes)
}
