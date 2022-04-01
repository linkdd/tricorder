#!/usr/bin/env

set -ex

source ./tests/setup_ssh.sh
source ./tests/setup_tricorder.sh

cargo test
