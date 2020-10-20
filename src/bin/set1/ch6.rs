#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::{BASE64, HEXLOWER};

use cryptopals::ascii;
use cryptopals::stat::text;
use cryptopals::types::print_type_of;
use cryptopals::xor;

fn edit_distance_metric(s1: Vec<u8>, s2: Vec<u8>) -> f64 {
    assert_eq!(s1.len(), s2.len(), "Arguments must be the same length!");

    let distance = text::hamming_distance(s1.to_vec(), s2.to_vec());
    let metric: f64 = distance as f64 / s1.len() as f64;

    println!("{:x?}, {:x?}: distance {}", ascii::print(s1.to_vec()), ascii::print(s2.to_vec()), distance);

    return metric;
}

fn guess_keysize(secret: Vec<u8>, keysize: usize) -> f64 {
    let mut chunks = secret.chunks(keysize).take(2);
    let s1 = chunks.next().unwrap();
    let s2 = chunks.next().unwrap();

    let metric = edit_distance_metric(s1.to_vec(), s2.to_vec());

    return metric;
}

fn ch6() {
    // let secret: Vec<u8> = BASE64.decode(b"HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS").unwrap();
    // let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    // let key = "ICE";
    // let secret: Vec<u8> = xor::encrypt_repeated(plaintext, key);
    let secret = b"ICEICE Burning 'em, if you ain't quick and nimble";
    let keysizes = (2..=15).collect::<Vec<usize>>();

    for keysize in keysizes {

        let metric = guess_keysize(secret.to_vec(), keysize);
        println!("Keysize: {}, metric: {}", keysize, metric);
        println!();
    }
}

fn main() {
    println!("Ch6:");
    ch6();
}
