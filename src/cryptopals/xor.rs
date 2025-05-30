#![allow(unused_variables)]

use data_encoding::HEXLOWER;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use crate::cryptopals::ascii;
use crate::cryptopals::stat::text;

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
                    let metric = text::englishness(&text);
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

fn print_metric(metric: &OrderedFloat<f64>, keys: &Vec<char>, decodings: &BTreeMap<char, Vec<u8>>) {
    println!("--- Probably encoded with {:?}: {:.1}", &keys, metric);
    for (letter, decoding) in decodings {
        println!("{}\t{:?}", ascii::printable_escape(*letter as char), ascii::print(decoding.to_vec()));
    }
    println!();
}

pub fn decrypt_single_byte(secret: &Vec<u8>, count: usize) -> Vec<char> {
    let letters: Vec<u8> = (0..=255).collect::<Vec<u8>>();
    let metrics = metrics(&secret, &letters);
    let mut metrics_iter = metrics.iter().filter(|(metric, _)| metric.into_inner() > 0.0f64);
    let mut probable_keys = vec![];

    if &metrics.len() > &0 {
        println!(">>> {}", HEXLOWER.encode(&secret));
    }

    for _ in 0..count {
        // Get minimum element of BTreeMap: https://stackoverflow.com/a/58951038/470560
        if let Some((metric, decodings)) = metrics_iter.next_back() {
            let mut keys = decodings.keys().copied().collect::<Vec<char>>();

            print_metric(&metric, &keys, &decodings);
            probable_keys.append(&mut keys);
        }
    }

    return probable_keys;
}

pub fn encrypt_repeated(plaintext: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let repeated: Vec<u8> = key.iter().cloned().cycle().take(plaintext.len()).collect::<Vec<u8>>();
    let encrypted: Vec<u8> = xor_buffers(
        &plaintext,
        &repeated
    );

    return encrypted;
}
