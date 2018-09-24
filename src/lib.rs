mod ticket;

extern crate rand;
extern crate time;
extern crate md5;
#[macro_use]
extern crate lazy_static;

pub use ticket::{
    Ticket,
    encode,
    decode
};
