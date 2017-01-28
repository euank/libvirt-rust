# libvirt-rs 
[![Build Status](https://travis-ci.org/euank/libvirt-rust.svg?branch=master)](https://travis-ci.org/euank/libvirt-rust)

## Overview
Libvirt bindings for rust. This is in WIP but most of the common functions that are used by us is available here.

Note: Everything is subject to change and not every function is guarantee to be stable.

```rust
extern crate libvirt;

use libvirt::connection::{Connection, ConnectionType};

fn main() {
    let conn: Connection = match Connection::new("qemu:///system".to_string(), ConnectionType::READONLY) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to hypervisor: {}", e.message)
    };

    println!("Connected to hypervisor");

    match conn.close() {
        Ok(()) => println!("Disconnected from hypervisor"),
        Err(e) => panic!("Failed to disconnect from hypervisor: {}", e.message)
    }

}

```


## Requirements

For this to work you will need libvirt 1.2.2 or higher as well as the development libraries (especially if you plan on adding additional functions). Please know that we cannot promise that this binding will work with any older version of libvirt, nor will we add support for any older version of libvirt.


For Ubuntu/Debian: `libvirt-dev`
For RHEL/Centos/Fedora: `libvirt-devel`
For Archlinux: `libvirt`

## Contributing

All are welcome to fork and submit PR towards this binding. In addition to making a PR to add new features, please add a unit test. It is not required but would explain the function would work.
