// Hexagonal Architecture modules
pub mod adapters;
pub mod config;
pub mod domain;
pub mod ports;

// Re-export key types for easier access
pub use adapters::{
    AlgorithmRepository, CliHandler, ConsolePresenter, ConsoleProgress, FileStorage,
};
pub use config::AppConfig;
pub use domain::{Algorithm, ComputationEstimate, Digits, PiCalculator, PiComputation, Precision};
pub use ports::{
    AlgorithmInfo, FileWriter, PiComputationUseCase, PiRepository, ProgressReporter, UiPresenter,
};

// Re-export interrupt functions for signal handling
pub use adapters::algorithms::chudnovsky_ultra_scale_streaming::{request_interrupt, is_interrupt_requested, reset_interrupt};

// Legacy module for backward compatibility (will be removed later)
pub mod pi;

// Application facade for easy setup
pub struct PiStreamApp {
    config: AppConfig,
}

impl PiStreamApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(AppConfig::default())
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Creates the complete application with all dependencies wired up
    pub fn setup() -> (
        impl PiComputationUseCase + Send + Sync,
        impl UiPresenter,
        impl FileWriter,
        AppConfig,
    ) {
        let config = AppConfig::default();
        let algorithm_repository = AlgorithmRepository::new();
        let calculator = PiCalculator::new(algorithm_repository);
        let cli_handler = CliHandler::new(calculator);
        let presenter = ConsolePresenter::new();
        let file_writer = FileStorage::new();

        (cli_handler, presenter, file_writer, config)
    }
}

impl Default for PiStreamApp {
    fn default() -> Self {
        Self::with_default_config()
    }
}
