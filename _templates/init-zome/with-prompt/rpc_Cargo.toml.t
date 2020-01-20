---
to: lib/<%= h.changeCase.snake(zome_name) %>/rpc/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1.0.89"
serde_json = { version = "=1.0.39", features = ["preserve_order"] }
serde_derive = "=1.0.89"
holochain_json_api = "=0.0.17"
holochain_json_derive = "=0.0.17"

hdk_graph_helpers = { path = "../../hdk_graph_helpers" }
vf_core = { path = "../../vf_core" }

[lib]
crate-type = ["lib"]
