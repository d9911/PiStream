pub mod algorithms;
pub mod input;
pub mod output;

// Re-export commonly used adapters
pub use algorithms::AlgorithmRepository;
pub use input::{CliHandler, ConsoleProgress};
pub use output::{ConsolePresenter, FileStorage};
