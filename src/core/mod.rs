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
//! use tricorder::core::{Inventory, Host};
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new("localhost", "localhost:22")
//!       .set_user("root")
//!       .add_tag("local")
//!       .set_var("foo", json!("bar"))
//!   );
//! ```

mod result;
mod error;
mod inventory;
mod host;

pub use self::{
  result::Result,
  error::Error,
  inventory::Inventory,
  host::Host,
};
