//! **tricorder** uses an inventory to configure which hosts it needs to connect
//! to.
//!
//! The inventory is either a TOML file, or an executable producing a JSON
//! document.
//!
//! Both needs to respect the following structure:
//!
//! ## Root Document
//!
//! | Field | Type | Required |
//! | --- | --- | --- |
//! | `hosts` | List<[Host](#host)> | :white_check_mark: |
//!
//! ## Host
//!
//! | Field | Type | Required |
//! | --- | --- | --- |
//! | `id` | String | :white_check_mark: |
//! | `address` | String | :white_check_mark: |
//! | `user` | String | :x: (defaults to: `"root"`) |
//! | `tags` | List<String> | :x: (defaults to: `[]` |
//! | `vars` | Map<String, Any> | :x: (defaults to: `{}` |

use crate::{Result, Host};

use serde_derive::Deserialize;

/// Strutcure containing a deserialized inventory
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Inventory {
  /// List of hosts provided by this inventory
  #[serde(default = "Inventory::default_hostlist")]
  pub hosts: Vec<Host>,
}

impl Inventory {
  /// Create a new inventory with only `localhost`.
  ///
  /// Similar to the following inventory:
  ///
  /// ```toml
  /// [[hosts]]
  ///
  /// id = "localhost"
  /// address = "localhost:22"
  /// user = "root"
  /// tags = []
  /// vars = {}
  /// ```
  pub fn new() -> Self {
    Inventory { hosts: Self::default_hostlist() }
  }

  /// Deserialize a TOML document into an inventory.
  pub fn from_toml(content: &str) -> Result<Self> {
    let inventory: Self = toml::from_str(content)?;
    Ok(inventory)
  }

  /// Deserialize a JSON document into an inventory.
  pub fn from_json(content: &str) -> Result<Self> {
    let inventory: Self = serde_json::from_str(content)?;
    Ok(inventory)
  }

  /// Get `Some(host)` by its ID, or `None` if it does not exist.
  pub fn get_host_by_id(&self, id: String) -> Option<Host> {
    self.hosts.iter()
      .find(|host| host.id == id)
      .map(|host| host.clone())
  }

  /// Get a list of host matching at least on of the provided tags.
  pub fn get_hosts_by_tags(&self, tags: Vec<String>) -> Vec<Host> {
    self.hosts.iter()
      .filter(|host| {
        for tag in tags.iter() {
          if host.tags.contains(tag) {
            return true
          }
        }

        return false
      })
      .map(|host| host.clone())
      .collect()
  }

  /// Helper function for deserialization when no host is provided.
  ///
  /// Returns a single host pointing to `localhost`.
  pub fn default_hostlist() -> Vec<Host> {
    vec![Host {
      id: String::from("localhost"),
      address: String::from("localhost:22"),
      user: Host::default_user(),
      tags: Host::default_tags(),
      vars: Host::default_vars(),
    }]
  }
}
