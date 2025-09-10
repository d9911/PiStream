use std::fmt;

/// Represents the different algorithms available for computing π
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    ChudnovskyNaive,
    ChudnovskyBinarySplitting,
    MachinFormula,
    BorweinQuadratic,
    ChudnovskyUltraScale,
    ChudnovskyUltraScaleStreaming,
}

impl Algorithm {
    /// Returns the maximum number of digits this algorithm can handle
    pub fn max_digits(&self) -> usize {
        match self {
            Algorithm::ChudnovskyNaive => 1_000_000,
            Algorithm::ChudnovskyBinarySplitting => 100_000_000,
            Algorithm::MachinFormula => 5_000_000,
            Algorithm::BorweinQuadratic => 2_000_000,
            Algorithm::ChudnovskyUltraScale => usize::MAX,
            Algorithm::ChudnovskyUltraScaleStreaming => usize::MAX,
        }
    }

    /// Returns the display name of the algorithm
    pub fn display_name(&self) -> &'static str {
        match self {
            Algorithm::ChudnovskyNaive => "Чудновский (наивный)",
            Algorithm::ChudnovskyBinarySplitting => "Чудновский (Binary Splitting)",
            Algorithm::MachinFormula => "Формула Мачина",
            Algorithm::BorweinQuadratic => "Алгоритм Борвейна",
            Algorithm::ChudnovskyUltraScale => "🚀 ULTRA SCALE Чудновский",
            Algorithm::ChudnovskyUltraScaleStreaming => "💾 ULTRA SCALE STREAMING Чудновский",
        }
    }

    /// Returns a description of the algorithm
    pub fn description(&self) -> &'static str {
        match self {
            Algorithm::ChudnovskyNaive => "стандартная реализация (макс: 1M знаков)",
            Algorithm::ChudnovskyBinarySplitting => "быстрый для больших чисел (макс: 100M знаков)",
            Algorithm::MachinFormula => "классический быстрый метод (макс: 5M знаков)",
            Algorithm::BorweinQuadratic => "квадратичная сходимость (макс: 2M знаков)",
            Algorithm::ChudnovskyUltraScale => "для триллионов знаков (макс: 100M)",
            Algorithm::ChudnovskyUltraScaleStreaming => "потоковое сохранение для экстремальных вычислений (макс: ∞)",
        }
    }

    /// Returns true if this is the ultra scale algorithm
    pub fn is_ultra_scale(&self) -> bool {
        matches!(self, Algorithm::ChudnovskyUltraScale | Algorithm::ChudnovskyUltraScaleStreaming)
    }

    /// Validates if the algorithm can handle the requested number of digits
    pub fn can_handle_digits(&self, digits: usize) -> bool {
        digits <= self.max_digits()
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl From<u8> for Algorithm {
    fn from(value: u8) -> Self {
        match value {
            1 => Algorithm::ChudnovskyNaive,
            2 => Algorithm::ChudnovskyBinarySplitting,
            3 => Algorithm::MachinFormula,
            4 => Algorithm::BorweinQuadratic,
            5 => Algorithm::ChudnovskyUltraScale,
            6 => Algorithm::ChudnovskyUltraScaleStreaming,
            _ => Algorithm::ChudnovskyBinarySplitting, // Default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_max_digits() {
        assert_eq!(Algorithm::ChudnovskyNaive.max_digits(), 1_000_000);
        assert_eq!(Algorithm::ChudnovskyBinarySplitting.max_digits(), 100_000_000);
        assert_eq!(Algorithm::MachinFormula.max_digits(), 5_000_000);
        assert_eq!(Algorithm::BorweinQuadratic.max_digits(), 2_000_000);
        assert_eq!(Algorithm::ChudnovskyUltraScale.max_digits(), usize::MAX);
    }

    #[test]
    fn test_algorithm_can_handle_digits() {
        let naive = Algorithm::ChudnovskyNaive;
        assert!(naive.can_handle_digits(500_000));
        assert!(!naive.can_handle_digits(2_000_000));

        let ultra = Algorithm::ChudnovskyUltraScale;
        assert!(ultra.can_handle_digits(1_000_000_000));
    }

    #[test]
    fn test_algorithm_from_u8() {
        assert_eq!(Algorithm::from(1), Algorithm::ChudnovskyNaive);
        assert_eq!(Algorithm::from(2), Algorithm::ChudnovskyBinarySplitting);
        assert_eq!(Algorithm::from(3), Algorithm::MachinFormula);
        assert_eq!(Algorithm::from(4), Algorithm::BorweinQuadratic);
        assert_eq!(Algorithm::from(5), Algorithm::ChudnovskyUltraScale);
        assert_eq!(Algorithm::from(99), Algorithm::ChudnovskyBinarySplitting); // Default
    }

    #[test]
    fn test_algorithm_is_ultra_scale() {
        assert!(!Algorithm::ChudnovskyNaive.is_ultra_scale());
        assert!(!Algorithm::ChudnovskyBinarySplitting.is_ultra_scale());
        assert!(!Algorithm::MachinFormula.is_ultra_scale());
        assert!(!Algorithm::BorweinQuadratic.is_ultra_scale());
        assert!(Algorithm::ChudnovskyUltraScale.is_ultra_scale());
    }
}

