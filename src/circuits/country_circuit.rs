use bellman::{Circuit, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use super::CustomCircuit;

pub struct CountryCircuit {
    /// The input to SHA-256d we are proving that we know. Set to `None` when we
    /// are verifying a proof (and do not have the witness data).
    pub preimage: Option<[u8; 40]>,
}

impl CustomCircuit<Option<[u8; 40]>> for CountryCircuit {
    fn bit_values(preimage: Option<[u8; 40]>) -> Vec<Option<bool>> {
        if let Some(preimage) = preimage {
            preimage
                .iter()
                .map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8))
                .flatten()
                .map(|b| Some(b))
                .collect()
        } else {
            vec![None; 8 * 40]
        }
    }
}

impl<Scalar: PrimeField> Circuit<Scalar> for CountryCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let preimage = self.preimage;
        CustomCircuit::synthesize(self, cs, preimage)
    }
}
