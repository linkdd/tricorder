+++
title = "howto inventory"
documentName = "Inventory file"
description = "Syntax of the inventory file and dynamic inventories"
menuHref = "/tutorials/"
weight = 2
+++

{{< wip >}}
# Example of static inventory file: 
```toml
[[hosts]]
id = "testserver"
tags = ["server", "test"]
address = "192.168.178.6:22"
user = "testuser"
vars = { 
    module_ping = { 
        gateway = "192.168.178.1", internet = "8.8.8.8" 
    } 
}

[[hosts]]
id = "production"
tags = ["server", "database", "webserver"]
address = "192.168.178.7:22"
user = "produsesr"
```