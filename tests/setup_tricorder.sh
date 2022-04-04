#!/usr/bin/env bash
set -ex

# This script prepare the fixtures for tricorder's test-suite

TRICORDER_DIR=$(pwd)/tests/tricorder
TRICORDER_USER=$(whoami)

# Blow away any prior state and re-configure our inventory
rm -rf $TRICORDER_DIR
mkdir -p $TRICORDER_DIR

cat > $TRICORDER_DIR/inventory.toml <<-EOT
[[hosts]]

id = "localhost"
address = "localhost:${SSH_FIXTURE_PORT}"
user = "${TRICORDER_USER}"
tags = ["local", "test-success"]
vars = { msg = "hi" }

[[hosts]]

id = "localhost-fail"
address = "non-existant-domain:22"
user = "${TRICORDER_USER}"
tags = ["local", "test-failure"]
vars = { msg = "hi" }
EOT
