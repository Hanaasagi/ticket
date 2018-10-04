mod ticket;
mod id;

extern crate rand;
extern crate time;
extern crate md5;
extern crate machine_uid;
#[macro_use]
extern crate lazy_static;

pub use ticket::{
    Ticket,
    encode,
    decode
};

pub use id:: {
    ID
};

const RAW_LEN: usize = 12; // binary raw length
