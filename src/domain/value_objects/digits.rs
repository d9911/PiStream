use std::fmt;

/// Value object representing the number of digits to compute
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digits(usize);

impl Digits {
    /// Creates a new Digits value object
    pub fn new(value: usize) -> Result<Self, String> {
        if value == 0 {
            return Err("Количество знаков должно быть больше нуля".to_string());
        }
        Ok(Digits(value))
    }

    /// Creates a new Digits value object with default validation
    pub fn new_with_default(value: usize, default: usize) -> Self {
        if value == 0 {
            Digits(default)
        } else {
            Digits(value)
        }
    }

    /// Returns the underlying value
    pub fn value(&self) -> usize {
        self.0
    }

    /// Returns true if this represents a large computation (>= 1M digits)
    pub fn is_large_computation(&self) -> bool {
        self.0 >= 1_000_000
    }

    /// Returns true if this represents a huge computation (>= 1B digits)
    pub fn is_huge_computation(&self) -> bool {
        self.0 >= 1_000_000_000
    }

    /// Returns true if this represents a record-breaking computation (>= 1T digits)
    pub fn is_record_breaking(&self) -> bool {
        self.0 >= 1_000_000_000_000
    }

    /// Returns a human-readable description of the computation size
    pub fn size_description(&self) -> &'static str {
        match self.0 {
            0..=999 => "малое вычисление",
            1_000..=99_999 => "среднее вычисление",
            100_000..=999_999 => "большое вычисление",
            1_000_000..=99_999_999 => "очень большое вычисление",
            100_000_000..=999_999_999 => "огромное вычисление",
            1_000_000_000..=999_999_999_999 => "миллиардное вычисление",
            _ => "рекордное вычисление",
        }
    }

    /// Returns formatted string for display
    pub fn formatted(&self) -> String {
        if self.0 >= 1_000_000_000_000 {
            format!("{:.1} трлн", self.0 as f64 / 1_000_000_000_000.0)
        } else if self.0 >= 1_000_000_000 {
            format!("{:.1} млрд", self.0 as f64 / 1_000_000_000.0)
        } else if self.0 >= 1_000_000 {
            format!("{:.1} млн", self.0 as f64 / 1_000_000.0)
        } else if self.0 >= 1_000 {
            format!("{:.1} тыс", self.0 as f64 / 1_000.0)
        } else {
            self.0.to_string()
        }
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Digits {
    fn from(value: usize) -> Self {
        Digits::new_with_default(value, 1000)
    }
}

impl TryFrom<String> for Digits {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = value
            .trim()
            .parse::<usize>()
            .map_err(|_| "Неверный формат числа".to_string())?;
        Digits::new(parsed)
    }
}

impl TryFrom<&str> for Digits {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value
            .trim()
            .parse::<usize>()
            .map_err(|_| "Неверный формат числа".to_string())?;
        Digits::new(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_creation() {
        let digits = Digits::new(1000).unwrap();
        assert_eq!(digits.value(), 1000);

        let invalid = Digits::new(0);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_digits_with_default() {
        let digits = Digits::new_with_default(0, 1000);
        assert_eq!(digits.value(), 1000);

        let digits = Digits::new_with_default(500, 1000);
        assert_eq!(digits.value(), 500);
    }

    #[test]
    fn test_digits_size_checks() {
        let small = Digits::new(500).unwrap();
        assert!(!small.is_large_computation());
        assert!(!small.is_huge_computation());
        assert!(!small.is_record_breaking());

        let large = Digits::new(2_000_000).unwrap();
        assert!(large.is_large_computation());
        assert!(!large.is_huge_computation());
        assert!(!large.is_record_breaking());

        let huge = Digits::new(2_000_000_000).unwrap();
        assert!(huge.is_large_computation());
        assert!(huge.is_huge_computation());
        assert!(!huge.is_record_breaking());

        let record = Digits::new(2_000_000_000_000).unwrap();
        assert!(record.is_large_computation());
        assert!(record.is_huge_computation());
        assert!(record.is_record_breaking());
    }

    #[test]
    fn test_digits_formatted() {
        assert_eq!(Digits::new(500).unwrap().formatted(), "500");
        assert_eq!(Digits::new(1_500).unwrap().formatted(), "1.5 тыс");
        assert_eq!(Digits::new(2_500_000).unwrap().formatted(), "2.5 млн");
        assert_eq!(Digits::new(3_500_000_000).unwrap().formatted(), "3.5 млрд");
        assert_eq!(
            Digits::new(4_500_000_000_000).unwrap().formatted(),
            "4.5 трлн"
        );
    }

    #[test]
    fn test_digits_from_string() {
        let digits = Digits::try_from("1000".to_string()).unwrap();
        assert_eq!(digits.value(), 1000);

        let digits = Digits::try_from("  1500  ").unwrap();
        assert_eq!(digits.value(), 1500);

        let invalid = Digits::try_from("abc".to_string());
        assert!(invalid.is_err());

        let zero = Digits::try_from("0".to_string());
        assert!(zero.is_err());
    }
}
