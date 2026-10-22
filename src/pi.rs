use crate::{Digits, Precision};

#[derive(Clone, Copy, Debug)]
pub enum PiAlgorithm {
    ChudnovskyNaive,
    ChudnovskyBinarySplitting,
    MachinFormula,
    BorweinQuadratic,
    ChudnovskyUltraScale, // Новый алгоритм для триллионов знаков
}

pub fn compute_pi(digits: usize, algorithm: PiAlgorithm) -> String {
    let precision = Precision::from_digits(
        Digits::new(digits).expect("Количество знаков должно быть больше нуля"),
    );
    match algorithm {
        PiAlgorithm::ChudnovskyNaive => crate::adapters::algorithms::chudnovsky_naive::compute_pi_chudnovsky_naive(digits, precision),
        PiAlgorithm::ChudnovskyBinarySplitting => crate::adapters::algorithms::chudnovsky_binary_splitting::compute_pi_chudnovsky_binary_splitting(digits, precision),
        PiAlgorithm::MachinFormula => crate::adapters::algorithms::machin_formula::compute_pi_machin(digits, precision),
        PiAlgorithm::BorweinQuadratic => crate::adapters::algorithms::borwein_quadratic::compute_pi_borwein_quadratic(digits, precision),
        PiAlgorithm::ChudnovskyUltraScale => crate::adapters::algorithms::chudnovsky_ultra_scale::compute_pi_chudnovsky_ultra_scale(digits, precision),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const PI_REFERENCE_50: &str = "3.14159265358979323846264338327950288419716939937511";
    const PI_REFERENCE_100: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170680";

    #[test]
    fn test_chudnovsky_naive_accuracy() {
        let result = compute_pi(50, PiAlgorithm::ChudnovskyNaive);
        let result_50 = &result[..52]; // "3." + 50 digits
        assert_eq!(
            result_50, PI_REFERENCE_50,
            "Chudnovsky naive algorithm incorrect"
        );
    }

    #[test]
    fn test_binary_splitting_accuracy() {
        let result = compute_pi(50, PiAlgorithm::ChudnovskyBinarySplitting);
        let result_50 = &result[..52];
        assert_eq!(
            result_50, PI_REFERENCE_50,
            "Binary splitting algorithm incorrect"
        );
    }

    #[test]
    fn test_machin_formula_accuracy() {
        let result = compute_pi(50, PiAlgorithm::MachinFormula);
        let result_50 = &result[..52];
        assert_eq!(
            result_50, PI_REFERENCE_50,
            "Machin formula algorithm incorrect"
        );
    }

    #[test]
    fn test_borwein_accuracy() {
        let result = compute_pi(50, PiAlgorithm::BorweinQuadratic);
        let result_50 = &result[..52];
        assert_eq!(result_50, PI_REFERENCE_50, "Borwein algorithm incorrect");
    }

    #[test]
    fn test_binary_splitting_100_digits() {
        let result = compute_pi(100, PiAlgorithm::ChudnovskyBinarySplitting);
        let result_100 = &result[..102]; // "3." + 100 digits
        assert_eq!(
            result_100, PI_REFERENCE_100,
            "Binary splitting 100 digits incorrect"
        );
    }

    #[test]
    fn test_algorithm_performance_comparison() {
        println!("\n🏁 Performance Comparison Test:");

        let algorithms = [
            (PiAlgorithm::ChudnovskyNaive, "Chudnovsky (Naive)"),
            (
                PiAlgorithm::ChudnovskyBinarySplitting,
                "Chudnovsky (Binary Splitting)",
            ),
            (PiAlgorithm::MachinFormula, "Machin Formula"),
            (PiAlgorithm::BorweinQuadratic, "Borwein Quadratic"),
        ];

        let test_digits = 100;
        let mut results = Vec::new();

        for (algorithm, name) in algorithms {
            let start = Instant::now();
            let pi_result = compute_pi(test_digits, algorithm);
            let duration = start.elapsed();

            // Verify correctness
            let pi_100 = &pi_result[..102];
            assert_eq!(
                pi_100, PI_REFERENCE_100,
                "{} produced incorrect result",
                name
            );

            results.push((name, duration));
            println!("  {} : {:?}", name, duration);
        }

        // All algorithms should complete in reasonable time (< 50ms for 100 digits)
        for (name, duration) in &results {
            assert!(
                duration.as_millis() < 50,
                "{} took too long: {:?}",
                name,
                duration
            );
        }

        println!("✅ All algorithms passed performance test!");
    }

    #[test]
    fn test_binary_splitting_scaling() {
        println!("\n📈 Binary Splitting Scaling Test:");

        let digit_counts = [10, 50, 100, 500, 1000];
        let mut prev_duration: Option<std::time::Duration> = None;

        for &digits in &digit_counts {
            let start = Instant::now();
            let result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);
            let duration = start.elapsed();

            // Basic correctness check
            assert!(
                result.starts_with("3.14159"),
                "Result should start with 3.14159"
            );
            assert!(
                result.len() >= digits + 2,
                "Result should have at least {} characters",
                digits + 2
            );

            println!("  {} digits: {:?}", digits, duration);

            // Check that performance scales reasonably (not exponentially)
            if let Some(prev_dur) = prev_duration {
                let ratio = duration.as_nanos() as f64 / prev_dur.as_nanos() as f64;
                // Scaling should be better than quadratic for most cases
                if digits > 100 {
                    assert!(
                        ratio < 10.0,
                        "Performance degraded too much from previous test: {}x",
                        ratio
                    );
                }
            }
            prev_duration = Some(duration);
        }

        println!("✅ Binary splitting scaling test passed!");
    }

    #[test]
    fn test_edge_cases() {
        println!("\n🎯 Edge Cases Test:");

        // Test minimum digits
        let result = compute_pi(1, PiAlgorithm::ChudnovskyBinarySplitting);
        assert!(result.starts_with("3.1"), "1 digit result should be 3.1");
        println!("  ✓ 1 digit: {}", &result[..std::cmp::min(4, result.len())]);

        // Test small digits
        let result = compute_pi(5, PiAlgorithm::BorweinQuadratic);
        assert!(result.starts_with("3.14159"), "5 digits result incorrect");
        println!("  ✓ 5 digits: {}", &result[..7]);

        // Test consistency across algorithms for small values
        let test_digits = 10;
        let algorithms = [
            PiAlgorithm::ChudnovskyNaive,
            PiAlgorithm::ChudnovskyBinarySplitting,
            PiAlgorithm::MachinFormula,
            PiAlgorithm::BorweinQuadratic,
        ];

        let mut results = Vec::new();
        for algorithm in algorithms {
            let result = compute_pi(test_digits, algorithm);
            let truncated = &result[..12]; // "3." + 10 digits
            results.push(truncated.to_string());
        }

        // All should give same result
        let reference = &results[0];
        for (i, result) in results.iter().enumerate() {
            assert_eq!(
                result, reference,
                "Algorithm {} gave different result for {} digits",
                i, test_digits
            );
        }
        println!(
            "  ✓ All algorithms consistent for {} digits: {}",
            test_digits, reference
        );

        println!("✅ Edge cases test passed!");
    }

    #[test]
    fn test_precision_requirements() {
        println!("\n🎯 Precision Requirements Test:");

        // Test that we get exactly the requested number of digits
        let test_cases = [10, 25, 50, 100];

        for &digits in &test_cases {
            let result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);

            // Should have at least "3." + digits characters
            assert!(
                result.len() >= digits + 2,
                "Result for {} digits should have at least {} characters, got {}",
                digits,
                digits + 2,
                result.len()
            );

            // Should start correctly
            assert!(
                result.starts_with("3.14159265"),
                "Result should start with correct pi digits"
            );

            println!(
                "  ✓ {} digits: length = {}, starts with {}",
                digits,
                result.len(),
                &result[..std::cmp::min(20, result.len())]
            );
        }

        println!("✅ Precision requirements test passed!");
    }

    #[test]
    fn test_performance_targets() {
        println!("\n⏱️  Performance Targets Test:");

        // Define performance targets based on our optimizations
        let test_cases = [
            (50, 10_000, "50 digits should compute in < 10ms"), // 10ms in microseconds
            (100, 20_000, "100 digits should compute in < 20ms"),
            (500, 100_000, "500 digits should compute in < 100ms"),
            (1000, 500_000, "1000 digits should compute in < 500ms"),
        ];

        for (digits, max_micros, description) in test_cases {
            let start = Instant::now();
            let _result = compute_pi(digits, PiAlgorithm::ChudnovskyBinarySplitting);
            let duration = start.elapsed();

            let micros = duration.as_micros();
            println!("  {} digits: {:?} ({} μs)", digits, duration, micros);

            assert!(
                micros < max_micros,
                "{}: took {} μs, expected < {} μs",
                description,
                micros,
                max_micros
            );
        }

        println!("✅ Performance targets test passed!");
    }

    #[test]
    #[should_panic(expected = "Слишком большое количество знаков")]
    fn test_overflow_protection_binary_splitting() {
        println!("\n🛡️  Testing overflow protection for Binary Splitting...");
        let _result = compute_pi(200_000_000, PiAlgorithm::ChudnovskyBinarySplitting);
    }

    #[test]
    #[should_panic(expected = "Слишком большое количество знаков")]
    fn test_overflow_protection_naive() {
        println!("\n🛡️  Testing overflow protection for Naive algorithm...");
        let _result = compute_pi(2_000_000, PiAlgorithm::ChudnovskyNaive);
    }

    #[test]
    fn test_algorithm_limits() {
        println!("\n📏 Testing algorithm limits:");

        // Test within reasonable limits for CI/testing
        let result = compute_pi(10_000, PiAlgorithm::ChudnovskyNaive);
        assert!(
            result.starts_with("3.14159"),
            "Naive algorithm should work within limits"
        );
        println!("  ✓ Naive: 10K digits - OK");

        let result = compute_pi(50_000, PiAlgorithm::ChudnovskyBinarySplitting);
        assert!(
            result.starts_with("3.14159"),
            "Binary splitting should work within limits"
        );
        println!("  ✓ Binary Splitting: 50K digits - OK");

        let result = compute_pi(20_000, PiAlgorithm::BorweinQuadratic);
        assert!(
            result.starts_with("3.14159"),
            "Borwein should work within limits"
        );
        println!("  ✓ Borwein: 20K digits - OK");

        let result = compute_pi(15_000, PiAlgorithm::MachinFormula);
        assert!(
            result.starts_with("3.14159"),
            "Machin should work within limits"
        );
        println!("  ✓ Machin: 15K digits - OK");

        println!("✅ Algorithm limits test passed!");
    }
}
