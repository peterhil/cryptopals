#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::HEXLOWER;

use cryptopals::xor;

fn ch3() {
    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_vec();
    let secret: Vec<u8> = HEXLOWER.decode(&hex).unwrap();

    xor::decrypt_single_byte(&secret, 1);
}

fn main() {
    println!("Ch3:");
    ch3();
}
