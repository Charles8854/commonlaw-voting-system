pub mod entry_types;

use hdk::prelude::*;
use entry_types::Ballot;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Ballot(Ballot),
}

#[hdk_extern]
pub fn create_ballot(ballot: Ballot) -> ExternResult<Record> {
    create_entry(&ballot)?;
    let hash = hash_entry(&ballot)?;
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_ballot(hash: ActionHash) -> ExternResult<Option<Record>> {
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn update_ballot(original_hash: ActionHash, ballot: Ballot) -> ExternResult<Record> {
    update_entry(original_hash, &ballot)?;
    get(hash_entry(&ballot)?, GetOptions::default())
}
