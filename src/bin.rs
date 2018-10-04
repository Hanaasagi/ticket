extern crate ticket;
use ticket::{Ticketing, encode};

fn main() {
    let id = Ticketing::new().gen();
    println!("{}", encode(id));
}
