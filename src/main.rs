#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{BASE64, HEXLOWER};
use ordered_float::OrderedFloat;
// use statrs::statistics::Statistics;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use cryptopals::ascii;
use cryptopals::encoding;
use cryptopals::stat::character;
use cryptopals::string;
// use cryptopals::types;
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

// Use letter frequency of text as a metric and return a metric of likeness to English text
fn englishness(text: &Vec<u8>) -> f64 {
    // - Get letter frequency counts (lowercase the message!)
    let s = string::from_vec(text.to_vec()).to_lowercase();

    let frequencies = character::frequencies(character::counts(s));

    // - Only consider union of letters freqs in english and message (maybe normalise)
    let english_set: HashSet<char> = character::ENGLISH.keys().copied().collect();
    let frequency_set: HashSet<char> = frequencies.keys().copied().collect();
    let common = english_set.intersection(&frequency_set).copied().collect::<Vec<char>>();

    // println!("Common characters: {:?}", common);
    // println!("Frequencies:");
    // for ch in &common {
    //     println!("{}: {:?}", ch, frequencies.get(ch).unwrap());
    // }

    // - Do least squares on difference of each letter frequency
    let mut l2_differences: HashMap<char, f64> = HashMap::new();

    // println!("L2 diffs:");
    for ch in &common {
        let l2_diff: f64 = (character::ENGLISH.get(&ch).unwrap() - frequencies.get(&ch).unwrap()).powi(2);
        l2_differences.insert(*ch, l2_diff);
        // println!("{}: {:?}", ch, l2_diff);
    }

    // - Calculate median or mean
    // let _l2_sum = l2_differences.values().sum::<f64>();
    // let _l2_mean = l2_differences.values().mean();
    let l2_product: f64 = l2_differences.values().product();
    // println!("Sum of diffs: {:?}", l2_sum);
    // println!("Mean of diffs: {:?}", l2_mean);

    return l2_product;
}

fn ch3() {
    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let secret: Vec<u8> = encoding::hex_decode(hex.to_vec());
    let letters: Vec<u8> = (0..=255).collect::<Vec<u8>>();
    // let letters = b"WZ\x80xX".to_vec();

    println!("Ch3:");

    let mut metrics: BTreeMap<OrderedFloat<f64>, Vec<char>> = BTreeMap::new();
    letters
        .iter()
        .for_each(|&letter| {
            let decoded = &xor::xor_char(secret, letter);
            // println!("{}:\t{:?}", ascii::printable_escape(letter as char), string::from_vec(decoded.to_vec()));
            let metric = englishness(decoded);
            metrics.entry(OrderedFloat::<f64>::from(metric)).or_insert(vec![]).append(&mut vec!(letter as char));
            println!("{}\t{}\t{:?}", ascii::printable_escape(letter as char), metric, ascii::print(decoded.to_vec()));

            // let decoded = &xor::xor_char(secret.to_vec(), chr);
            // println!("{}: {:?}", ascii::printable(letter as char), ascii::print(decoded.to_vec()));
        });

    // Get minimum element of BTreeMap: https://stackoverflow.com/a/58951038/470560
    if let Some((minimum, chars)) = metrics.iter().next() {
        println!("Probably encoded with {:?}: {}", chars, minimum);
    }
}

fn main() {
    ch1();
    ch2();
    ch3();
}
