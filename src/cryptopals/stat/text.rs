#![allow(unused_variables)]

use std::collections::HashMap;
use std::collections::HashSet;

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
