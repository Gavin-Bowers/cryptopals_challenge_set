mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_5;
mod challenge_6;
mod challenge_4;
mod challenge_9;
mod challenge_7;
mod challenge_8;

use crate::{
    challenge_1::hex_to_base64,
    challenge_2::fixed_xor,
    challenge_3::crack_sbx,
    challenge_4::find_xor,
    challenge_5::repeating_key_xor, 
    challenge_6::break_repeating_key_xor, challenge_9::pkcs7_padding};

fn main() {
    
    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 1
    ////////////////////////////////////////////////////////////////
    
    let s1_p1_hex_value_1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("\nThe hex value {} in base 64 is: {}",
        s1_p1_hex_value_1, hex_to_base64(s1_p1_hex_value_1));

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 2
    ////////////////////////////////////////////////////////////////
    
    let s1_p2_hex_value_1 = "1c0111001f010100061a024b53535009181c";
    let s1_p2_hex_value_2 = "686974207468652062756c6c277320657965";
    println!("\nThe XOR combination of the values is: {}",
        fixed_xor(s1_p2_hex_value_1, s1_p2_hex_value_2));

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 3
    ////////////////////////////////////////////////////////////////
    
    let s1_p3_hex_value_1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let s1_p3_crack: String = crack_sbx(s1_p3_hex_value_1).0;
    println!("\nThe crack of the hex string is : {}", s1_p3_crack);

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 4
    ////////////////////////////////////////////////////////////////

    let s1_p4_crack: String = find_xor("4.txt");
    println!("\nThe xor\'d text hidden in 4.txt is: {}", s1_p4_crack);

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 5
    ////////////////////////////////////////////////////////////////

    let s1_p5_input: String = "Burning 'em, if you ain't quick and nimble
    I go crazy when I hear a cymbal".to_string();
    println!("\nThe repeating key xor with a key of ICE is: {}", repeating_key_xor(&s1_p5_input, "ICE"));

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 6
    ////////////////////////////////////////////////////////////////

    let s1_p6_file = "6.txt";
    println!("\nThe decrypted text of the file for problem 6 is: {}", break_repeating_key_xor(s1_p6_file));

    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 9
    ////////////////////////////////////////////////////////////////


    ////////////////////////////////////////////////////////////////
    //Set 1 Problem 8
    ////////////////////////////////////////////////////////////////
    


    //Set 2 --------------------------------------------------------

    ////////////////////////////////////////////////////////////////
    //Set 2 Problem 9
    ////////////////////////////////////////////////////////////////
    
    let s2_p9_text = "YELLOW SUBMARINE";
    println!("\nThe text for problem 7 with padding is: {} ", pkcs7_padding(s2_p9_text,25));

    ////////////////////////////////////////////////////////////////
    //Set 2 Problem 10
    ////////////////////////////////////////////////////////////////

    



}
