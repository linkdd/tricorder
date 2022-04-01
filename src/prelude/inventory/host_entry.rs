use crate::prelude::Result;
use super::{
  host_id::HostId,
  host_tag::HostTag,
};

use serde_json::Value;
use serde_derive::{Serialize, Deserialize};

use ssh2::Session;
use std::{
  net::TcpStream,
  collections::HashMap,
};

/// Abstraction of a host found in the inventory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Host {
  /// Host identifier
  pub id: HostId,
  /// SSH host address in the form of `hostname:port`
  pub address: String,
  /// SSH user to authenticate with (defaults to `root`)
  #[serde(default = "default_user")]
  pub user: String,
  /// Tags used to apply commands on a subset of hosts from the inventory (defaults to `[]`)
  #[serde(default = "default_tags")]
  pub tags: Vec<HostTag>,
  /// Variables specific to this host, used by templates (defaults to `{}`)
  #[serde(default = "default_vars")]
  pub vars: HashMap<String, Value>,
}

impl Host {
  /// Shortcut to `HostId::new()`
  pub fn id(src: &str) -> Result<HostId> {
    HostId::new(src)
  }

  /// Shortcut to `HostTag::new()`
  pub fn tag(src: &str) -> Result<HostTag> {
    HostTag::new(src)
  }

  /// Create a new host
  pub fn new(id: HostId, address: String) -> Self {
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
  pub fn add_tag(&mut self, tag: HostTag) -> &mut Self {
    self.tags.push(tag);
    self
  }

  /// Remove tag from this host
  pub fn remove_tag(&mut self, tag: HostTag) -> &mut Self {
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

fn default_tags() -> Vec<HostTag> {
  vec![]
}

fn default_vars() -> HashMap<String, Value> {
  HashMap::new()
}
