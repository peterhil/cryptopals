#![warn(clippy::all, rust_2018_idioms)]

use hex_literal::hex;

use data_encoding::{BASE64};

fn ch1() -> String {
    let bytes: Vec<u8> = hex!["49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"].to_vec();
    let plain = b"I'm killing your brain like a poisonous mushroom";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(&bytes[..], &plain[..]);

    let encoded = BASE64.encode(&bytes);
    assert_eq!(&encoded[..], &expected[..]);

    return encoded;
}

fn main() {
    println!("Ch1: Base64 encoded message: {:?}", ch1());
}
