#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::BASE64;
use ordered_float::OrderedFloat;
use permutate::{Permutator};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use transpose::transpose;

use cryptopals::ascii;
use cryptopals::io::{exit_err, read_lines_by};
use cryptopals::stat::text;
use cryptopals::xor;

fn edit_distance_metric(s1: Vec<u8>, s2: Vec<u8>) -> f64 {
    // assert_eq!(s1.len(), s2.len(), "Arguments must be the same length!");

    let distance = text::hamming_distance(s1.to_vec(), s2.to_vec());
    let metric: f64 = distance as f64 / s1.len() as f64;

    println!("{:x?}, {:x?}: distance {}", ascii::print(s1.to_vec()), ascii::print(s2.to_vec()), distance);

    return metric;
}

fn try_keysize(secret: Vec<u8>, keysize: usize) -> f64 {
    let mut chunks = secret.chunks(keysize).take(2);
    let s1 = chunks.next().unwrap();
    let s2 = chunks.next().unwrap();

    let metric = edit_distance_metric(s1.to_vec(), s2.to_vec());

    return metric;
}

#[allow(dead_code)]
fn try_keysize_avg(secret: Vec<u8>, keysize: usize, blocks: usize) -> f64 {
    let chunks = secret.chunks(keysize).take(blocks);
    let mut metrics: Vec<f64> = vec![];

    chunks
        .into_iter()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|window| {
            let mut wi = window.iter();
            let s1 = &wi.next().unwrap();
            let s2 = &wi.next().unwrap();
            let metric = edit_distance_metric(s1.to_vec(), s2.to_vec());
            // println!("Metric: {:?}, s1: {:?}, s2: {:?}", metric, s1, s2);
            metrics.push(metric);
        });

    return metrics.iter().sum::<f64>() / metrics.len() as f64;
}

fn guess_keysize(secret: &Vec<u8>, max_keysize: usize) ->
    BTreeMap<OrderedFloat<f64>, BTreeSet<usize>>
{
    let keysizes = (2..=max_keysize).collect::<Vec<usize>>();
    let mut metrics = BTreeMap::new();

    for keysize in keysizes {
        let metric = try_keysize(secret.to_vec(), keysize);
        // let metric = try_keysize_avg(secret.to_vec(), keysize, 4);
        println!("Keysize: {}, metric: {}", keysize, metric);
        println!();

        metrics
            .entry(OrderedFloat::<f64>::from(metric))
            .or_insert(BTreeSet::new())
            .insert(keysize);
    }

    return metrics;
}

fn base64_decode(base64: &str) -> Vec<u8> {
    return BASE64
        .decode(&base64.as_bytes()[..])
        .unwrap_or_else(|e| exit_err(e, 3));
}

fn get_secret() -> Vec<u8> {
    // let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    // let key = "ICE";
    // let secret: Vec<u8> = xor::encrypt_repeated(&plaintext.as_bytes().to_vec(), &key.as_bytes().to_vec());
    // let secret: Vec<u8> = BASE64.decode(b"HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS").unwrap();
    // let secret: Vec<u8> = b"ICEICE Burning 'em, if you ain't quick and nimble".to_vec();

    let full_path = env::args().nth(1)
        .ok_or(format!("Usage: {} data/6.txt", env::args().nth(0).unwrap()))
        .unwrap_or_else(|e| exit_err(e, 1));
    let secret = read_lines_by(&full_path, base64_decode).unwrap();

    return secret;
}

fn get_keysize(secret: &Vec<u8>) -> usize {
    let metrics = guess_keysize(&secret, 15);
    let mut keysize: usize = 0;

    if let Some((metric, sizes)) = metrics.iter().next() {
        println!("Probable keysizes: {:?} with metric {:?}", sizes, metric.into_inner());
        keysize = *sizes.iter().nth(0).unwrap();
    }

    return keysize;
}

fn pick_repeating_key(secret: &Vec<u8>) -> Vec<Vec<char>> {
    let keysize = 29;
    // let byte_count = keysize * (keysize * 14);
    // let keysize = get_keysize(&secret);
    let byte_count = keysize * (secret.len() / keysize);

    let blocksize = byte_count / keysize;
    println!("Using keysize: {:?}, and block size: {:?} for {:?} bytes\n", keysize, blocksize, byte_count);

    let m: Vec<u8> = secret.iter().cloned().take(byte_count).collect::<Vec<u8>>();
    let mut blocks = vec![0; byte_count];

    transpose(&m, &mut blocks, keysize, blocksize);
    println!("Bytes: {:02x?}", &m);
    println!("Trans: {:02x?}\n", &blocks);

    let mut keys: Vec<Vec<char>> = vec![];

    for block in blocks.chunks(blocksize) {
        println!("Block: {:02x?}", &block);
        let ch: Vec<char> = xor::decrypt_single_byte(&block.to_vec(), 1);

        keys.push(ch);
    }

    return keys;
}

// See example on using permute: https://docs.rs/permutate/0.3.2/permutate/
fn permute_keys(guessed_keys: Vec<Vec<char>>) -> Vec<String> {
    // Convert the `Vec<Vec<char>>` into a `Vec<Vec<&str>>`
    let str_vec: Vec<Vec<String>> = guessed_keys.iter()
        .map(|list| list.iter().map(|ch| ch.to_string()).collect::<Vec<String>>())
        .collect();

    // Convert the `Vec<Vec<String>>` into a `Vec<Vec<&str>>`
    let tmp: Vec<Vec<&str>> = str_vec.iter()
        .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
        .collect();

    // Convert the `Vec<Vec<&str>>` into a `Vec<&[&str]>`
    let vector_of_arrays: Vec<&[&str]> = tmp.iter()
        .map(AsRef::as_ref).collect();

    // Pass the `Vec<&[&str]>` as an `&[&[&str]]`
    let permutator = Permutator::new(&vector_of_arrays[..]);
    let mut keys = vec![];

    // iteration 2: allocates a new buffer for each permutation
    // you may opt to re-allocate or not (see iteration 1)
    for key in permutator {
        let str_key: String = key.join("");
        keys.push(str_key);
    }

    return keys;
}

fn main() {
    let secret: Vec<u8> = get_secret();

    println!("Ch6:");
    println!("Secret length: {:?}", &secret.len());

    let guessed_keys: Vec<Vec<char>> = pick_repeating_key(&secret);
    println!("KEYS: {:#x?}", guessed_keys);

    let mut keys = permute_keys(guessed_keys);
    keys.push("Terminator X: Bring the noise".to_string());
    let texts: Vec<String> = vec![];

    for key in keys {
        let decrypted: &Vec<u8> = &xor::encrypt_repeated(&secret, &key.as_bytes().to_vec());
        println!("KEY: {:#x?}: {:?}", key, ascii::print(decrypted.to_vec()));
        println!();
        // texts.push(ascii::print(decrypted.to_vec()));
    }

    let metrics = text::most_english(&texts);
    let mut metrics_iter = metrics.iter();
    for _ in 0..2 {
        if let Some((metric, text)) = metrics_iter.next_back() {
            println!("{:?}: {:?}", metric.into_inner(), text);
            println!();
        }
    }
}
