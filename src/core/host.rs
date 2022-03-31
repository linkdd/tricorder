use super::{Result, Error};

use serde::Deserialize;
use ssh2::Session;

use serde_json::Value;
use serde_derive::{Serialize, Deserialize};
use serde::de::Deserializer;
use regex::Regex;

use std::{
  collections::HashMap,
  net::TcpStream,
};

const HOST_ID_REGEX: &str = r"^[a-zA-Z0-9_][a-zA-Z0-9_\-]*$";
const HOST_TAG_REGEX: &str = r"^[^!\&\|\t\n\r\f\(\) ]+$";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HostId(String);
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HostTag(String);

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

impl HostId {
  /// Create a new host identifier from a string.
  ///
  /// Example:
  ///
  /// ```rust
  /// use tricorder::core::HostId;
  ///
  /// let id = HostId::new("example").unwrap();
  /// # assert_eq!(id.to_string(), String::from("example"));
  /// ```
  pub fn new(src: &str) -> Result<Self> {
    let re = Regex::new(HOST_ID_REGEX)?;
    if !re.is_match(src) {
      Err(Box::new(Error::InvalidHostId(
        format!("ID {} does not match regex {}", src, HOST_ID_REGEX)
      )))
    }
    else {
      Ok(Self(src.to_string()))
    }
  }

  /// Return the underlying string
  pub fn to_string(self) -> String {
    let Self(s) = self;
    s.clone()
  }
}

impl<'de> Deserialize<'de> for HostId {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    let src = String::deserialize(deserializer)?;
    HostId::new(src.as_str()).map_err(serde::de::Error::custom)
  }
}

impl<'de> Deserialize<'de> for HostTag {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    let src = String::deserialize(deserializer)?;
    HostTag::new(src.as_str()).map_err(serde::de::Error::custom)
  }
}

impl HostTag {
  /// Create a new host identifier from a string.
  ///
  /// Example:
  ///
  /// ```rust
  /// use tricorder::core::HostTag;
  ///
  /// let tag = HostTag::new("example").unwrap();
  /// # assert_eq!(tag.to_string(), String::from("example"));
  /// ```
  pub fn new(src: &str) -> Result<Self> {
    let re = Regex::new(HOST_TAG_REGEX)?;
    if !re.is_match(src) {
      Err(Box::new(Error::InvalidHostTag(
        format!("Tag {} does not match regex {}", src, HOST_TAG_REGEX)
      )))
    }
    else {
      Ok(Self(src.to_string()))
    }
  }

  pub fn to_string(self) -> String {
    let Self(s) = self;
    s
  }
}

impl Host {
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
