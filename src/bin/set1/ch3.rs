#![warn(clippy::all, rust_2018_idioms)]

use cryptopals::xor;

fn ch3() {
    let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();

    xor::decrypt_single_byte(&hex);
}

fn main() {
    println!("Ch3:");
    ch3();
}
