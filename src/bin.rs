extern crate ticket;
use ticket::{Ticket, encode};

fn main() {
    let id = Ticket::new().gen();
    println!("{}", encode(id));
}
