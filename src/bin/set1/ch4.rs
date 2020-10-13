#![warn(clippy::all, rust_2018_idioms)]

use std::env;

use cryptopals::io::{exit_err, read_lines};
use cryptopals::xor;

fn ch4() {
    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/4.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));

    match read_lines(full_path) {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(hex) = line {
                    xor::decrypt_single_byte(&hex);
                }
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
        },
    }
}

fn main() {
    println!("Ch4:");
    ch4();
}
