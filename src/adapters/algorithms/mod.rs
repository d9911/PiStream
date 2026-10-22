pub mod borwein_quadratic;
pub mod chudnovsky_binary_splitting;
pub mod chudnovsky_naive;
pub mod chudnovsky_ultra_scale;
pub mod chudnovsky_ultra_scale_streaming;
pub mod machin_formula;

use crate::domain::entities::Algorithm;
use crate::domain::value_objects::{Digits, Precision};
use crate::ports::input::AlgorithmInfo;
use crate::ports::output::PiRepository;
use async_trait::async_trait;

/// Repository for managing available Pi computation algorithms
pub struct AlgorithmRepository;

impl AlgorithmRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn list_algorithms(&self) -> Vec<AlgorithmInfo> {
        vec![
            AlgorithmInfo::new(Algorithm::ChudnovskyNaive, 1),
            AlgorithmInfo::new(Algorithm::ChudnovskyBinarySplitting, 2),
            AlgorithmInfo::new(Algorithm::MachinFormula, 3),
            AlgorithmInfo::new(Algorithm::BorweinQuadratic, 4),
            AlgorithmInfo::new(Algorithm::ChudnovskyUltraScale, 5),
            AlgorithmInfo::new(Algorithm::ChudnovskyUltraScaleStreaming, 6),
        ]
    }

    pub fn get_algorithm_by_choice(&self, choice: u8) -> Algorithm {
        Algorithm::from(choice)
    }

    pub fn algorithm_names(&self) -> Vec<String> {
        vec![
            "chudnovsky".to_string(),
            "chudnovsky_binary".to_string(),
            "machin".to_string(),
            "borwein".to_string(),
            "chudnovsky_ultra".to_string(),
            "chudnovsky_ultra_streaming".to_string(),
        ]
    }
}

impl Default for AlgorithmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PiRepository for AlgorithmRepository {
    async fn compute_pi(
        &self,
        algorithm: Algorithm,
        digits: Digits,
        precision: Precision,
    ) -> Result<String, String> {
        let digits_value = digits.value();

        // Run computation in a blocking task since the algorithms are synchronous
        tokio::task::spawn_blocking(move || {
            if algorithm == Algorithm::ChudnovskyUltraScaleStreaming {
                return chudnovsky_ultra_scale_streaming::compute_pi_chudnovsky_ultra_scale_streaming(digits_value, precision);
            }
            Ok(match algorithm {
            Algorithm::ChudnovskyNaive => {
                chudnovsky_naive::compute_pi_chudnovsky_naive(digits_value, precision)
            }
            Algorithm::ChudnovskyBinarySplitting => {
                chudnovsky_binary_splitting::compute_pi_chudnovsky_binary_splitting(
                    digits_value,
                    precision,
                )
            }
            Algorithm::ChudnovskyUltraScale => {
                chudnovsky_ultra_scale::compute_pi_chudnovsky_ultra_scale(digits_value, precision)
            }
            Algorithm::ChudnovskyUltraScaleStreaming => unreachable!(),
            Algorithm::BorweinQuadratic => {
                borwein_quadratic::compute_pi_borwein_quadratic(digits_value, precision)
            }
            Algorithm::MachinFormula => machin_formula::compute_pi_machin(digits_value, precision),
            })
        })
        .await
        .map_err(|e| format!("Computation task failed: {}", e))?
    }
}
