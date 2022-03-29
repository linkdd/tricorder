# tricorder

Automation the [KISS](https://en.wikipedia.org/wiki/KISS_principle) way.

[![Crates.io](https://img.shields.io/crates/v/tricorder?style=flat-square)](https://crates.io/crates/tricorder)
[![Crates.io](https://img.shields.io/crates/l/tricorder?style=flat-square)](https://crates.io/crates/tricorder)
[![Crates.io](https://img.shields.io/crates/d/tricorder?style=flat-square)](https://crates.io/crates/tricorder)
[![docs.rs](https://img.shields.io/docsrs/tricorder?style=flat-square)](https://docs.rs/tricorder)

## Introduction

[Ansible](https://ansible.com) is a great tool for automation. But it suffers
from the same problem of many such tools: a big pile of custom YAML DSL.

YAML is used to provide a declarative syntax of your automated workflow. This is
nice for simple use cases, but automation can become rather complex very
quickly.

Then those tools implement control flow structures (conditional execution,
loops, parallelization, ...), then the ability to save values into variables.

Before you know it, you're programming in YAML. And the developer experience of
such a language is terrible.

**tricorder** aims to fix this. It gives you a single tool to perform tasks on
multiple remotes. You then use your common UNIX tools like `bash`, `jq`, `curl`,
etc... to compose those tasks together.

## Usage

Just like *Ansible*, **tricorder** uses an inventory file, listing the hosts
to connect to:

```toml
[[hosts]]

id = "backend"
tags = ["server", "backend", "myapp"]
address = "10.0.1.10:22"
user = "admin"

[[hosts]]

id = "frontend"
tags = ["server", "frontend", "myapp"]
address = "10.0.1.20:22"
user = "admin"
```

> **NB:** The inventory is either a TOML file or an executable producing a JSON
> output. This way you can create dynamic inventories by querying a remote
> service or database.

Then, run one of the following commands:

```
$ tricorder -i /path/to/inventory do -- echo "run on all hosts"
$ tricorder -i /path/to/inventory -H backend do -- echo "run on specific host"
$ tricorder -i /path/to/inventory -t server,myapp do -- echo "run on all hosts matching tags"
```

> **NB:** Authentication is done via `ssh-agent` only.

Every logging messages is written on `stderr`, the command result for each host
is written as a JSON document on `stdout`:

```json
[
  {
    "host": "backend",
    "success": false,
    "error": "..."
  },
  {
    "host": "frontend",
    "success": true,
    "info": {
      "exit_code": 0,
      "output": "..."
    }
  }
]
```

This way, you can compose this tool with `jq` to extract the relevant informations
in your scripts.

## Usage with the Rust API

**tricorder** is also available as a Rust crate to include it directly in your
software:

```rust
use tricorder::core::{Inventory, Host};
use tricorder::tasks::{TaskRunner, exec};
use serde_json::json;

let inventory = Inventory::new()
  .add_host(
    Host::new("localhost", "localhost:22")
      .set_user("root")
      .add_tag("local")
      .set_var("msg", json!("hello"))
  );

let task = exec::Task::new("echo \"{host.id} says {host.vars.msg}\"");
let result = inventory.hosts.run_task_seq(&task).unwrap();

println!("{}", result);
```

## Documentation

For more informations, consult the [documentation](https://docs.rs/tricorder).

## Roadmap

Checkout the [Bug Tracker](https://github.com/linkdd/tricorder/issues).

## License

This software is released under the terms of the [MIT License](./LICENSE.txt).
