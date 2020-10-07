#![warn(clippy::all, rust_2018_idioms)]

use relative_path::RelativePath;
use std::env;

use cryptopals::encoding;
use cryptopals::io::{read_lines};
use cryptopals::xor;

fn ch4() -> Result<String, std::io::Error> {
    // File must exist in current path before this produces output
    let relative_path = RelativePath::new("./data/4.txt");
    let full_path = relative_path.to_path(env::current_dir()?.as_path());

    match read_lines(full_path) {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(hex) = line {
                    println!("{}", hex);
                    let secret = encoding::hex_decode(Vec::<u8>::from(hex));
                    xor::decrypt_single_byte(secret);
                }
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
        },
    }

    return Ok("Done!".to_string());
}

fn main() {
    println!("Ch4:");
    let res = ch4();
    println!("{}", res.unwrap())  // TODO Fix type of ch4;
}
