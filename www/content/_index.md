+++
title = "tricorder"
+++

# Automation the KISS way

{{< automation-kiss >}}

# No YAML involved

## First an inventory

```toml
[[hosts]]
id = "backend"
address = "10.0.1.10:22"
user = "root"
tags = ["server", "backend", "myapp"]
vars = { msg = "hi" }
```

## Then a command

```console
$ tricorder -i /path/to/inventory do -- echo "{host.id} says {host.vars.msg}"
```

## Finally, a JSON output

```json
[
  {
    "host": "backend",
    "success": true,
    "info": {
      "exit_code": 0,
      "stdout": "backend says hi\n",
      "stderr": ""
    }
  }
]
```

# Rust API

## Add dependency

```toml
tricorder = "0.9"
```

## Write your recipe

### First, import symbols

```rust
use tricorder::prelude::*;
use tricorder::tasks::exec;
use serde_json::json;
```

### Then, build your inventory

```rust
let inventory = Inventory::new()
  .add_host(
    Host::new(Host::id("localhost").unwrap(), "localhost:22".to_string())
      .set_user("root".to_string())
      .add_tag(Host::tag("local").unwrap())
      .set_var("msg".to_string(), json!("hello"))
      .to_owned()
  )
  .to_owned();
```

### Finally, run your tasks

```rust
let task = exec::Task::new("echo \"{host.id} says {host.vars.msg}\"".to_string());
```

**Sequentially:**

```rust
let result = inventory.hosts.run_task_seq(&task).unwrap();
```

**Or concurrently:**

```rust
let result = inventory.hosts.run_task_parallel(&task).unwrap();
```

The result is a `serde_json::Value`:

```rust
println!("{}", result);
```

## Build and run

```console
$ cargo run
```

# Backstory

{{< figure
  src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c5/12.5.12GeorgeTakeiByLuigiNovi10.jpg/440px-12.5.12GeorgeTakeiByLuigiNovi10.jpg"
  title="Actor George Takei autographs a tricorder"
>}}

[Ansible](https://ansible.com) is a great tool for automation. But it suffers
from the same problem of many such tools: a big pile of custom YAML DSL.

YAML is used to provide a declarative syntax of your automated workflow. This is
nice for simple use cases, but automation can become rather complex very
quickly.

Once those tools start implementing:

 - control flow structures (conditions, loops)
 - variable assignations
 - modules
 - package management
 - ...

Your YAML files become a programming language with terrible developer
experience.

**tricorder** aims to fix this. It gives you a single tool to perform tasks on
multiple remotes. You then use your common UNIX tools like `bash`, `jq`, `curl`,
etc... to compose those tasks together.

The name comes from [Star Trek's Tricorder](https://en.wikipedia.org/wiki/Tricorder),
a multifunction hand-held device to perform sensor environment scans, data
recording, and data analysis. Pretty much anything required by the plot.

The main goal of **tricorder** is to provide the basic tools to perform tasks on
remote hosts and get out of your way. Allowing you to integrate it with any
scripting language or programming language of your choice, instead of forcing
you to develop in a sub-par custom YAML DSL.

> Spock stared hard at his tricorder, as if by sheer will he might force it to
> tell him the answer to his questions.

# Reading resources

{{< button-group >}}
  {{< button label="Source Code" href="https://github.com/linkdd/tricorder" >}}
  {{< button label="API reference" href="https://docs.rs/tricorder/latest/tricorder/" >}}
  {{< button label="Tutorials" href="/tutorials/" >}}
{{< /button-group >}}
