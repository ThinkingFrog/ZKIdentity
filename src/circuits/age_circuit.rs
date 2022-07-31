use bellman::{Circuit, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use super::CustomCircuit;

pub struct AgeCircuit {
    /// The input to SHA-256d we are proving that we know. Set to `None` when we
    /// are verifying a proof (and do not have the witness data).
    pub preimage: Option<u8>,
}

impl CustomCircuit<Option<u8>> for AgeCircuit {
    fn bit_values(preimage: Option<u8>) -> Vec<Option<bool>> {
        if let Some(preimage) = preimage {
            (0..8)
                .map(move |i| (preimage >> i) & 1u8 == 1u8)
                .map(|b| Some(b))
                .collect()
        } else {
            vec![None; 8]
        }
    }
}

impl<Scalar: PrimeField> Circuit<Scalar> for AgeCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let preimage = self.preimage;
        CustomCircuit::synthesize(self, cs, preimage)
    }
}
