/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub default_digits: usize,
    pub default_algorithm_choice: u8,
    pub enable_file_saving: bool,
    pub show_detailed_stats: bool,
    pub show_achievements: bool,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            default_digits: 1000,
            default_algorithm_choice: 2, // Binary Splitting
            enable_file_saving: true,
            show_detailed_stats: true,
            show_achievements: true,
        }
    }

    pub fn production() -> Self {
        Self::new()
    }

    pub fn testing() -> Self {
        Self {
            default_digits: 100,
            default_algorithm_choice: 2,
            enable_file_saving: false,
            show_detailed_stats: false,
            show_achievements: false,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
