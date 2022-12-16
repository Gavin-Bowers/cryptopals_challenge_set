use hex;
use base64;

//No error handling...
pub fn hex_to_base64(value: &str) -> String {
    let return_val = base64::encode(hex::decode(value).unwrap());
    return_val
}