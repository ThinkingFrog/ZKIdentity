use bellman::gadgets::multipack;
use core::ops::Range;
use ff::PrimeField;
use rand::prelude::SliceRandom;
use sha2::{Digest, Sha256};

pub fn string_to_u8_40_array(str: &String) -> [u8; 40] {
    let mut arr = [0; 40];
    for (idx, b) in str.bytes().enumerate() {
        arr[idx] = b;
    }
    arr
}

pub fn num_to_inputs<Scalar: PrimeField>(num: u8) -> Vec<Scalar> {
    let preimage = num;
    let hash = Sha256::digest(&Sha256::digest(&[preimage]));
    let hash_bits = multipack::bytes_to_bits_le(&hash);

    multipack::compute_multipacking(&hash_bits)
}

pub fn string_to_inputs<Scalar: PrimeField>(str: &String) -> Vec<Scalar> {
    let preimage = string_to_u8_40_array(str);
    let hash = Sha256::digest(&Sha256::digest(&preimage));
    let hash_bits = multipack::bytes_to_bits_le(&hash);

    multipack::compute_multipacking(&hash_bits)
}

pub fn u8_range_to_inputs<Scalar: PrimeField>(range: &Range<u8>) -> Vec<Vec<Scalar>> {
    let mut res: Vec<Vec<Scalar>> = range.to_owned().map(|num| num_to_inputs(num)).collect();
    res.shuffle(&mut rand::thread_rng());
    res
}

pub fn string_vec_to_inputs<Scalar: PrimeField>(vec: &Vec<String>) -> Vec<Vec<Scalar>> {
    let mut res: Vec<Vec<Scalar>> = vec.iter().map(|str| string_to_inputs(str)).collect();
    res.shuffle(&mut rand::thread_rng());
    res
}
