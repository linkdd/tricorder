+++
title = "howto cli"
documentName = "Command Line Interface"
description = "Flags and commands descriptions"
menuHref = "/tutorials/"
weight = 4
+++

{{< wip >}}

# tricorder
This is the executable. Here, you can control which hosts are affected by the subcommand.
You can select host from an inventory file by using the flag `--inventory`.

## Flags: 

| Flag | Description |
| - | - |
| -i --inventory \<FILE> | Path to TOML inventory file or program producing JSON inventory|
| -H --host_id \<STR>| Identifier of the host to connect to  |
| -t --host_tags \<STR> | Comma-separated list of tags identifying the hosts to connect to  |


# info (Subcommand)
Gather information on hosts

## Examples:
```shell
$ tricorder -i inventory info
```

## Flags: 

| Flag | Description |
| - | - |
| -p --parallel | If set, the task will be executed concurrently |

# do (Subcommand)
Execute a command on multiple hosts.

## Examples:
```shell
$ tricorder -i inventory do -- echo "run on all hosts"
$ tricorder -i inventory -H foo do -- echo "run only on host 'foo'"
$ tricorder -i inventory -t myapp do -- echo "run only on hosts tagged with 'myapp'"
```
Commands can be templated with data from the host as defined in the
inventory:
```shell
$ tricorder -i inventory do -- echo "{host.id} says {host.vars.msg}"
```

## Flags

| Flags | Description |
| - | - |
| -p --parallel | If set, the task will be executed concurrently |

# upload (Subcommand)
Upload a file to multiple remote hosts.

## Examples:
```shell
$ tricorder -i inventory upload LOCAL_PATH REMOTE_PATH [FILE_MODE]
$ tricorder -i inventory upload -T LOCAL_PATH REMOTE_PATH [FILE_MODE]
```

## Flags: 

| Flags | Description |
| - | - |
| -p --parallel | If set, the task will be executed concurrently |
| -T --template | If set, the task will be executed concurrently |
| [LOCAL_PATH] | Path on local host to the file to be uploaded |
| [REMOTE_PATH] | Path on remote host to upload the file |
| [MODE] (default: 0644) | UNIX file mode to set on the uploaded file |

# download (Subcommand)
Download a file from multiple remote hosts.
The files will be downloaded to: `{pwd}/{host.id}/{local_path}`

## Examples:
```shell
$ tricorder -i inventory download REMOTE_PATH LOCAL_PATH
```

## Flags:

| Flags | Description |
| - | - |
| -p --parallel | If set, the task will be executed concurrently |
| [LOCAL_PATH] | Path on local host to the file to be uploaded |
| [REMOTE_PATH] | Path on remote host to upload the file |

# module (Subcommand)
Upload a module to the remote host and call it with data.
Data can be a specified JSON data file. 
The data in the file will be overwritten by variables in `host.vars.module_<modulename>.`

A module is an executable, that gets uploaded to `~/.local/tricorder/<modulename>`. The Module reads the supplied data from stdin. 

You could also create a Module, that calls external sources like APIs or a database to get its data.

## Examples:
```shell
$ tricorder -i inventory module --data <DATA_FILE_PATH> --module <MODULE_PATH>
$ tricorder -i inventory module --module <MODULE_PATH>
```

## Flags:
| Flags | Description |
| - | - |
| -p --parallel | If set, the task will be executed concurrently |
| -d --data [DATA_PATH] | Path to the file containing the data in JSON-format |
| -m --module [MODULE_PATH] | Path to the executable that should be run |

