use crate::circuits::{AgeCircuit, CountryCircuit};

use bellman::groth16;
use bls12_381::Bls12;
use rand::rngs::OsRng;

pub struct Parameters {
    age_params: groth16::Parameters<Bls12>,
    country_params: groth16::Parameters<Bls12>,
}

impl Parameters {
    pub fn new() -> Self {
        Parameters {
            age_params: groth16::generate_random_parameters::<Bls12, _, _>(
                AgeCircuit { preimage: None },
                &mut OsRng,
            )
            .unwrap(),
            country_params: groth16::generate_random_parameters::<Bls12, _, _>(
                CountryCircuit { preimage: None },
                &mut OsRng,
            )
            .unwrap(),
        }
    }

    pub fn age_params(&self) -> groth16::Parameters<Bls12> {
        self.age_params.to_owned()
    }

    pub fn country_params(&self) -> groth16::Parameters<Bls12> {
        self.country_params.to_owned()
    }
}
