use bellman::groth16;
use bls12_381::Bls12;
use rand::rngs::OsRng;
use std::collections::HashMap;

use crate::circuits::{AgeCircuit, CountryCircuit};
use crate::core::hasher::hash_u8_vec;
use crate::core::user::User;

pub struct TrustedCenter {
    age_proof_by_id: HashMap<Vec<u8>, groth16::Proof<Bls12>>,
    country_proof_by_id: HashMap<Vec<u8>, groth16::Proof<Bls12>>,
}

impl TrustedCenter {
    pub fn new() -> Self {
        TrustedCenter {
            age_proof_by_id: HashMap::new(),
            country_proof_by_id: HashMap::new(),
        }
    }

    fn add_user_age(
        &mut self,
        user_id_hash: &Vec<u8>,
        user_age: u8,
        age_params: &groth16::Parameters<Bls12>,
    ) {
        // Create an instance of our circuit (with the preimage as a witness).
        let ac = AgeCircuit {
            preimage: Some(user_age),
        };
        // Create a Groth16 proof with our parameters.
        let age_proof = groth16::create_random_proof(ac, age_params, &mut OsRng).unwrap();

        self.age_proof_by_id
            .insert(user_id_hash.to_owned(), age_proof);
    }

    fn add_user_country(
        &mut self,
        user_id_hash: &Vec<u8>,
        user_country: &[u8; 40],
        country_params: &groth16::Parameters<Bls12>,
    ) {
        // Create an instance of our circuit (with the preimage as a witness).
        let cc = CountryCircuit {
            preimage: Some(user_country.to_owned()),
        };
        // Create a Groth16 proof with our parameters.
        let country_proof = groth16::create_random_proof(cc, country_params, &mut OsRng).unwrap();

        self.country_proof_by_id
            .insert(user_id_hash.to_owned(), country_proof);
    }

    pub fn add_user(
        &mut self,
        user: &User,
        age_params: &groth16::Parameters<Bls12>,
        country_params: &groth16::Parameters<Bls12>,
    ) {
        let user_id_hash = hash_u8_vec(&user.id());

        self.add_user_age(&user_id_hash, user.age(), age_params);
        self.add_user_country(&user_id_hash, &user.country(), country_params);
    }

    pub fn get_age_proof(&self, id_hash: &Vec<u8>) -> &groth16::Proof<Bls12> {
        &self.age_proof_by_id.get(id_hash).unwrap()
    }

    pub fn get_country_proof(&self, id_hash: &Vec<u8>) -> &groth16::Proof<Bls12> {
        &self.country_proof_by_id.get(id_hash).unwrap()
    }
}
