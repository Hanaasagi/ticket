# Ticket

[![Build Status](https://travis-ci.org/Hanaasagi/ticket.svg?branch=master)](https://travis-ci.org/Hanaasagi/ticket)
![](https://img.shields.io/badge/version-beta-EB6EA5.svg)

```
   _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
  ）                                （
---                                  ---
  9   _____ ___ ___ _  _____ _____   7
  6  |_   _|_ _/ __| |/ / __|_   _|  1
  3    | |  | | (__| ' <| _|  | |    6
  8    |_| |___\___|_|\_\___| |_|    5
  0                                  2
---                                  ---
  )_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _（

```

### About Ticket

Ticket is an Unique-ID-Generator.

The ID generated from Ticket only occupies 12 bytes.

- 4-byte from unix timestamp,
- 3-byte from machine id(`/sys/class/dmi/id/product_uuid`),
- 2-byte from current process id, and
- 3-byte counter which starting with a random value.


### Usage

Put this in your `Cargo.toml`:

```Toml
[dependencies]
ticket = { git = "https://github.com/Hanaasagi/ticket" }
```

Use it like following

```Rust

extern crate ticket;
use ticket::{Ticket, encode, decode};

fn main() {
    // create a `Ticket` to generate ticket number.
    let id: [u8; 12] = Ticket::new().gen();
    println!("{:?}", id);  // [91, 168, 206, 39, 123, 235, 192, 35, 15, 80, 249, 118]

    // encode and dedode
    println!("{}", encode(id));  // "bekcs9rrtf0263qgv5r0"
    assert_eq!(decode(encode(id)), id)
}

```

### License
[BSD 3-Clause License](LICENSE) Copyright (c) 2018, Hanaasagi
