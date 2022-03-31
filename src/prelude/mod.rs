//! Core features of **tricorder**.
//!
//! **tricorder** uses an inventory to configure which hosts it needs to connect
//! to and which data are associated to those specific hosts.
//!
//! This inventory can be built from a TOML document:
//!
//! ```toml
//! [[hosts]]
//!
//! id = "localhost"
//! address = "localhost:22"
//! user = "root"
//! tags = ["local"]
//! vars = { foo = "bar" }
//! ```
//!
//! A JSON document:
//!
//! ```json
//! {"hosts": [
//!   {
//!     "id": "localhost",
//!     "address": "localhost:22",
//!     "user": "root",
//!     "tags": ["local"],
//!     "vars": {"foo": "bar"}
//!   }
//! ]}
//! ```
//!
//! Or directly via the Rust API:
//!
//! ```rust
//! use tricorder::prelude::{Inventory, Host};
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new(Host::id("localhost").unwrap(), "localhost:22".to_string())
//!       .set_user("root".to_string())
//!       .add_tag(Host::tag("local").unwrap())
//!       .set_var("foo".to_string(), json!("bar"))
//!       .to_owned()
//!   )
//!   .to_owned();
//! ```

mod result;
mod error;
mod inventory;
mod tasks;

pub use self::{
  result::Result,
  error::Error,
  inventory::*,
  tasks::*,
};
