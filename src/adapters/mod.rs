pub mod input;
pub mod output;
pub mod algorithms;

// Re-export commonly used adapters
pub use input::{CliHandler, ConsoleProgress};
pub use output::{FileStorage, ConsolePresenter};
pub use algorithms::AlgorithmRepository;

