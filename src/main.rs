#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{BASE64, HEXLOWER};
use maplit::hashmap;
use std::collections::HashMap;

use cryptopals::ascii;
use cryptopals::encoding;
use cryptopals::xor;

fn ch1() {
    let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = b"I'm killing your brain like a poisonous mushroom";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = encoding::hex_decode(hex.to_vec());
    assert_eq!(&decoded[..], &bytes[..]);

    let encoded = BASE64.encode(&decoded);
    assert_eq!(&encoded[..], &expected[..]);

    println!("Base64 encoded message: {:?}", encoded);
}

fn ch2() {
    let hex1 = b"1c0111001f010100061a024b53535009181c".to_vec();
    let hex2 = b"686974207468652062756c6c277320657965".to_vec();
    let expected = "746865206b696420646f6e277420706c6179";

    let v3 = HEXLOWER.encode(&xor::xor_buffers(encoding::hex_decode(hex1), encoding::hex_decode(hex2)).to_vec());
    assert_eq!(&v3[..], &expected[..]);

    println!("Ch2 result: {:?}", v3);
}

fn ch3() {
    let secret = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let letters = b"v\nD \x1a\x17\x05vETAOINSHRDLUCMFWYPVBGKQJXZ etaoinshrdlucmfwypvbgkqjxz0123456789@!\"#$%&/()+{}[]=,.-;:_\\|'*^~";  // English letter frequency order

    // Use letter frequency of decoded as a metric and choose the best one
    // - Get letter frequencies (lowercase the message!)
    // - Only consider union of letters freqs in english and message (maybe normalise)
    // - Do Least squares on each letter
    // - Calculate median or mean
    let _english: HashMap<char, f64> = hashmap!{
        'a' => 0.08167,
        'b' => 0.01492,
        'c' => 0.02782,
        'd' => 0.04253,
        'e' => 0.12702,
        'f' => 0.02228,
        'g' => 0.02015,
        'h' => 0.06094,
        'i' => 0.06966,
        'j' => 0.00153,
        'k' => 0.00772,
        'l' => 0.04025,
        'm' => 0.02406,
        'n' => 0.06749,
        'o' => 0.07507,
        'p' => 0.01929,
        'q' => 0.00095,
        'r' => 0.05987,
        's' => 0.06327,
        't' => 0.09056,
        'u' => 0.02758,
        'v' => 0.00978,
        'w' => 0.0236,
        'x' => 0.0015,
        'y' => 0.01974,
        'z' => 0.00074,
    };

    // let decoded = &xor::xor_char(secret.to_vec(), b'z');
    // println!("Ch3: {:?}\n{:?}\n{:?}", &decoded, BASE64.encode(&decoded), ascii::print(decoded.to_vec()));

    letters
        .iter()
        .for_each(|&chr| {
            let decoded = &xor::xor_char(secret.to_vec(), chr);
            println!("{}: {:?}", ascii::printable(chr as char), ascii::print(decoded.to_vec()));
        })
}

fn main() {
    ch1();
    ch2();
    ch3();
}
