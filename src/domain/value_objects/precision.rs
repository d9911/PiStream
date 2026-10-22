use super::Digits;
use std::fmt;

/// Value object representing the precision in bits for calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Precision(u32);

impl Precision {
    /// Creates a new Precision value object
    pub fn new(bits: u32) -> Result<Self, String> {
        if bits < 32 {
            return Err("Точность должна быть не менее 32 бит".to_string());
        }
        if bits > u32::MAX / 2 {
            return Err("Слишком высокая точность".to_string());
        }
        Ok(Precision(bits))
    }

    /// Calculates precision from digits using the standard formula
    pub fn from_digits(digits: Digits) -> Self {
        let base_prec = ((digits.value() as f64) * 3.3219280948873626).ceil() as u64;
        let prec_with_buffer = base_prec.saturating_add(128);
        let final_prec = std::cmp::min(prec_with_buffer, u32::MAX as u64) as u32;

        Precision(final_prec)
    }

    /// Calculates precision for ultra scale computations
    pub fn from_digits_ultra_scale(digits: Digits) -> Self {
        if digits.value() > 1_000_000 {
            let base_prec = ((digits.value() as f64) * 3.3219280948873626).ceil() as u64;
            let extra_bits = (digits.value() as f64).log10().ceil() as u64 * 10;
            let total_prec = base_prec.saturating_add(extra_bits).saturating_add(128);
            let final_prec = std::cmp::min(total_prec, u32::MAX as u64) as u32;
            Precision(final_prec)
        } else {
            Self::from_digits(digits)
        }
    }

    /// Returns the underlying value in bits
    pub fn bits(&self) -> u32 {
        self.0
    }

    /// Returns the precision in bytes
    pub fn bytes(&self) -> u64 {
        (self.0 as u64).div_ceil(8) // Round up to nearest byte
    }

    /// Returns estimated memory usage in MB
    pub fn estimated_memory_mb(&self) -> f64 {
        self.bytes() as f64 / (1024.0 * 1024.0)
    }

    /// Returns estimated memory usage in GB
    pub fn estimated_memory_gb(&self) -> f64 {
        self.estimated_memory_mb() / 1024.0
    }

    /// Returns true if this is a high precision computation (> 1M bits)
    pub fn is_high_precision(&self) -> bool {
        self.0 > 1_000_000
    }

    /// Returns true if this is an ultra precision computation (> 100M bits)
    pub fn is_ultra_precision(&self) -> bool {
        self.0 > 100_000_000
    }
}

impl fmt::Display for Precision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 1_000_000 {
            write!(f, "{:.1}M бит", self.0 as f64 / 1_000_000.0)
        } else if self.0 >= 1_000 {
            write!(f, "{:.1}K бит", self.0 as f64 / 1_000.0)
        } else {
            write!(f, "{} бит", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_creation() {
        let precision = Precision::new(1024).unwrap();
        assert_eq!(precision.bits(), 1024);

        let invalid = Precision::new(16);
        assert!(invalid.is_err());

        let too_high = Precision::new(u32::MAX);
        assert!(too_high.is_err());
    }

    #[test]
    fn test_precision_from_digits() {
        let digits = Digits::new(1000).unwrap();
        let precision = Precision::from_digits(digits);

        // Should be approximately 3322 + 128 = 3450 bits
        assert!(precision.bits() > 3000);
        assert!(precision.bits() < 4000);
    }

    #[test]
    fn test_precision_from_digits_ultra_scale() {
        let digits = Digits::new(2_000_000).unwrap();
        let precision = Precision::from_digits_ultra_scale(digits);

        // Should be higher than regular precision due to extra stability bits
        let regular_precision = Precision::from_digits(digits);
        assert!(precision.bits() > regular_precision.bits());
    }

    #[test]
    fn test_precision_memory_calculation() {
        let precision = Precision::new(8192).unwrap(); // 8K bits = 1KB
        assert_eq!(precision.bytes(), 1024);
        assert!((precision.estimated_memory_mb() - 0.001).abs() < 0.0001);
    }

    #[test]
    fn test_precision_size_checks() {
        let small = Precision::new(1024).unwrap();
        assert!(!small.is_high_precision());
        assert!(!small.is_ultra_precision());

        let high = Precision::new(2_000_000).unwrap();
        assert!(high.is_high_precision());
        assert!(!high.is_ultra_precision());

        let ultra = Precision::new(200_000_000).unwrap();
        assert!(ultra.is_high_precision());
        assert!(ultra.is_ultra_precision());
    }

    #[test]
    fn test_precision_display() {
        let small = Precision::new(512).unwrap();
        assert_eq!(format!("{}", small), "512 бит");

        let medium = Precision::new(5120).unwrap();
        assert_eq!(format!("{}", medium), "5.1K бит");

        let large = Precision::new(5_120_000).unwrap();
        assert_eq!(format!("{}", large), "5.1M бит");
    }
}
