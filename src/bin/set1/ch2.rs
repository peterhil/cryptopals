#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{HEXLOWER};

use cryptopals::xor;

fn ch2() -> String {
    let hex1: Vec<u8> = HEXLOWER.decode(b"1c0111001f010100061a024b53535009181c").unwrap();
    let hex2: Vec<u8> = HEXLOWER.decode(b"686974207468652062756c6c277320657965").unwrap();
    let expected = "746865206b696420646f6e277420706c6179";

    let xorred = HEXLOWER.encode(&xor::xor_buffers(&hex1, &hex2).to_vec());
    assert_eq!(&xorred[..], &expected[..]);

    return xorred;
}

fn main() {
    println!("Ch2: Result: {:?}", ch2());
}
