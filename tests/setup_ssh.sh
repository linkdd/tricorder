#!/usr/bin/env bash
set -ex

# This script spawns an SSH daemon with a known configuration so that we can
# test various functionality against it.

SSH_FIXTURE_PORT=8022
SSH_DIR=$(pwd)/tests/sshd

cleanup_ssh() {
  # Stop the SSH server and local SSH agent
  kill $(< $SSH_DIR/sshd.pid) $SSH_AGENT_PID || true

  test -f $SSFDIR/sshd.log && cat $SSH_DIR/sshd.log
}
trap cleanup_ssh EXIT

# Blow away any prior state and re-configure our test server
rm -rf $SSH_DIR
mkdir -p $SSH_DIR

eval $(ssh-agent -s)

ssh-keygen -t rsa -f $SSH_DIR/id_rsa -N "" -q
chmod 0600 $SSH_DIR/id_rsa*
ssh-add $SSH_DIR/id_rsa
cp $SSH_DIR/id_rsa.pub $SSH_DIR/authorized_keys

ssh-keygen -f $SSH_DIR/ssh_host_rsa_key -N "" -t rsa

cat > $SSH_DIR/sshd_config <<-EOT
AuthorizedKeysFile=$SSH_DIR/authorized_keys
HostKey=$SSH_DIR/ssh_host_rsa_key
PidFile=$SSH_DIR/sshd.pid
Subsystem sftp internal-sftp
UsePAM yes
X11Forwarding no
UsePrivilegeSeparation no
PrintMotd yes
PermitTunnel yes
KbdInteractiveAuthentication yes
AllowTcpForwarding yes
MaxStartups 500
# Relax modes when the repo is under eg: /var/tmp
StrictModes no
EOT

cat $SSH_DIR/sshd_config

# Detect path to sshd binary
SSHD=/usr/sbin/sshd

if [ ! -f $SSHD ]
then
  SSHD=/usr/bin/sshd
fi

if [ ! -f $SSHD ]
then
  SSHD=$(which sshd)
fi

# Start an SSH server
$SSHD -p $SSH_FIXTURE_PORT -f $SSH_DIR/sshd_config -E $SSH_DIR/sshd.log
# Give it a moment to start up
sleep 2
