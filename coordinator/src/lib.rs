use hdk::prelude::*;

#[hdk_extern]
pub fn get_resource(entry_hash: EntryHash) -> ExternResult<Option<Record>> {
    get(entry_hash, GetOptions::default())
}