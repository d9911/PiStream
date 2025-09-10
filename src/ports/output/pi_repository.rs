use crate::domain::entities::Algorithm;
use crate::domain::value_objects::{Digits, Precision};
use async_trait::async_trait;

/// Output port for π computation repository
#[async_trait]
pub trait PiRepository {
    /// Computes π using the specified algorithm
    async fn compute_pi(
        &self,
        algorithm: Algorithm,
        digits: Digits,
        precision: Precision,
    ) -> Result<String, String>;
}

