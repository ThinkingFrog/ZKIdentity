use bellman::{
    gadgets::{boolean::Boolean, sha256::sha256},
    ConstraintSystem, SynthesisError,
};
use ff::PrimeField;
use sha2::{Digest, Sha256};

pub fn hash_u8_vec(arr: &Vec<u8>) -> Vec<u8> {
    Sha256::digest(arr)[..].into()
}

/// Our own SHA-256d gadget. Input and output are in little-endian bit order.
pub fn sha256d<Scalar: PrimeField, CS: ConstraintSystem<Scalar>>(
    mut cs: CS,
    data: &[Boolean],
) -> Result<Vec<Boolean>, SynthesisError> {
    // Flip endianness of each input byte
    let input: Vec<_> = data
        .chunks(8)
        .map(|c| c.iter().rev())
        .flatten()
        .cloned()
        .collect();

    let mid = sha256(cs.namespace(|| "SHA-256(input)"), &input)?;
    let res = sha256(cs.namespace(|| "SHA-256(mid)"), &mid)?;

    // Flip endianness of each output byte
    Ok(res
        .chunks(8)
        .map(|c| c.iter().rev())
        .flatten()
        .cloned()
        .collect())
}
