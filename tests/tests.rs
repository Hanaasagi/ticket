extern crate ticket;

use std::thread;
use ticket::{decode, encode, Ticketing, ID};

#[test]
fn test_unique() {
    let mut generator = Ticketing::new();
    let id_1 = generator.gen();
    let id_2 = generator.gen();
    assert_ne!(id_1, id_2);
}

#[test]
fn test_id_component() {
    let id_1 = Ticketing::new().gen().as_bytes();
    let id_2 = Ticketing::new().gen().as_bytes();

    // machine id
    assert_eq!(id_1[4..7], id_2[4..7]);
    // pid
    assert_eq!(id_1[7..9], id_2[7..9]);
}

#[test]
fn test_thread_safe() {
    let mut generator = Ticketing::new();
    let id_1 = generator.gen();
    let id_2 = thread::spawn(move || generator.gen()).join().unwrap();

    assert_ne!(id_1, id_2);
    // only id counter will be different.
    assert_eq!(id_1.as_bytes()[0..9], id_2.as_bytes()[0..9]);
}

#[test]
fn test_encode() {
    let id: ID = ID::new([91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249]);
    assert_eq!(encode(id), "bekc04rrtf01j8cpunsg");
}

#[test]
fn test_decode() {
    let id: ID = ID::new([91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249]);
    assert_eq!(decode(&"bekc04rrtf01j8cpunsg".to_string()), id);
}

#[test]
fn test_encode_and_decode() {
    for _ in 0..100 {
        let id = Ticketing::new().gen();
        assert_eq!(decode(&encode(id)), id);
    }
}
