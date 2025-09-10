use crate::domain::entities::{Algorithm, PiComputation};
use crate::domain::services::ComputationEstimate;
use crate::ports::input::AlgorithmInfo;

/// Output port for UI presentation
pub trait UiPresenter {
    /// Shows the welcome message
    fn show_welcome(&self);

    /// Shows available algorithms
    fn show_algorithms(&self, algorithms: &[AlgorithmInfo]);

    /// Prompts for algorithm selection
    fn prompt_algorithm_selection(&self) -> String;

    /// Prompts for digits input
    fn prompt_digits_input(&self) -> String;

    /// Shows computation start message
    fn show_computation_start(&self, algorithm: Algorithm, digits: usize);

    /// Shows resource estimation
    fn show_resource_estimation(&self, estimate: &ComputationEstimate);

    /// Shows computation results
    fn show_results(&self, computation: &PiComputation);

    /// Shows computation statistics
    fn show_statistics(&self, computation: &PiComputation);

    /// Shows achievement message based on digits computed
    fn show_achievement(&self, digits: usize);

    /// Prompts for file saving
    fn prompt_save_to_file(&self) -> bool;

    /// Shows file save result
    fn show_save_result(&self, filename: &str, success: bool, error: Option<&str>);

    /// Shows error message
    fn show_error(&self, message: &str);

    /// Shows warning message
    fn show_warning(&self, message: &str);

    /// Shows info message
    fn show_info(&self, message: &str);

    /// Shows ultra scale warning
    fn show_ultra_scale_warning(&self);

    /// Prompts for ultra scale confirmation
    fn prompt_ultra_scale_confirmation(&self, digits: usize) -> bool;
}

