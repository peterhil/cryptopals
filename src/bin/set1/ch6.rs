#![warn(clippy::all, rust_2018_idioms)]

use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, BTreeSet};

use cryptopals::ascii;
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

fn ch6() {
    // let secret: Vec<u8> = BASE64.decode(b"HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS").unwrap();
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let secret: Vec<u8> = xor::encrypt_repeated(plaintext, key);
    // let secret: Vec<u8> = b"ICEICE Burning 'em, if you ain't quick and nimble".to_vec();

    let metrics = guess_keysize(&secret, 40);
    if let Some((metric, sizes)) = metrics.iter().next() {
        println!("Probable keysizes: {:?} with metric {:?}", sizes, metric);
    }
}

fn main() {
    println!("Ch6:");
    ch6();
}
