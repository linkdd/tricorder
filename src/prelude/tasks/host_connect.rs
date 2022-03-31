use crate::prelude::{Result, Host};

use ssh2::Session;
use std::net::TcpStream;

pub trait SSHProtocol {
  /// Open SSH session to host and authenticate using `ssh-agent`
  fn get_session(&self) -> Result<Session>;
}

impl SSHProtocol for Host {
  fn get_session(&self) -> Result<Session> {
    let sock = TcpStream::connect(self.address.clone())?;
    let mut sess = Session::new()?;

    sess.set_tcp_stream(sock);
    sess.handshake()?;
    sess.userauth_agent(&self.user)?;

    Ok(sess)
  }
}
