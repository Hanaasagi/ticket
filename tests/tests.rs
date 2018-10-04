extern crate ticket;

use std::thread;
use ticket::Ticket;


#[test]
fn test_unique() {
    let mut generator = Ticket::new();
    let id_1 = generator.gen();
    let id_2 = generator.gen();
    assert_ne!(id_1, id_2);
}


#[test]
fn test_id_component() {
    let id_1 = Ticket::new().gen().as_bytes();
    let id_2 = Ticket::new().gen().as_bytes();

    // machine id
    assert_eq!(id_1[4 .. 7], id_2[4 .. 7]);
    // pid
    assert_eq!(id_1[7 .. 9], id_2[7 .. 9]);
}


#[test]
fn test_thread_safe() {
    let mut generator = Ticket::new();
    let id_1 = generator.gen();
    let id_2 = thread::spawn(move || {
        generator.gen()
    }).join().unwrap();

    assert_ne!(id_1, id_2);
    // only id counter will be different.
    assert_eq!(id_1.as_bytes()[0 .. 9], id_2.as_bytes()[0 .. 9]);
}
