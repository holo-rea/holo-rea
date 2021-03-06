/**
 * Holo-REA satisfaction zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::MaybeUndefined;
use vf_core::{
    measurement::QuantityValue,
    type_aliases::{
        EventOrCommitmentAddress,
        IntentAddress,
        CommitmentAddress,
        Address,
    },
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_core::type_aliases::{ SatisfactionAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: SatisfactionAddress,
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub satisfaction: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data

    pub fn get_satisfied_by(&'a self) -> &EventOrCommitmentAddress {
        &self.satisfied_by
    }

    pub fn get_satisfies(&'a self) -> &IntentAddress {
        &self.satisfies
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdCreateRequest {
    pub satisfaction: CreateRequest,
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckCommitmentRequest {
    pub address: Address,
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub id: SatisfactionAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub satisfied_by: MaybeUndefined<EventOrCommitmentAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub satisfies: MaybeUndefined<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &SatisfactionAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdUpdateRequest {
    pub satisfaction: UpdateRequest,
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub satisfies: Option<IntentAddress>,
    pub satisfied_by: Option<CommitmentAddress>,
}
