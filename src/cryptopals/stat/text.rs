#![allow(unused_variables)]

use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Sum;

use crate::cryptopals::xor;
use super::character;

// Use letter frequency of text as a metric and return a metric of likeness to English text
pub fn englishness(text: &String) -> f64 {
    let frequencies = character::frequencies(character::counts(text));

    // - Only consider union of letters freqs in english and message
    let english_set: HashSet<char> = character::ENGLISH.keys().copied().collect();
    let frequency_set: HashSet<char> = frequencies.keys().copied().collect();
    let common = english_set.intersection(&frequency_set).copied().collect::<Vec<char>>();

    // - Do least squares on difference of each letter frequency
    let mut l2_differences: HashMap<char, f64> = HashMap::new();
    for ch in &common {
        let l2_diff: f64 = (character::ENGLISH.get(&ch).unwrap() - frequencies.get(&ch).unwrap()).powi(2);
        l2_differences.insert(*ch, l2_diff);
    }

    // - Calculate metric
    let metric: f64 = -l2_differences.values().product::<f64>().log2();

    return metric;
}

pub fn most_english(texts: &Vec<String>) -> BTreeMap<OrderedFloat<f64>, Vec<&String>> {
    let mut metrics = BTreeMap::new();

    texts
        .iter()
        .for_each(|text| {
            let metric = englishness(&text);
            metrics
                .entry(OrderedFloat::<f64>::from(metric))
                .or_insert(vec![])
                .push(text);
        });

    return metrics;
}

pub fn hamming_distance(a: Vec<u8>, b: Vec<u8>) -> u64 {
    return xor::xor_buffers(&a, &b).iter()
        .map(|c| c.count_ones() as u64)
        .sum::<u64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    mod hamming_distance {
        use super::*;

        #[test]
        fn gives_37() {
            let a: Vec<u8> = b"this is a test".to_vec();
            let b: Vec<u8> = b"wokka wokka!!!".to_vec();
            assert_eq!(37, hamming_distance(a, b));
        }
    }
}
