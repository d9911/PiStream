use crate::domain::entities::{Algorithm, PiComputation};
use crate::domain::services::ComputationEstimate;
use crate::domain::value_objects::Digits;
use async_trait::async_trait;

/// Input port for π computation use cases
#[async_trait]
pub trait PiComputationUseCase {
    /// Computes π with the given algorithm and digits
    async fn compute_pi(
        &self,
        algorithm: Algorithm,
        digits: Digits,
    ) -> Result<PiComputation, String>;

    /// Validates if a computation is feasible
    fn validate_computation(&self, algorithm: Algorithm, digits: Digits) -> Result<(), String>;

    /// Estimates resources needed for a computation
    fn estimate_resources(&self, algorithm: Algorithm, digits: Digits) -> ComputationEstimate;

    /// Lists available algorithms
    fn list_algorithms(&self) -> Vec<AlgorithmInfo>;

    /// Gets algorithm by choice number
    fn get_algorithm_by_choice(&self, choice: u8) -> Algorithm;
}

/// Information about an available algorithm
#[derive(Debug, Clone)]
pub struct AlgorithmInfo {
    pub algorithm: Algorithm,
    pub choice_number: u8,
    pub name: String,
    pub description: String,
    pub max_digits: usize,
    pub is_ultra_scale: bool,
}

impl AlgorithmInfo {
    pub fn new(algorithm: Algorithm, choice_number: u8) -> Self {
        Self {
            algorithm,
            choice_number,
            name: algorithm.display_name().to_string(),
            description: algorithm.description().to_string(),
            max_digits: algorithm.max_digits(),
            is_ultra_scale: algorithm.is_ultra_scale(),
        }
    }

    pub fn format_for_menu(&self) -> String {
        format!(
            "{}. {} - {}",
            self.choice_number, self.name, self.description
        )
    }
}
