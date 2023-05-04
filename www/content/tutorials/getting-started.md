+++
title = "howto start"
documentName = "Getting Started"
description = "Build, install and run tricorder"
menuHref = "/tutorials/"
weight = 1
+++

{{< wip >}}
# Build from Source

**Requirements:**

| package | minimal Version |
| - | - |
| Rust | tested with version 1.69 |
| cargo | tested with version 1.67.0 |
| git | tested with version 2.40.0 |

Clone from repository:
```shell
git clone git@github.com:linkdd/tricorder.git
cd tricorder
```
Compile from source (requirement rust installed via rustup):
```shell
cargo install --locked --path . 
```
Make sure `~/.cargo/bin` is on the $PATH