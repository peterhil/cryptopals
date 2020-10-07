#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{HEXLOWER};

use cryptopals::encoding;
use cryptopals::xor;

fn ch2() -> String {
    let hex1 = b"1c0111001f010100061a024b53535009181c".to_vec();
    let hex2 = b"686974207468652062756c6c277320657965".to_vec();
    let expected = "746865206b696420646f6e277420706c6179";

    let v3 = HEXLOWER.encode(&xor::xor_buffers(&encoding::hex_decode(hex1), &encoding::hex_decode(hex2)).to_vec());
    assert_eq!(&v3[..], &expected[..]);

    return v3;
}

fn main() {
    println!("Ch2: Result: {:?}", ch2());
}
