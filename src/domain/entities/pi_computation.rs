use super::Algorithm;
use crate::domain::value_objects::{Digits, Precision};
use std::time::{Duration, Instant};

/// Represents a π computation request and its results
#[derive(Debug, Clone)]
pub struct PiComputation {
    pub algorithm: Algorithm,
    pub digits: Digits,
    pub precision: Option<Precision>,
    pub result: Option<String>,
    pub computation_time: Option<Duration>,
    pub total_time: Option<Duration>,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
}

impl PiComputation {
    /// Creates a new π computation request
    pub fn new(algorithm: Algorithm, digits: Digits) -> Self {
        Self {
            algorithm,
            digits,
            precision: None,
            result: None,
            computation_time: None,
            total_time: None,
            started_at: None,
            completed_at: None,
        }
    }

    /// Marks the computation as started
    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }

    /// Marks the computation as completed with result
    pub fn complete(&mut self, result: String, computation_time: Duration) {
        self.result = Some(result);
        self.computation_time = Some(computation_time);
        self.completed_at = Some(Instant::now());

        if let Some(start_time) = self.started_at {
            self.total_time = Some(start_time.elapsed());
        }
    }

    /// Returns true if the computation is completed
    pub fn is_completed(&self) -> bool {
        self.result.is_some()
    }

    /// Returns the computation speed in digits per second
    pub fn speed_digits_per_second(&self) -> Option<f64> {
        if let Some(total_time) = self.total_time {
            if total_time.as_secs_f64() > 0.0 {
                Some(self.digits.value() as f64 / total_time.as_secs_f64())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the computation speed in digits per millisecond
    pub fn speed_digits_per_millisecond(&self) -> Option<f64> {
        if let Some(total_time) = self.total_time {
            let millis = total_time.as_millis() as f64;
            if millis > 0.0 {
                Some(self.digits.value() as f64 / millis)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the efficiency in microseconds per digit
    pub fn efficiency_microseconds_per_digit(&self) -> Option<f64> {
        if let Some(total_time) = self.total_time {
            let micros = total_time.as_micros() as f64;
            Some(micros / self.digits.value() as f64)
        } else {
            None
        }
    }

    /// Returns a truncated version of the result for display
    pub fn display_result(&self, max_length: Option<usize>) -> Option<String> {
        if let Some(ref result) = self.result {
            if let Some(max_len) = max_length {
                if result.len() > max_len {
                    let chars: Vec<char> = result.chars().collect();
                    if chars.len() > 60 {
                        let first_part: String = chars[..52].iter().collect(); // включая "3."
                        let last_part: String = chars[chars.len() - 10..].iter().collect();
                        return Some(format!("{}...{}", first_part, last_part));
                    }
                }
            }
            Some(result.clone())
        } else {
            None
        }
    }

    /// Validates that the algorithm can handle the requested digits
    pub fn validate(&self) -> Result<(), String> {
        if !self.algorithm.can_handle_digits(self.digits.value()) {
            return Err(format!(
                "Алгоритм {} не поддерживает {} знаков! Максимум: {} знаков",
                self.algorithm.display_name(),
                self.digits.value(),
                self.algorithm.max_digits()
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_computation_new() {
        let digits = Digits::new(1000).unwrap();
        let computation = PiComputation::new(Algorithm::ChudnovskyBinarySplitting, digits);

        assert_eq!(computation.algorithm, Algorithm::ChudnovskyBinarySplitting);
        assert_eq!(computation.digits.value(), 1000);
        assert!(!computation.is_completed());
    }

    #[test]
    fn test_pi_computation_lifecycle() {
        let digits = Digits::new(1000).unwrap();
        let mut computation = PiComputation::new(Algorithm::ChudnovskyBinarySplitting, digits);

        // Start computation
        computation.start();
        assert!(computation.started_at.is_some());

        // Complete computation
        let result = "3.14159...".to_string();
        let computation_time = Duration::from_millis(100);
        computation.complete(result.clone(), computation_time);

        assert!(computation.is_completed());
        assert_eq!(computation.result, Some(result));
        assert_eq!(computation.computation_time, Some(computation_time));
        assert!(computation.completed_at.is_some());
        assert!(computation.total_time.is_some());
    }

    #[test]
    fn test_pi_computation_validation() {
        let digits = Digits::new(2_000_000).unwrap();
        let computation = PiComputation::new(Algorithm::ChudnovskyNaive, digits);

        // Should fail validation because naive algorithm can't handle 2M digits
        assert!(computation.validate().is_err());

        // Should pass with binary splitting
        let digits = Digits::new(1_000_000).unwrap();
        let computation = PiComputation::new(Algorithm::ChudnovskyBinarySplitting, digits);
        assert!(computation.validate().is_ok());
    }

    #[test]
    fn test_pi_computation_speed_calculation() {
        let digits = Digits::new(1000).unwrap();
        let mut computation = PiComputation::new(Algorithm::ChudnovskyBinarySplitting, digits);

        computation.start();
        std::thread::sleep(Duration::from_millis(10));
        computation.complete("3.14159...".to_string(), Duration::from_millis(5));

        let speed = computation.speed_digits_per_second();
        assert!(speed.is_some());
        assert!(speed.unwrap() > 0.0);

        let efficiency = computation.efficiency_microseconds_per_digit();
        assert!(efficiency.is_some());
        assert!(efficiency.unwrap() > 0.0);
    }
}
