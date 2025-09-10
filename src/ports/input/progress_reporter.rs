/// Trait for reporting computation progress
pub trait ProgressReporter {
    /// Called when computation starts
    fn start_computation(&self, total_iterations: u64, message: &str);

    /// Called to update progress
    fn update_progress(&self, current: u64, message: &str);

    /// Called when computation completes
    fn finish_computation(&self, message: &str);

    /// Called to set a status message
    fn set_message(&self, message: &str);

    /// Called to increment progress by one
    fn increment(&self);

    /// Called to set the current position
    fn set_position(&self, position: u64);
}

