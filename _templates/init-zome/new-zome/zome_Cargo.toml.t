---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(zome_name) %>/code/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "=1.0.104"
serde_json = { version = "=1.0.47", features = ["preserve_order"] }
# :DUPE: hdk-rust-revid
hdk = "=0.0.47-alpha1"
hdk_proc_macros = "=0.0.47-alpha1"

hc_zome_<%= h.changeCase.snake(zome_name) %>_defs = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/defs" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/rpc" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_lib = { path = "../../../../../lib/<%= h.changeCase.snake(zome_name) %>/lib" }

[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]
