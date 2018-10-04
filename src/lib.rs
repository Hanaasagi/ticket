//! Simple Unique-ID-Generator inspired by [rs/xid](https://github.com/rs/xid).
//!
//! The ID generated from Ticket only occupies 12 bytes.
//!
//! - 4-byte from unix timestamp,
//! - 3-byte from machine id,
//! - 2-byte from current process id, and
//! - 3-byte counter which starting with a random value.
//!
//! ### Usage
//!
//! ```Rust
//! extern crate ticket;
//! use ticket::{Ticketing, encode, decode};
//!
//! fn main() {
//!     // create a `Ticketing` to generate ticket number.
//!     let id: [u8; 12] = Ticketing::new().gen();
//!     println!("{:?}", id);  // [91, 168, 206, 39, 123, 235, 192, 35, 15, 80, 249, 118]
//!
//!     // encode and dedode
//!     println!("{}", encode(id));  // "bekcs9rrtf0263qgv5r0"
//!     assert_eq!(decode(&encode(id)), id)
//! }
//! ```
//!

mod ticket;
mod id;

extern crate rand;
extern crate time;
extern crate md5;
extern crate machine_uid;
#[macro_use]
extern crate lazy_static;

pub use ticket::{
    Ticketing,
    encode,
    decode
};

pub use id:: {
    ID
};

/// raw id length
const RAW_LEN: usize = 12;
