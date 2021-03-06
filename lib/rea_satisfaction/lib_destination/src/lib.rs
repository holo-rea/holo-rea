/**
 * Holo-REA satisfaction zome library API
 *
 * Contains helper methods that can be used to manipulate `Satisfaction` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "destination" side of an "indirect remote index" pair
 * (@see `hdk_graph_helpers` README).
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    local_indexes::{
        query_direct_index_with_foreign_key,
        create_direct_index,
    },
};

use hc_zome_rea_economic_event_storage_consts::{EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::Entry;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

pub fn receive_create_satisfaction(satisfaction: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_satisfaction(&satisfaction)
}

pub fn receive_get_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<ResponseData> {
    handle_get_satisfaction(&address)
}

pub fn receive_update_satisfaction(satisfaction: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_satisfaction(&satisfaction)
}

pub fn receive_delete_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_satisfactions(&params)
}

fn handle_create_satisfaction(satisfaction: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (satisfaction_address, entry_resp): (SatisfactionAddress, Entry) = create_record(
        SATISFACTION_BASE_ENTRY_TYPE, SATISFACTION_ENTRY_TYPE,
        SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
        satisfaction.to_owned()
    )?;

    // link entries in the local DNA
    let _results = create_direct_index(
        satisfaction_address.as_ref(),
        satisfaction.get_satisfied_by().as_ref(),
        SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
        EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
    );

    // register in the associated foreign DNA as well
    // :TODO: probably need to remove this and rethink to use a message broadcast / respond flow
    // let _pingback = call(
    //     BRIDGED_PLANNING_DHT,
    //     "fulfillment",
    //     Address::from(PUBLIC_TOKEN.to_string()),
    //     "fulfillment_created",
    //     fulfillment.into(),
    // );

    Ok(construct_response(&satisfaction_address, &entry_resp))
}

fn handle_update_satisfaction(satisfaction: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = satisfaction.get_id();
    let new_entry = update_record(SATISFACTION_ENTRY_TYPE, &base_address, satisfaction)?;
    Ok(construct_response(&base_address, &new_entry))
}

/// Read an individual fulfillment's details
fn handle_get_satisfaction(base_address: &SatisfactionAddress) -> ZomeApiResult<ResponseData> {
    let entry = read_record_entry(base_address)?;
    Ok(construct_response(&base_address, &entry))
}

fn handle_query_satisfactions(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(SatisfactionAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = query_direct_index_with_foreign_key(
                satisfied_by, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    // :TODO: avoid cloning entry
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(&entry_base_address, &entry)),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}
