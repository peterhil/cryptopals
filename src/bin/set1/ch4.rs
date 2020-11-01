#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::HEXLOWER;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use cryptopals::io::{exit_err};
use cryptopals::xor;


fn ch4() -> io::Result<()> {
    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/4.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));
    let file = File::open(full_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(hex) = line {
            let secret = HEXLOWER.decode(hex.as_bytes()).unwrap();
            xor::decrypt_single_byte(&secret, 1);
        }
    }

    Ok(())
}


fn main() {
    println!("Ch4:");
    match ch4() {
        Ok(res) => {
            res
        },
        Err(err) => {
            exit_err(err, 2)
        }
    }
}
