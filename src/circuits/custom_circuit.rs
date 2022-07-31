use crate::core::hasher::sha256d;
use bellman::{
    gadgets::{
        boolean::{AllocatedBit, Boolean},
        multipack,
    },
    ConstraintSystem, SynthesisError,
};
use ff::PrimeField;

pub trait CustomCircuit<T> {
    fn bit_values(preimage: T) -> Vec<Option<bool>>;

    fn synthesize<Scalar, CS>(self, cs: &mut CS, preimage: T) -> Result<(), SynthesisError>
    where
        Self: Sized,
        Scalar: PrimeField,
        CS: ConstraintSystem<Scalar>,
    {
        // Compute the values for the bits of the preimage. If we are verifying a proof,
        // we still need to create the same constraints, so we return an equivalent-size
        // Vec of None (indicating that the value of each bit is unknown).
        let bit_values = Self::bit_values(preimage);

        // Witness the bits of the preimage.
        let preimage_bits = bit_values
            .into_iter()
            .enumerate()
            // Allocate each bit.
            .map(|(i, b)| AllocatedBit::alloc(cs.namespace(|| format!("preimage bit {}", i)), b))
            // Convert the AllocatedBits into Booleans (required for the sha256 gadget).
            .map(|b| b.map(Boolean::from))
            .collect::<Result<Vec<_>, _>>()?;

        // Compute hash = SHA-256d(preimage).
        let hash = sha256d(cs.namespace(|| "SHA-256d(preimage)"), &preimage_bits)?;

        // Expose the vector of 32 boolean variables as compact public inputs.
        multipack::pack_into_inputs(cs.namespace(|| "pack hash"), &hash)
    }
}
