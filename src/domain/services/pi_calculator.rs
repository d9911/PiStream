use crate::domain::entities::{Algorithm, PiComputation};
use crate::domain::value_objects::{Digits, Precision};
use crate::ports::output::PiRepository;
use std::time::Instant;

/// Domain service for π calculations
pub struct PiCalculator<R: PiRepository> {
    repository: R,
}

impl<R: PiRepository> PiCalculator<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Computes π using the specified algorithm and digits
    pub async fn compute_pi(
        &self,
        algorithm: Algorithm,
        digits: Digits,
    ) -> Result<PiComputation, String> {
        let mut computation = PiComputation::new(algorithm, digits);

        // Validate the computation
        computation.validate()?;

        // Calculate precision
        let precision = if algorithm.is_ultra_scale() {
            Precision::from_digits_ultra_scale(digits)
        } else {
            Precision::from_digits(digits)
        };

        computation.precision = Some(precision);

        // Start computation
        computation.start();

        // Delegate to repository for actual computation
        let computation_start = Instant::now();
        let result = self
            .repository
            .compute_pi(algorithm, digits, precision)
            .await?;
        let computation_time = computation_start.elapsed();

        // Complete computation
        computation.complete(result, computation_time);

        Ok(computation)
    }

    /// Validates if a computation is feasible
    pub fn validate_computation(&self, algorithm: Algorithm, digits: Digits) -> Result<(), String> {
        let computation = PiComputation::new(algorithm, digits);
        computation.validate()
    }

    /// Estimates computation resources
    pub fn estimate_resources(&self, algorithm: Algorithm, digits: Digits) -> ComputationEstimate {
        let precision = if algorithm.is_ultra_scale() {
            Precision::from_digits_ultra_scale(digits)
        } else {
            Precision::from_digits(digits)
        };

        let iterations = self.estimate_iterations(algorithm, digits);
        let memory_gb = precision.estimated_memory_gb();
        let estimated_time = self.estimate_time(algorithm, digits);

        ComputationEstimate {
            precision,
            iterations,
            memory_gb,
            estimated_time,
        }
    }

    fn estimate_iterations(&self, algorithm: Algorithm, digits: Digits) -> usize {
        match algorithm {
            Algorithm::ChudnovskyNaive
            | Algorithm::ChudnovskyBinarySplitting
            | Algorithm::ChudnovskyUltraScale
            | Algorithm::ChudnovskyUltraScaleStreaming => {
                // Chudnovsky algorithms: approximately digits/14 iterations
                (digits.value() / 14).saturating_add(20)
            }
            Algorithm::MachinFormula => {
                // Machin formula needs more terms for convergence
                digits.value().saturating_add(50)
            }
            Algorithm::BorweinQuadratic => {
                // Quadratic convergence: log2(digits) iterations
                ((digits.value() as f64).log2()).ceil() as usize + 5
            }
        }
    }

    fn estimate_time(&self, algorithm: Algorithm, digits: Digits) -> EstimatedTime {
        // These are rough estimates based on algorithm complexity
        let base_time_seconds = match algorithm {
            Algorithm::ChudnovskyNaive => {
                // O(n^2) complexity for large n
                (digits.value() as f64 / 10_000.0).powi(2)
            }
            Algorithm::ChudnovskyBinarySplitting => {
                // O(n log n) complexity
                let n = digits.value() as f64;
                (n * n.log2()) / 1_000_000.0
            }
            Algorithm::MachinFormula => {
                // O(n) complexity but with higher constant
                digits.value() as f64 / 100_000.0
            }
            Algorithm::BorweinQuadratic => {
                // Fast quadratic convergence
                digits.value() as f64 / 500_000.0
            }
            Algorithm::ChudnovskyUltraScale => {
                // Similar to binary splitting but with overhead for large numbers
                let n = digits.value() as f64;
                (n * n.log2()) / 500_000.0
            }
            Algorithm::ChudnovskyUltraScaleStreaming => {
                // Streaming version with additional overhead for file I/O
                let n = digits.value() as f64;
                (n * n.log2()) / 400_000.0
            }
        };

        if base_time_seconds < 60.0 {
            EstimatedTime::Seconds(base_time_seconds as u32)
        } else if base_time_seconds < 3600.0 {
            EstimatedTime::Minutes((base_time_seconds / 60.0) as u32)
        } else if base_time_seconds < 86400.0 {
            EstimatedTime::Hours((base_time_seconds / 3600.0) as u32)
        } else {
            EstimatedTime::Days((base_time_seconds / 86400.0) as u32)
        }
    }
}

/// Represents estimated computation resources
#[derive(Debug, Clone)]
pub struct ComputationEstimate {
    pub precision: Precision,
    pub iterations: usize,
    pub memory_gb: f64,
    pub estimated_time: EstimatedTime,
}

/// Represents estimated computation time
#[derive(Debug, Clone)]
pub enum EstimatedTime {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
}

impl std::fmt::Display for EstimatedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EstimatedTime::Seconds(s) => write!(f, "{} сек", s),
            EstimatedTime::Minutes(m) => write!(f, "{} мин", m),
            EstimatedTime::Hours(h) => write!(f, "{} ч", h),
            EstimatedTime::Days(d) => write!(f, "{} дн", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::output::PiRepository;
    use async_trait::async_trait;

    // Mock repository for testing
    struct MockPiRepository;

    #[async_trait]
    impl PiRepository for MockPiRepository {
        async fn compute_pi(
            &self,
            _algorithm: Algorithm,
            _digits: Digits,
            _precision: Precision,
        ) -> Result<String, String> {
            Ok("3.14159265358979323846".to_string())
        }
    }

    #[tokio::test]
    async fn test_compute_pi_success() {
        let repository = MockPiRepository;
        let calculator = PiCalculator::new(repository);

        let digits = Digits::new(100).unwrap();
        let result = calculator
            .compute_pi(Algorithm::ChudnovskyBinarySplitting, digits)
            .await;

        assert!(result.is_ok());
        let computation = result.unwrap();
        assert!(computation.is_completed());
        assert!(computation.result.is_some());
    }

    #[tokio::test]
    async fn test_compute_pi_validation_failure() {
        let repository = MockPiRepository;
        let calculator = PiCalculator::new(repository);

        // Try to use naive algorithm with too many digits
        let digits = Digits::new(2_000_000).unwrap();
        let result = calculator
            .compute_pi(Algorithm::ChudnovskyNaive, digits)
            .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_computation() {
        let repository = MockPiRepository;
        let calculator = PiCalculator::new(repository);

        let digits = Digits::new(500_000).unwrap();

        // Should succeed with binary splitting
        assert!(calculator
            .validate_computation(Algorithm::ChudnovskyBinarySplitting, digits)
            .is_ok());

        // Should fail with naive algorithm
        let large_digits = Digits::new(2_000_000).unwrap();
        assert!(calculator
            .validate_computation(Algorithm::ChudnovskyNaive, large_digits)
            .is_err());
    }

    #[test]
    fn test_estimate_resources() {
        let repository = MockPiRepository;
        let calculator = PiCalculator::new(repository);

        let digits = Digits::new(1_000_000).unwrap();
        let estimate = calculator.estimate_resources(Algorithm::ChudnovskyBinarySplitting, digits);

        assert!(estimate.iterations > 0);
        assert!(estimate.memory_gb > 0.0);
        assert!(estimate.precision.bits() > 0);
    }

    #[test]
    fn test_estimate_iterations() {
        let repository = MockPiRepository;
        let calculator = PiCalculator::new(repository);

        let digits = Digits::new(1400).unwrap(); // Should need ~100 iterations for Chudnovsky

        // Chudnovsky algorithms
        let iterations =
            calculator.estimate_iterations(Algorithm::ChudnovskyBinarySplitting, digits);
        assert!((100..=150).contains(&iterations));

        // Borwein should need fewer iterations due to quadratic convergence
        let borwein_iterations =
            calculator.estimate_iterations(Algorithm::BorweinQuadratic, digits);
        assert!(borwein_iterations < iterations);
    }
}
