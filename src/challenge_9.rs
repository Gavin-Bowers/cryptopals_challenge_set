
pub fn pkcs7_padding(text: &str, block_size: u32) -> String {
    let mut bytes: Vec<u8> = text.as_bytes().to_vec();
    if block_size < bytes.len() as u32 {
        println!("Block size is smaller than the text: no padding applied");
        return String::from_utf8(bytes).unwrap()
    } else {
        let filler_byte = "\x04".as_bytes()[0];
        let mut filler = vec![filler_byte; (block_size - bytes.len() as u32).try_into().unwrap()];
        bytes.append(&mut filler);

        //println!("The padded bytes are: {:?}", bytes);
        return String::from_utf8(bytes).unwrap();
    }
}