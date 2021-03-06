/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const PROPOSED_INTENT_ENTRY_TYPE: &str = "vf_proposed_intent";
pub const PROPOSED_INTENT_BASE_ENTRY_TYPE: &str = "vf_proposed_intent_baseurl";
pub const PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposed_intent_entry";

pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE: &str = "vf_proposed_intent_published_in";
pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG: &str = "published_in";

pub const PROPOSED_INTENT_PUBLISHES_LINK_TYPE: &str = "vf_proposed_intent_publishes";
pub const PROPOSED_INTENT_PUBLISHES_LINK_TAG: &str = "publishes";

pub const BRIDGED_PLANNING_DHT: &str = "vf_planning";
