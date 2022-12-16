use hex;
use std::{char};
use std::collections::{HashMap};

// Source:
// Lee, E. Stewart. "Essays about Computer Security" (PDF). University of Cambridge Computer Laboratory. p. 181.
static EXPECTED_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];

fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}

fn get_character_counts(v: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();
    for &c in v.iter() {
        if is_control(c) {
            continue;
        }
        let key = if is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t' {
            b' '
        } else {
            b'.'
        };

        let count = counts.entry(key).or_insert(0f32);
        *count += 1f32;
    }
    counts
}

fn compute_score(v: &Vec<u8>) -> u32 {
    if !v.is_ascii() {
        return std::u32::MAX;
    }

    if v.iter().any(|&c| is_control(c) && c != b'\n') {
        return std::u32::MAX;
    }

    let counts = get_character_counts(v);
    let length = v.len() as f32;

    EXPECTED_FREQUENCIES.iter().fold(0 as f32, |a, &(c, score)| {
        let expected_count = score / 100 as f32 * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        a + (expected_count - actual_count).powi(2)
    }) as u32
}

fn single_byte_xor(bytes: &Vec<u8>, key: char) -> Vec<u8> {
    let key_byte: u8 = key as u8;

    let xored_bytes: Vec<u8> = bytes.iter().map(|a| a ^ key_byte).collect();
    xored_bytes
}

fn generate_cracks(bytes: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut crack_attempts: Vec<Vec<u8>> = Vec::new();

    for i in 0..char::MAX as u32 {
        let key = char::from_u32(i);
        if let Some(key) = key {
            if char::is_ascii(&key) {
                let crack_attempt: Vec<u8> = single_byte_xor(&bytes, key);
                crack_attempts.push(crack_attempt);
            }
        }
    }
    crack_attempts
}

fn single_byte_xor_crack(bytes: &Vec<u8>) -> (String, u32) {
    let mut highscore: u32 = u32::MAX;
    let mut best_crack: Vec<u8> = Vec::new();
    let cracks: Vec<Vec<u8>> = generate_cracks(&bytes);
    
    for crack_attempt in cracks.iter() {
        let score = compute_score(&crack_attempt);

        if score < highscore {
            highscore = score;
            best_crack = crack_attempt.to_vec();
        }
    }
    (String::from_utf8(best_crack).unwrap_or(
        "Error parsing crack".to_string()), highscore)
}

pub fn crack_sbx(hex_str: &str) -> (String, u32) {
    let bytes = hex::decode(hex_str).unwrap();
    crack_sbx_bytes(&bytes)
}

pub fn crack_sbx_bytes(bytes: &Vec<u8>) -> (String, u32) {
    single_byte_xor_crack(&bytes)
}

