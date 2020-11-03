#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::HEXLOWER;
use std::env;
use std::io;

use cryptopals::io::{exit_err, read_lines_by};
use cryptopals::xor;

fn xor_decrypt_hex(hex: &str) -> Vec<char> {
    let secret = HEXLOWER.decode(hex.as_bytes()).unwrap_or_else(|e| exit_err(e, 4));
    return xor::decrypt_single_byte(&secret, 1);
}

fn ch4() -> io::Result<Vec<char>> {
    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/4.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));

    return read_lines_by(&full_path, xor_decrypt_hex);
}

fn main() {
    println!("Ch4:");
    match ch4() {
        Ok(res) => {
            println!("{:?}", res);
        },
        Err(err) => {
            exit_err(err, 2)
        }
    }
}
