// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;

mod intent_requests;
mod satisfaction_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};

use hdk_graph_helpers::{
    LINK_TYPE_INITIAL_ENTRY,
};
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use intent_requests::{
    handle_get_intent,
    handle_create_intent,
    handle_update_intent,
    handle_delete_intent,
};
use satisfaction_requests::{
    handle_link_satisfactions,
    handle_get_satisfactions,
};
use vf_planning::intent::ResponseData as IntentResponse;
use vf_planning::identifiers::{
    INTENT_ENTRY_TYPE,
    INTENT_BASE_ENTRY_TYPE,
    INTENT_SATISFIEDBY_LINK_TYPE,
    INTENT_SATISFIEDBY_LINK_TAG,
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_SATISFIES_LINK_TYPE,
    COMMITMENT_SATISFIES_LINK_TAG,
};

// Zome entry type wrappers

fn intent_entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_ENTRY_TYPE,
        description: "A planned economic flow, which can lead to economic events (sometimes through commitments).",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

fn intent_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial intent addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                INTENT_ENTRY_TYPE,
                link_type: LINK_TYPE_INITIAL_ENTRY,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: INTENT_SATISFIEDBY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: COMMITMENT_SATISFIES_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

fn commitment_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_BASE_ENTRY_TYPE,
        description: "Pointer to a commitment from related commitments zome.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
        ]
    )
}

// Zome definition

define_zome! {
    entries: [
        intent_entry_def(),
        intent_base_entry_def(),
        commitment_base_entry_def()
    ]

    init: || {
        Ok(())
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    receive: |from, payload| {
      format!("Received: {} from {}", payload, from)
    }

    functions: [
        create_intent: {
            inputs: |intent: CreateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: handle_create_intent
        }
        get_intent: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: handle_get_intent
        }
        update_intent: {
            inputs: |intent: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: handle_update_intent
        }
        delete_intent: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_intent
        }

        link_satisfactions: {
            inputs: |base_entry: Address, target_entries: Vec<Address>|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_link_satisfactions
        }
        get_satisfactions: {
            inputs: |economic_event: Address|,
            outputs: |result: ZomeApiResult<Vec<IntentResponse>>|,
            handler: handle_get_satisfactions
        }
    ]

    traits: {
        hc_public [
            create_intent,
            get_intent,
            update_intent,
            delete_intent,
            link_satisfactions
        ]
    }
}