+++
title = "howto start"
documentName = "Getting Started"
description = "Build, install and run tricorder"
menuHref = "/tutorials/"
weight = 1
+++

{{< wip >}}
# Build from Source

Clone from repository
```shell
git clone git@github.com:linkdd/tricorder.git
cd tricorder
```
Compile from source (requirement rust installed via rustup):
```shell
cargo install --locked --path . 
```
make sure `~/.cargo/bin` is on the PATH