use std::{fs::File, io::{BufReader, BufRead}};

use crate::challenge_3::crack_sbx_bytes;

pub fn find_xor(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut highscore: u32 = u32::MAX;
    let mut best_crack: String = String::new();

    for line in reader.lines() {
        let line_bytes = hex::decode(line.expect("unable to read line from dictionary file")).unwrap();
        let attempt = crack_sbx_bytes(&line_bytes);
        if attempt.1 < highscore {
            highscore = attempt.1;
            best_crack = attempt.0;
        }
    }
    best_crack
}