# Ticket

Unique-ID-Generator inspired by [rs/xid](https://github.com/rs/xid).

[![ticket on Travis CI][travis-image]][travis]
[![ticket on crates.io][cratesio-image]][cratesio]
[![ticket on docs.rs][docsrs-image]][docsrs]

[travis-image]: https://travis-ci.org/Hanaasagi/ticket.svg?branch=master
[travis]: https://travis-ci.org/Hanaasagi/ticket
[cratesio-image]: https://img.shields.io/crates/v/ticket.svg
[cratesio]: https://crates.io/crates/ticket
[docsrs-image]: https://docs.rs/ticket/badge.svg
[docsrs]: https://docs.rs/ticket

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

The ID generated from Ticket only occupies 12 bytes.

- 4-byte from unix timestamp,
- 3-byte from machine id,
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
use ticket::{Ticketing, encode, decode};


fn main() {
    // create a `Ticketing` to generate ticket number.
    let id = Ticketing::new().gen();

    // using base32 encoding.
    println!("{}", id);  // "bekcs9rrtf0263qgv5r0"

    // as 12 bytes array.
    println!("{:?}", id.as_bytes());  // [91, 168, 206, 39, 123, 235, 192, 35, 15, 80, 249, 118]

    // encode and dedode
    assert_eq!(decode(&encode(id)), id);
}
```

### License
[BSD 3-Clause License](https://github.com/Hanaasagi/ticket/blob/master/LICENSE) Copyright (c) 2018, Hanaasagi
