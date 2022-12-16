use hex;

pub fn fixed_xor(buffer_1: &str, buffer_2: &str) -> String {
    let buffer_1_bytes = hex::decode(buffer_1).unwrap();
    let buffer_2_bytes = hex::decode(buffer_2).unwrap();
    let xored_bytes: Vec<u8> = 
        buffer_1_bytes.iter().zip(buffer_2_bytes.iter())
        .map(|(a,b)| a ^ b).collect();
    hex::encode(xored_bytes)
}