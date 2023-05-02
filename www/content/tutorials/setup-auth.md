+++
title = "howto auth"
documentName = "SSH authentication"
description = "Setup SSH server and ssh-agent configuration"
menuHref = "/tutorials/"
weight = 3
+++

{{< wip >}}
# Client
## install using apt
```shell
$ sudo apt update
$ sudo apt install openssh-client
```
## install using pacman
```shell
$ sudo pacman -Sy openssh
```


## generate and use ssh-keypair
generate keypair using following command.
note: you can rename the key with the `-f` flag
```shell
$ ssh-keygen -t ed25519
```

this generates a keypair (public and private key) under `~/.ssh` 

you can upload the key with following command
```shell
$ ssh-copy-id -i ~/.ssh/id_ed25519 <user>@<host>
```
or copy the content of the public key (.pub) to the hosts authorized_keys file (~/.ssh/authorized_keys)

## add key to ssh-agent
following command adds your key to your agent 
```shell
ssh-add ~/.ssh/id_ed25519
```