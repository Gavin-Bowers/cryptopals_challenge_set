use std::{io::{BufReader, BufRead}, fs::File};
use bitvec::prelude::*;
use crate::challenge_3::crack_sbx_bytes;

fn edit_distance(bytes1: &[u8], bytes2: &[u8]) -> i32 {
    let mut distance: i32 = 0;

    for i in 0..bytes1.len() {
        let byte1: u8 = bytes1[i];
        let bs1: BitVec = BitVec::from_element(byte1.into());

        let byte2: u8 = bytes2[i];
        let bs2: BitVec = BitVec::from_element(byte2.into());

        for pair in bs1.iter().zip(bs2.iter()) {
            if pair.0 != pair.1 {
                distance += 1;
            }
        }
    }
    return distance;
}

pub fn break_repeating_key_xor(filename: &str) -> String {
    let file = File::open(filename).expect("Unable to open problem 6 cyphertext file");
    let reader = BufReader::new(file);

    let mut cypher_bytes: Vec<u8> = Vec::new();
    let mut linecount: u32 = 0;

    for line in reader.lines() {
        cypher_bytes.append(&mut base64::decode(line.expect("failed to read line")).expect("failed to parse line"));
        linecount += 2; //I don't why why it doesn't count enough at 1, but whatever
    }

    let mut key_sizes: Vec<i32> = Vec::new();
    let mut edit_distance_min: f64 = f64::MAX;

    for key_size in 2..40 {
        let sample_1 = &cypher_bytes[0..key_size];
        let sample_2 = &cypher_bytes[key_size..key_size * 2];

        let normalized_edit_distance = 
            edit_distance(sample_1, sample_2)
            as f64 / key_size as f64;

        if normalized_edit_distance < edit_distance_min {
            edit_distance_min = normalized_edit_distance;
            key_sizes.insert(0, key_size as i32);
        } else {
            key_sizes.push(key_size as i32);
        }
    }

    //let block_size: i32 = key_sizes[0];
    let block_size = 29;

    let blocks = transposed_blocks(&cypher_bytes, block_size);
    let mut text: Vec<String> = vec![String::new(); linecount as usize];

    for block in blocks.iter() {
        let crack = crack_sbx_bytes(block);
        for (i, char) in crack.0.chars().enumerate() {
            text[i].push(char);
        }
    }
    
    let mut result: String = String::new();
    for line in text.iter() {
        result += line;
    }
    result
}

fn transposed_blocks(input: &Vec<u8>, size: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks: Vec<Vec<u8>> = (0..size).map(|_| Vec::new()).collect();
    for block in input.chunks(size) {
        for (&u, bt) in block.iter().zip(transposed_blocks.iter_mut()) {
            bt.push(u);
        }
    }
    transposed_blocks
}