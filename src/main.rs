use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let mut f = BufReader::new(File::open("./src/input.txt").expect("open failed"));

    let mut buf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("Character: {}", c);
        }
        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }
}