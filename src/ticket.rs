use rand;
use time;
use md5;
use machine_uid;
use id::{ID};
use std::str;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};

///   - 4-byte value representing the seconds since the Unix epoch,
///   - 3-byte machine identifier,
///   - 2-byte process id, and
///   - 3-byte counter, starting with a random value.

const ENCODED_LEN: usize = 20;  // string encoded length
const ENCODING: &str =  "0123456789abcdefghijklmnopqrstuv";

lazy_static! {
    static ref DECODING: [u8; 256] = {
        let mut dec = [0xFFu8; 256];
        for (index, &chr) in ENCODING.as_bytes().iter().enumerate() {
            dec[chr as usize] = index as u8;
        }
        dec
    };

    static ref MACHINE_ID: String = machine_uid::get()
        .unwrap_or_else(|_| panic!("could not get machine id."));
}


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

        let mut raw = [0u8; ::RAW_LEN];
        raw[0]  = ((sec >> 24)   & 0xFF) as u8;
        raw[1]  = ((sec >> 16)   & 0xFF) as u8;
        raw[2]  = ((sec >> 8)    & 0xFF) as u8;
        raw[3]  = (sec           & 0xFF) as u8;
        raw[4]  = self.machine_id[0];
        raw[5]  = self.machine_id[1];
        raw[6]  = self.machine_id[2];
        raw[7]  = (self.pid >> 8 & 0xFF) as u8;
        raw[8]  = (self.pid      & 0xFF) as u8;
        raw[9]  = (count >> 16   & 0xFF) as u8;
        raw[10] = (count >> 8    & 0xFF) as u8;
        raw[11] = (count         & 0xFF) as u8;

        ID::new(raw)
    }
}


pub fn encode(id: ID) -> String {
    let encoding = ENCODING.as_bytes();
    let raw = id.as_bytes();
    let mut result = [0u8; ENCODED_LEN];
	result[0]  = encoding[( raw[0]  >> 3) as usize];
	result[1]  = encoding[((raw[1]  >> 6) & 0x1F | (raw[0] << 2) & 0x1F) as usize];
	result[2]  = encoding[((raw[1]  >> 1) & 0x1F) as usize];
	result[3]  = encoding[((raw[2]  >> 4) & 0x1F | (raw[1] << 4) & 0x1F) as usize];
	result[4]  = encoding[((raw[3]  >> 7) | (raw[2] << 1) & 0x1F) as usize];
	result[5]  = encoding[((raw[3]  >> 2) & 0x1F) as usize];
	result[6]  = encoding[((raw[4]  >> 5) | (raw[3] << 3) & 0x1F) as usize];
	result[7]  = encoding[( raw[4]  &  0x1F) as usize];
	result[8]  = encoding[( raw[5]  >> 3) as usize];
	result[9]  = encoding[((raw[6]  >> 6) & 0x1F | (raw[5] << 2) & 0x1F) as usize];
	result[10] = encoding[((raw[6]  >> 1) & 0x1F ) as usize];
	result[11] = encoding[((raw[7]  >> 4) & 0x1F | (raw[6] << 4 ) & 0x1F) as usize];
	result[12] = encoding[((raw[8]  >> 7) | (raw[7] << 1) & 0x1F) as usize];
	result[13] = encoding[((raw[8]  >> 2) & 0x1F) as usize];
	result[14] = encoding[((raw[9]  >> 5) | (raw[8] << 3) & 0x1F) as usize];
	result[15] = encoding[( raw[9]  &  0x1F) as usize];
	result[16] = encoding[( raw[10] >> 3) as usize];
	result[17] = encoding[((raw[11] >> 6) & 0x1F | (raw[10] << 2) & 0x1F) as usize];
	result[18] = encoding[((raw[11] >> 1) & 0x1F) as usize];
    result[19] = encoding[((raw[11] << 4) & 0x1F) as usize];

    str::from_utf8(&result).unwrap().to_string()
}

#[test]
fn test_encode() {
    let id: ID = ID::new([91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249]);
    assert_eq!(encode(id), "bekc04rrtf01j8cpunsg");
}


pub fn decode(s: &str) -> ID {
    let dec = *DECODING;
    let s = s.as_bytes();
    let mut raw = [0u8; ::RAW_LEN];
    raw[0]  = dec[s[0]  as usize] << 3 | dec[s[1]  as usize] >> 2;
    raw[1]  = dec[s[1]  as usize] << 6 | dec[s[2]  as usize] << 1 | dec[s[3]  as usize] >> 4;
    raw[2]  = dec[s[3]  as usize] << 4 | dec[s[4]  as usize] >> 1;
    raw[3]  = dec[s[4]  as usize] << 7 | dec[s[5]  as usize] << 2 | dec[s[6]  as usize] >> 3;
    raw[4]  = dec[s[6]  as usize] << 5 | dec[s[7]  as usize];
    raw[5]  = dec[s[8]  as usize] << 3 | dec[s[9]  as usize] >> 2;
    raw[6]  = dec[s[9]  as usize] << 6 | dec[s[10] as usize] << 1 | dec[s[11] as usize] >> 4;
    raw[7]  = dec[s[11] as usize] << 4 | dec[s[12] as usize] >> 1;
    raw[8]  = dec[s[12] as usize] << 7 | dec[s[13] as usize] << 2 | dec[s[14] as usize] >> 3;
    raw[9]  = dec[s[14] as usize] << 5 | dec[s[15] as usize];
    raw[10] = dec[s[16] as usize] << 3 | dec[s[17] as usize] >> 2;
    raw[11] = dec[s[17] as usize] << 6 | dec[s[18] as usize] << 1 | dec[s[19] as usize] >> 4;

    ID::new(raw)
}

#[test]
fn test_decode() {
    let id: ID = ID::new([91, 168, 192, 19, 123, 235, 192, 25, 161, 153, 245, 249]);
    assert_eq!(decode(&"bekc04rrtf01j8cpunsg".to_string()), id);
}

#[test]
fn test_encode_and_decode() {
    for _ in 0..100 {
        let id = Ticket::new().gen();
        assert_eq!(decode(&encode(id)), id);
    }
}
