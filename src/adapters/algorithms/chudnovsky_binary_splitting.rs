use crate::{
    calculation::{compute, Cancellation},
    Algorithm, Precision,
};

/// Compatibility adapter for the console and repository APIs.
pub fn compute_pi_chudnovsky_binary_splitting(digits: usize, precision: Precision) -> String {
    compute(
        Algorithm::ChudnovskyBinarySplitting,
        digits,
        precision,
        &Cancellation::default(),
    )
    .unwrap_or_else(|error| panic!("{error}"))
}
