#![warn(clippy::all, rust_2018_idioms)]

use aes::Aes128;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;
use data_encoding::{BASE64};
use std::env;

use cryptopals::ascii;
use cryptopals::io::{exit_err, read_lines_by};

// create an alias for convenience
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

fn base64_decode(base64: &str) -> Vec<u8> {
    return BASE64
        .decode(&base64.as_bytes()[..])
        .unwrap_or_else(|e| exit_err(e, 3));
}

fn get_secret() -> Vec<u8> {
    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/7.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));
    let secret = read_lines_by(&full_path, base64_decode).unwrap();

    return secret;
}

fn ch7() {
    let key: [u8; 16] = *b"YELLOW SUBMARINE";
    let iv: [u8; 0] = Default::default();  // Ecb ignores iv
    let ciphertext: Vec<u8> = get_secret();

    let cipher = Aes128Ecb::new_var(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    println!("{:?}", ascii::print(decrypted_ciphertext));
}

fn main() {
    println!("Ch7:");
    ch7();
}
