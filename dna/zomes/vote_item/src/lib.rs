pub mod entry_types;

use hdk::prelude::*;
use entry_types::VoteItem;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    VoteItem(VoteItem),
}

#[hdk_extern]
pub fn create_vote_item(item: VoteItem) -> ExternResult<Record> {
    create_entry(&item)?;
    let hash = hash_entry(&item)?;
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_vote_item(hash: ActionHash) -> ExternResult<Option<Record>> {
    get(hash, GetOptions::default())
}

#[hdk_extern]
pub fn update_vote_item(original_hash: ActionHash, item: VoteItem) -> ExternResult<Record> {
    update_entry(original_hash, &item)?;
    get(hash_entry(&item)?, GetOptions::default())
}
#[hdk_extern]
pub fn create_vote_item_with_link(input: (ActionHash, VoteItem)) -> ExternResult<Record> {
    let (ballot_hash, item) = input;

    // Create the VoteItem entry
    create_entry(&item)?;
    let vote_item_hash = hash_entry(&item)?;

    // Link it to the Ballot
    create_link(ballot_hash.clone(), vote_item_hash.clone(), LinkTag::new("vote_item"))?;

    // Return the record
    get(vote_item_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_vote_items_for_ballot(ballot_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(ballot_hash, Some(LinkTag::new("vote_item")))?;

    let mut items = Vec::new();
    for link in links {
        if let Some(record) = get(link.target.clone(), GetOptions::default())? {
            items.push(record);
        }
    }
    Ok(items)
}
