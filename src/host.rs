use crate::Result;

use serde_derive::Deserialize;
use ssh2::Session;

use std::{
  io::prelude::*,
  net::TcpStream,
};


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Host {
  pub id: String,
  pub address: String,
  #[serde(default = "Host::default_user")]
  pub user: String,
  #[serde(default = "Host::default_tags")]
  pub tags: Vec<String>,
}

impl Host {
  pub fn exec(&self, command: &str) -> Result<(i32, String)> {
    let sock = TcpStream::connect(self.address.clone())?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(sock);
    sess.handshake()?;
    sess.userauth_agent(&self.user)?;

    let mut channel = sess.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;

    let exit_code = channel.exit_status()?;

    Ok((exit_code, output))
  }

  pub fn default_user() -> String {
    String::from("root")
  }

  pub fn default_tags() -> Vec<String> {
    vec![]
  }
}
