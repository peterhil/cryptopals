extern crate data_encoding;

use data_encoding::{BASE64, HEXLOWER};

fn hex_decode(hex: Vec<u8>) -> Vec<u8> {
    return HEXLOWER.decode(&hex).unwrap();
}

fn ch1() {
    let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = b"I'm killing your brain like a poisonous mushroom";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = hex_decode(hex.to_vec());
    assert_eq!(&decoded[..], &bytes[..]);

    let encoded = BASE64.encode(&decoded);
    assert_eq!(&encoded[..], &expected[..]);

    println!("Base64 encoded message: {:?}", encoded);
}

fn xor_buffers(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
    return v1
        .iter()
        .zip(v2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
}

fn ch2() {
    let hex1 = b"1c0111001f010100061a024b53535009181c".to_vec();
    let hex2 = b"686974207468652062756c6c277320657965".to_vec();
    let expected = "746865206b696420646f6e277420706c6179";

    let v3 = HEXLOWER.encode(&xor_buffers(hex_decode(hex1), hex_decode(hex2)).to_vec());
    assert_eq!(&v3[..], &expected[..]);

    println!("Ch2 result: {:?}", v3);
}

fn main() {
    ch1();
    ch2();
}
