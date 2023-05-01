use super::{host_entry::Host, host_id::HostId, tag_expr::eval_tag_expr};
use crate::prelude::{Error, Result};

use serde_derive::{Deserialize, Serialize};

use is_executable::IsExecutable;
use std::{fs, path::Path, process::Command};

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

    /// Parse inventory from a file or executable
    pub fn from_file(path: &str) -> Result<Self> {
        let inventory_path = Path::new(path);

        if inventory_path.exists() {
            if inventory_path.is_executable() {
                let result = Command::new(path).output()?;

                if !result.status.success() {
                    Err(Box::new(Error::CommandExecutionFailed(format!(
                        "Failed to execute inventory {}: {}",
                        path, result.status
                    ))))
                } else {
                    let content = String::from_utf8(result.stdout)?;
                    Ok(Inventory::from_json(&content)?)
                }
            } else {
                let content = fs::read_to_string(path)?;
                Ok(Inventory::from_toml(&content)?)
            }
        } else {
            Err(Box::new(Error::FileNotFound(format!(
                "Inventory '{}' does not exist",
                path
            ))))
        }
    }

    /// Add host to the inventory.
    pub fn add_host(&mut self, host: Host) -> &mut Self {
        self.hosts.push(host);
        self
    }

    /// Remove host from the inventory or do nothing if the host's ID was not
    /// found.
    pub fn remove_host(&mut self, host_id: HostId) -> &mut Self {
        self.hosts.retain(|host| host.id != host_id);
        self
    }

    /// Get `Some(host)` by its ID, or `None` if it does not exist.
    pub fn get_host_by_id(&self, id: HostId) -> Option<Host> {
        self.hosts
            .iter()
            .find(|host| host.id == id)
            .map(|host| host.clone())
    }

    /// Get a list of host matching the tag expression.
    pub fn get_hosts_by_tags(&self, tag_expr: String) -> Result<Vec<Host>> {
        let mut hosts = vec![];

        for host in self.hosts.iter() {
            let tags = host
                .tags
                .iter()
                .map(|tag| tag.clone().to_string())
                .collect();
            if eval_tag_expr(&tag_expr, tags)? {
                hosts.push(host.clone());
            }
        }

        Ok(hosts)
    }
}

fn default_hostlist() -> Vec<Host> {
    vec![]
}
