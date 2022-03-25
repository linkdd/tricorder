use crate::{Result, Host};

use serde_derive::Deserialize;


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Inventory {
  #[serde(default = "default_hostlist")]
  pub hosts: Vec<Host>,
}

impl Inventory {
  pub fn new() -> Self {
    Inventory { hosts: default_hostlist() }
  }

  pub fn from_toml(content: &str) -> Result<Self> {
    let inventory: Self = toml::from_str(content)?;
    Ok(inventory)
  }

  pub fn from_json(content: &str) -> Result<Self> {
    let inventory: Self = serde_json::from_str(content)?;
    Ok(inventory)
  }

  pub fn get_host_by_id(&self, id: String) -> Option<Host> {
    self.hosts.iter()
      .find(|host| host.id == id)
      .map(|host| host.clone())
  }

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
}

fn default_hostlist() -> Vec<Host> {
  vec![Host {
    id: String::from("localhost"),
    address: String::from("localhost:22"),
    user: String::from("root"),
    tags: vec![],
  }]
}
