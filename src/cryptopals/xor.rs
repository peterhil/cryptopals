#![allow(unused_variables)]

use data_encoding::HEXLOWER;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use crate::cryptopals::stat::text;
use crate::cryptopals::ascii;

pub fn xor_buffers(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    return v1
        .iter()
        .zip(v2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
}

pub fn xor_char(message: &Vec<u8>, chr: u8) -> Vec<u8> {
    return message
        .iter()
        .map(|&letter| letter ^ chr)
        .collect();
}

pub fn metrics(secret: &Vec<u8>, letters: &Vec<u8>) -> BTreeMap<OrderedFloat<f64>, BTreeMap<char, Vec<u8>>> {
    let mut metrics = BTreeMap::new();

    letters
        .iter()
        .for_each(|&letter| {
            let decoded = &xor_char(secret, letter);

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

    return metrics;
}

pub fn decrypt_single_byte(secret: Vec<u8>) {
    let letters: Vec<u8> = (0..=255).collect::<Vec<u8>>();
    let metrics = metrics(&secret, &letters);

    // Get minimum element of BTreeMap: https://stackoverflow.com/a/58951038/470560
    if let Some((minimum, decodings)) = metrics.iter().next_back() {
        if minimum.into_inner() > 160.0f64 {
            println!("Probably encoded with {:?}: {:.1}", decodings.keys().copied().collect::<Vec<char>>(), minimum);
            for (letter, decoding) in decodings {
                println!("{}\t{:?}", ascii::printable_escape(*letter as char), ascii::print(decoding.to_vec()));
            }
            println!("---");
        }
    }
}

pub fn encrypt_repeated(plaintext: &str, key: &str) -> String {
    let count: usize = (plaintext.len() as f64 / key.len() as f64).ceil() as usize;
    let repeated = &key.repeat(count)[..plaintext.len()];
    println!("{}", plaintext);

    let encoded: Vec<u8> = xor_buffers(
        &plaintext.as_bytes().to_vec(),
        &repeated.as_bytes().to_vec()
    );
    let hex = HEXLOWER.encode(&encoded);

    return hex;
}
