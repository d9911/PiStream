pub mod entities;
pub mod services;
pub mod value_objects;

// Re-export commonly used types
pub use entities::{Algorithm, PiComputation};
pub use services::{ComputationEstimate, EstimatedTime, PiCalculator};
pub use value_objects::{Digits, Precision};
