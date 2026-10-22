use crate::ports::input::ProgressReporter;
use indicatif::{ProgressBar, ProgressStyle};

/// Console progress reporter adapter
pub struct ConsoleProgress {
    progress_bar: Option<ProgressBar>,
}

impl ConsoleProgress {
    pub fn new() -> Self {
        Self { progress_bar: None }
    }

    pub fn new_spinner() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );

        Self {
            progress_bar: Some(pb),
        }
    }
}

impl Default for ConsoleProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressReporter for ConsoleProgress {
    fn start_computation(&self, total_iterations: u64, message: &str) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_length(total_iterations);
            pb.set_message(message.to_string());
        }
    }

    fn update_progress(&self, current: u64, message: &str) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_position(current);
            pb.set_message(message.to_string());
        }
    }

    fn finish_computation(&self, message: &str) {
        if let Some(ref pb) = self.progress_bar {
            pb.finish_with_message(message.to_string());
        }
    }

    fn set_message(&self, message: &str) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_message(message.to_string());
        }
    }

    fn increment(&self) {
        if let Some(ref pb) = self.progress_bar {
            pb.inc(1);
        }
    }

    fn set_position(&self, position: u64) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_position(position);
        }
    }
}

impl Drop for ConsoleProgress {
    fn drop(&mut self) {
        if let Some(ref pb) = self.progress_bar {
            pb.finish_and_clear();
        }
    }
}
