use hex;

pub fn repeating_key_xor(text: &str, key: &str) -> String {

    let key_bytes = key.as_bytes();
    let text_bytes = text.as_bytes();

    let mut result: Vec<u8> = Vec::new();
    let mut i = 0;
    for byte in text_bytes.iter() {
        result.push(byte ^ key_bytes[i]);
        i += 1;
        if i >= key_bytes.len() {
            i = 0;
        }
    }
    let return_val = hex::encode(result);
    return_val
}