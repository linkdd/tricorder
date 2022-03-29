use crate::core::{Result, Host};
use serde_derive::{Serialize, Deserialize};

/// Abstraction of inventory file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
  /// List of host provided by inventory (defaults to `[]`)
  #[serde(default = "default_hostlist")]
  pub hosts: Vec<Host>,
}

impl Inventory {
  /// Create a new empty inventory
  pub fn new() -> Self {
    Inventory { hosts: vec![] }
  }

  /// Deserialize a TOML document into an inventory.
  ///
  /// Example:
  ///
  /// ```toml
  /// [[hosts]]
  ///
  /// id = "localhost"
  /// address = "localhost:22"
  /// user = "root"
  /// tags = ["local"]
  /// vars = { foo = "bar" }
  /// ```
  pub fn from_toml(content: &str) -> Result<Self> {
    let inventory: Self = toml::from_str(content)?;
    Ok(inventory)
  }

  /// Deserialize a JSON document into an inventory.
  ///
  /// Example:
  ///
  /// ```json
  /// {"hosts": [
  ///   {
  ///     "id": "localhost",
  ///     "address": "localhost:22",
  ///     "user": "root",
  ///     "tags": ["local"],
  ///     "vars": {"foo": "bar"}
  ///   }
  /// ]}
  /// ```
  pub fn from_json(content: &str) -> Result<Self> {
    let inventory: Self = serde_json::from_str(content)?;
    Ok(inventory)
  }

  /// Add host to the inventory.
  pub fn add_host(&mut self, host: Host) -> &mut Self {
    self.hosts.push(host);
    self
  }

  /// Remove host from the inventory or do nothing if the host's ID was not
  /// found.
  pub fn remove_host(&mut self, host_id: String) -> &mut Self {
    self.hosts.retain(|host| host.id != host_id);
    self
  }

  /// Get `Some(host)` by its ID, or `None` if it does not exist.
  pub fn get_host_by_id(&self, id: String) -> Option<Host> {
    self.hosts.iter()
      .find(|host| host.id == id)
      .map(|host| host.clone())
  }

  /// Get a list of host matching at least on of the provided tags.
  pub fn get_hosts_by_tags(&self, tags: Vec<String>) -> Vec<Host> {
    self.hosts
      .iter()
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
}

fn default_hostlist() -> Vec<Host> {
  vec![]
}
