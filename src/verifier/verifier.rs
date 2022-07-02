use crate::core::utils::*;
use crate::core::TrustedCenter;

use bellman::groth16;
use bls12_381::Bls12;
use core::ops::Range;

pub fn verify_age(
    user_id_hash: &Vec<u8>,
    age_range: &Range<u8>,
    tc: &TrustedCenter,
    age_params: &groth16::Parameters<Bls12>,
) -> bool {
    let pvk = groth16::prepare_verifying_key(&age_params.vk);

    let age_proof = tc.get_age_proof(user_id_hash);

    let age_inputs = u8_range_to_inputs(age_range);

    for inp in age_inputs.iter() {
        if groth16::verify_proof(&pvk, &age_proof, &inp).is_ok() {
            return true;
        }
    }

    false
}

pub fn verify_country(
    user_id_hash: &Vec<u8>,
    countries: &Vec<String>,
    tc: &TrustedCenter,
    country_params: &groth16::Parameters<Bls12>,
) -> bool {
    let pvk = groth16::prepare_verifying_key(&country_params.vk);

    let user_country_proof = tc.get_country_proof(user_id_hash);

    let country_inputs = string_vec_to_inputs(countries);

    for inp in country_inputs.iter() {
        if groth16::verify_proof(&pvk, &user_country_proof, &inp).is_ok() {
            return true;
        }
    }

    false
}
