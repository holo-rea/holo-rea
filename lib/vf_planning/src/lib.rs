/**
 * Planning module datatypes & behaviours
 */

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hdk_graph_helpers;
extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod fulfillment;
pub mod satisfaction;

pub mod identifiers {
    pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";

    pub const FULFILLMENT_BASE_ENTRY_TYPE: &str = "vf_fulfillment_baseurl";
    pub const FULFILLMENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_fulfillment_entry";
    pub const FULFILLMENT_ENTRY_TYPE: &str = "vf_fulfillment";
    pub const FULFILLMENT_FULFILLS_LINK_TYPE: &str = "vf_fulfillment_fulfills";
    pub const FULFILLMENT_FULFILLS_LINK_TAG: &str = "fulfills";
    pub const FULFILLMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_fulfillment_fulfilled_by";
    pub const FULFILLMENT_FULFILLEDBY_LINK_TAG: &str = "fulfilled_by";

    pub const SATISFACTION_BASE_ENTRY_TYPE: &str = "vf_satisfaction_baseurl";
    pub const SATISFACTION_INITIAL_ENTRY_LINK_TYPE: &str = "vf_satisfaction_entry";
    pub const SATISFACTION_ENTRY_TYPE: &str = "vf_satisfaction";
    pub const SATISFACTION_SATISFIES_LINK_TYPE: &str = "vf_satisfaction_satisfies";
    pub const SATISFACTION_SATISFIES_LINK_TAG: &str = "satisfies";
    pub const SATISFACTION_SATISFIEDBY_LINK_TYPE: &str = "vf_satisfaction_satisfied_by";
    pub const SATISFACTION_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";
}
