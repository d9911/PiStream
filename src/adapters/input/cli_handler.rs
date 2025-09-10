use crate::domain::entities::{Algorithm, PiComputation};
use crate::domain::services::{ComputationEstimate, PiCalculator};
use crate::domain::value_objects::Digits;
use crate::ports::input::{AlgorithmInfo, PiComputationUseCase};
use crate::ports::output::PiRepository;
use async_trait::async_trait;

/// CLI input adapter that implements the π computation use case
pub struct CliHandler<R: PiRepository> {
    calculator: PiCalculator<R>,
}

impl<R: PiRepository> CliHandler<R> {
    pub fn new(calculator: PiCalculator<R>) -> Self {
        Self { calculator }
    }
}

#[async_trait]
impl<R: PiRepository + Send + Sync> PiComputationUseCase for CliHandler<R> {
    async fn compute_pi(
        &self,
        algorithm: Algorithm,
        digits: Digits,
    ) -> Result<PiComputation, String> {
        self.calculator.compute_pi(algorithm, digits).await
    }

    fn validate_computation(&self, algorithm: Algorithm, digits: Digits) -> Result<(), String> {
        self.calculator.validate_computation(algorithm, digits)
    }

    fn estimate_resources(&self, algorithm: Algorithm, digits: Digits) -> ComputationEstimate {
        self.calculator.estimate_resources(algorithm, digits)
    }

    fn list_algorithms(&self) -> Vec<AlgorithmInfo> {
        vec![
            AlgorithmInfo::new(Algorithm::ChudnovskyNaive, 1),
            AlgorithmInfo::new(Algorithm::ChudnovskyBinarySplitting, 2),
            AlgorithmInfo::new(Algorithm::MachinFormula, 3),
            AlgorithmInfo::new(Algorithm::BorweinQuadratic, 4),
            AlgorithmInfo::new(Algorithm::ChudnovskyUltraScale, 5),
        ]
    }

    fn get_algorithm_by_choice(&self, choice: u8) -> Algorithm {
        Algorithm::from(choice)
    }
}

