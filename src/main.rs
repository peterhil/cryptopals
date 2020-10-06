#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{BASE64, HEXLOWER};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use cryptopals::ascii;
use cryptopals::encoding;
use cryptopals::stat::text;
use cryptopals::xor;

fn ch1() -> String {
    let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = b"I'm killing your brain like a poisonous mushroom";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = encoding::hex_decode(hex.to_vec());
    assert_eq!(&decoded[..], &bytes[..]);

    let encoded = BASE64.encode(&decoded);
    assert_eq!(&encoded[..], &expected[..]);

    return encoded;
}

fn ch2() -> String {
    let hex1 = b"1c0111001f010100061a024b53535009181c".to_vec();
    let hex2 = b"686974207468652062756c6c277320657965".to_vec();
    let expected = "746865206b696420646f6e277420706c6179";

    let v3 = HEXLOWER.encode(&xor::xor_buffers(&encoding::hex_decode(hex1), &encoding::hex_decode(hex2)).to_vec());
    assert_eq!(&v3[..], &expected[..]);

    return v3;
}

fn ch3() {
    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let secret: Vec<u8> = encoding::hex_decode(hex.to_vec());
    let letters: Vec<u8> = (0..=255).collect::<Vec<u8>>();
    let mut metrics: BTreeMap<OrderedFloat<f64>, BTreeMap<char, Vec<u8>>> = BTreeMap::new();

    letters
        .iter()
        .for_each(|&letter| {
            let decoded = &xor::xor_char(&secret, letter);

            // Convert to string, lowercase the message and calculate metric
            match String::from_utf8(decoded.to_vec()) {
                Ok(text) => {
                    let metric = text::englishness(&text.to_lowercase());
                    metrics
                        .entry(OrderedFloat::<f64>::from(metric))
                        .or_insert(BTreeMap::new())
                        .insert(letter as char, decoded.to_vec());
                },
                Err(_) => (),
            }
        });

    // Get minimum element of BTreeMap: https://stackoverflow.com/a/58951038/470560
    if let Some((minimum, decodings)) = metrics.iter().next_back() {
        println!("Probably encoded with {:?}: {:.1}", decodings.keys().copied().collect::<Vec<char>>(), minimum);
        for (letter, decoding) in decodings {
            println!("{}\t{:?}", ascii::printable_escape(*letter as char), ascii::print(decoding.to_vec()));
        }
    }
}

fn main() {
    println!("Ch1: Base64 encoded message: {:?}", ch1());
    println!("Ch2: Result: {:?}", ch2());
    println!("Ch3:");
    ch3();
}
