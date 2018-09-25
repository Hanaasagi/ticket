use rand;
use time;
use md5;
use std::str;
use std::fs::File;
use std::process;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

///   - 4-byte value representing the seconds since the Unix epoch,
///   - 3-byte machine identifier,
///   - 2-byte process id, and
///   - 3-byte counter, starting with a random value.

const ENCODED_LEN: usize = 20;  // string encoded length
const RAW_LEN: usize = 12; // binary raw length
const ENCODING: &str =  "0123456789abcdefghijklmnopqrstuv";

lazy_static! {
    static ref DECODING: [u8; 256] = {
        let mut dec = [0xFFu8; 256];
        for (index, &chr) in ENCODING.as_bytes().iter().enumerate() {
            dec[chr as usize] = index as u8;
        }
        dec
    };

    static ref MACHINE_ID: String = get_machine_id();
}

type ID = [u8; RAW_LEN];

pub struct Ticket {
    object_id_counter: AtomicUsize,
    machine_id: md5::Digest,
    pid: u32,
}

impl Default for Ticket {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticket {

    pub fn new() -> Self {
        let object_id_counter = AtomicUsize::new(rand::random::<usize>());
        let machine_id = md5::compute(MACHINE_ID.as_bytes());
        let pid = process::id();
        Ticket {
            object_id_counter,
            machine_id,
            pid,
        }

    }

    pub fn gen(&mut self) -> ID {
        self.get_with_time(time::now_utc())
    }

    fn get_with_time(&mut self, t: time::Tm) -> ID {
        let sec = t.to_timespec().sec as u32;
        let count = self.object_id_counter.fetch_add(1, Ordering::SeqCst) as u32;

        let mut id: ID = [0; 12];
        id[0]  = ((sec >> 24)   & 0xFF) as u8;
        id[1]  = ((sec >> 16)   & 0xFF) as u8;
        id[2]  = ((sec >> 8)    & 0xFF) as u8;
        id[3]  = (sec           & 0xFF) as u8;
        id[4]  = self.machine_id[0];
        id[5]  = self.machine_id[1];
        id[6]  = self.machine_id[2];
        id[7]  = (self.pid >> 8 & 0xFF) as u8;
        id[8]  = (self.pid      & 0xFF) as u8;
        id[9]  = (count >> 16   & 0xFF) as u8;
        id[10] = (count >> 8    & 0xFF) as u8;
        id[11] = (count         & 0xFF) as u8;

        id
    }
}


fn get_machine_id() -> String {
    // only work in linux
    let file_path = "/sys/class/dmi/id/product_uuid";
    let mut f = File::open(file_path)
        .unwrap_or_else(|_| panic!("{} not found", file_path));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("can't read {}", file_path));
    // f will automatically closed when they go out of scope.
    contents
}


pub fn encode(id: ID) -> String {
    let encoding = ENCODING.as_bytes();
    let mut result = [0u8; ENCODED_LEN];
	result[0]  = encoding[( id[0]  >> 3) as usize];
	result[1]  = encoding[((id[1]  >> 6) & 0x1F | (id[0] << 2) & 0x1F) as usize];
	result[2]  = encoding[((id[1]  >> 1) & 0x1F) as usize];
	result[3]  = encoding[((id[2]  >> 4) & 0x1F | (id[1] << 4) & 0x1F) as usize];
	result[4]  = encoding[((id[3]  >> 7) | (id[2] << 1) & 0x1F) as usize];
	result[5]  = encoding[((id[3]  >> 2) & 0x1F) as usize];
	result[6]  = encoding[((id[4]  >> 5) | (id[3] << 3) & 0x1F) as usize];
	result[7]  = encoding[( id[4]  &  0x1F) as usize];
	result[8]  = encoding[( id[5]  >> 3) as usize];
	result[9]  = encoding[((id[6]  >> 6) & 0x1F | (id[5] << 2) & 0x1F) as usize];
	result[10] = encoding[((id[6]  >> 1) & 0x1F ) as usize];
	result[11] = encoding[((id[7]  >> 4) & 0x1F | (id[6] << 4 ) & 0x1F) as usize];
	result[12] = encoding[((id[8]  >> 7) | (id[7] << 1) & 0x1F) as usize];
	result[13] = encoding[((id[8]  >> 2) & 0x1F) as usize];
	result[14] = encoding[((id[9]  >> 5) | (id[8] << 3) & 0x1F) as usize];
	result[15] = encoding[( id[9]  &  0x1F) as usize];
	result[16] = encoding[( id[10] >> 3) as usize];
	result[17] = encoding[((id[11] >> 6) & 0x1F | (id[10] << 2) & 0x1F) as usize];
	result[18] = encoding[((id[11] >> 1) & 0x1F) as usize];
    result[19] = encoding[((id[11] << 4) & 0x1F) as usize];

    str::from_utf8(&result).unwrap().to_string()
}

#[test]
fn test_encode() {
    let id: ID = [91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249];
    assert_eq!(encode(id), "bekc04rrtf01j8cpunsg");
}


pub fn decode(s: &str) -> ID {
    let dec = *DECODING;
    let s = s.as_bytes();
    let mut id: ID = [0u8; RAW_LEN];
    id[0]  = dec[s[0]  as usize] << 3 | dec[s[1]  as usize] >> 2;
    id[1]  = dec[s[1]  as usize] << 6 | dec[s[2]  as usize] << 1 | dec[s[3]  as usize] >> 4;
    id[2]  = dec[s[3]  as usize] << 4 | dec[s[4]  as usize] >> 1;
    id[3]  = dec[s[4]  as usize] << 7 | dec[s[5]  as usize] << 2 | dec[s[6]  as usize] >> 3;
    id[4]  = dec[s[6]  as usize] << 5 | dec[s[7]  as usize];
    id[5]  = dec[s[8]  as usize] << 3 | dec[s[9]  as usize] >> 2;
    id[6]  = dec[s[9]  as usize] << 6 | dec[s[10] as usize] << 1 | dec[s[11] as usize] >> 4;
    id[7]  = dec[s[11] as usize] << 4 | dec[s[12] as usize] >> 1;
    id[8]  = dec[s[12] as usize] << 7 | dec[s[13] as usize] << 2 | dec[s[14] as usize] >> 3;
    id[9]  = dec[s[14] as usize] << 5 | dec[s[15] as usize];
    id[10] = dec[s[16] as usize] << 3 | dec[s[17] as usize] >> 2;
    id[11] = dec[s[17] as usize] << 6 | dec[s[18] as usize] << 1 | dec[s[19] as usize] >> 4;
    id
}

#[test]
fn test_decode() {
    let id: ID = [91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249];
    assert_eq!(decode(&"bekc04rrtf01j8cpunsg".to_string()), id);
}

#[test]
fn test_encode_and_decode() {
    for _ in 0..100 {
        let id = Ticket::new().gen();
        assert_eq!(decode(&encode(id)), id);
    }
}
