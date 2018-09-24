use rand;
use time;
use md5;
use std::fs::File;
use std::process;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

//   - 4-byte value representing the seconds since the Unix epoch,
//   - 3-byte machine identifier,
//   - 2-byte process id, and
//   - 3-byte counter, starting with a random value.


const ENCODED_LEN: usize = 20;  // string encoded length
const RAW_LEN: usize = 12; // binary raw length
const ENCODING: &str =  "0123456789abcdefghijklmnopqrstuv";

type ID = [u8; RAW_LEN];

struct Ticket {
    object_id_counter: AtomicUsize,
    machine_id: String,
    pid: u32,
    dec: [u8; 256]
}

impl Ticket {

    pub fn new() -> Ticket {
        let object_id_counter = AtomicUsize::new(rand::random::<usize>());
        let machine_id = get_machine_id();
        let pid = process::id();
        let mut dec = [0xFFu8; 256];
        for (index, &chr) in ENCODING.chars().collect::<Vec<char>>().iter().enumerate() {
            dec[chr as usize] = index as u8;
        }

        return Ticket {
            object_id_counter: object_id_counter,
            machine_id: machine_id,
            pid: pid,
            dec: dec,
        }

    }

    pub fn gen(&mut self) -> ID {
        let mut id: ID;
        return self.get_with_time(time::now_utc());
    }

    pub fn get_with_time(&mut self, t: time::Tm) -> ID {
        let sec = t.to_timespec().sec as u32;
        let digest = md5::compute(self.machine_id.as_bytes());
        let count = self.object_id_counter.fetch_add(1, Ordering::SeqCst) as u32;

        let mut id: ID = [0; 12];

        id[0] = ((sec >> 24) & 0xff) as u8;
        id[1] = ((sec >> 16) & 0xff) as u8;
        id[2] = ((sec >> 8) & 0xff) as u8;
        id[3]= (sec & 0xff) as u8;
        id[4] = digest[0];
        id[5] = digest[1];
        id[6] = digest[2];
        id[7] = (self.pid >> 8 & 0xff) as u8;
        id[8] = (self.pid & 0xff) as u8;
        id[9] = (count >> 16 & 0xff) as u8;
        id[10] = (count >> 8 & 0xff) as u8;
        id[11] = (count & 0xff) as u8;
        return id;
    }
}


fn get_machine_id() -> String {
    // only work in linux
    let file_path = "/sys/class/dmi/id/product_uuid";
    let mut f = File::open(file_path)
        .expect(&format!("{} not found", file_path));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("can't read {}", file_path));
    // f will automatically closed when they go out of scope.
    return contents;
}
