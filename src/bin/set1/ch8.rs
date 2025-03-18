#![warn(clippy::all, rust_2018_idioms)]

use aes::Aes128;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;
use data_encoding::HEXLOWER;
use std::env;
use std::io;

use cryptopals::ascii;
use cryptopals::io::{exit_err, read_iter};

// create an alias for convenience
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

fn decrypt(line: &str) -> Result<Vec<u8>, block_modes::BlockModeError> {
    let key: [u8; 16] = *b"YELLOW SUBMARINE";
    let iv: [u8; 0] = Default::default();  // Ecb ignores iv
    let cipher = Aes128Ecb::new_var(&key, &iv).unwrap();

    let secret = HEXLOWER.decode(&line.trim().as_bytes()).unwrap();
    let decrypted = cipher.decrypt_vec(&secret).unwrap();

    return Ok(decrypted);
}

fn main() -> Result<(), io::Error> {
    println!("Ch8:");
    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/8.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));

    let decrypted = read_iter(&full_path, decrypt);

    for res in decrypted {
        match res {
            Ok(line) => {
                println!("{:?}", ascii::print(line.to_vec()));
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
