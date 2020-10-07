#![warn(clippy::all, rust_2018_idioms)]

use cryptopals::ascii;
use cryptopals::encoding;
use cryptopals::xor;

fn ch3() {
    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let secret: Vec<u8> = encoding::hex_decode(hex.to_vec());

    println!("Hex: {:?}", ascii::print(hex.to_vec()));
    println!("Secret: {:?}", ascii::print(secret.to_vec()));

    xor::decrypt_single_byte(secret);
}

fn main() {
    println!("Ch3:");
    ch3();
}
