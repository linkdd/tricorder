use super::Result;

use ssh2::Session;

use serde_json::Value;
use serde_derive::{Serialize, Deserialize};

use std::{
  collections::HashMap,
  net::TcpStream,
};

/// Abstraction of a host found in the inventory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Host {
  /// Host identifier
  pub id: String,
  /// SSH host address in the form of `hostname:port`
  pub address: String,
  /// SSH user to authenticate with (defaults to `root`)
  #[serde(default = "default_user")]
  pub user: String,
  /// Tags used to apply commands on a subset of hosts from the inventory (defaults to `[]`)
  #[serde(default = "default_tags")]
  pub tags: Vec<String>,
  /// Variables specific to this host, used by templates (defaults to `{}`)
  #[serde(default = "default_vars")]
  pub vars: HashMap<String, Value>,
}

impl Host {
  /// Create a new host
  pub fn new(id: String, address: String) -> Self {
    Self {
      id,
      address,
      user: default_user(),
      tags: default_tags(),
      vars: default_vars(),
    }
  }

  /// Override this host's user
  pub fn set_user(&mut self, user: String) -> &mut Self {
    self.user = user;
    self
  }

  /// Add tag to this host
  pub fn add_tag(&mut self, tag: String) -> &mut Self {
    self.tags.push(tag);
    self
  }

  /// Remove tag from this host
  pub fn remove_tag(&mut self, tag: String) -> &mut Self {
    self.tags.retain(|current_tag| *current_tag != tag);
    self
  }

  /// Set host variable
  pub fn set_var(&mut self, key: String, val: Value) -> &mut Self {
    self.vars.insert(key, val);
    self
  }

  /// Remove host variable
  pub fn remove_var(&mut self, key: String) -> &mut Self {
    self.vars.remove(&key);
    self
  }

  /// Open SSH session to host and authenticate using `ssh-agent`
  pub fn get_session(&self) -> Result<Session> {
    let sock = TcpStream::connect(self.address.clone())?;
    let mut sess = Session::new()?;

    sess.set_tcp_stream(sock);
    sess.handshake()?;
    sess.userauth_agent(&self.user)?;

    Ok(sess)
  }
}

fn default_user() -> String {
  String::from("root")
}

fn default_tags() -> Vec<String> {
  vec![]
}

fn default_vars() -> HashMap<String, Value> {
  HashMap::new()
}
