use async_trait::async_trait;

/// Output port for file writing operations
#[async_trait]
pub trait FileWriter {
    /// Writes content to a file
    async fn write_file(&self, filename: &str, content: &str) -> Result<(), String>;

    /// Checks if a file exists
    async fn file_exists(&self, filename: &str) -> bool;

    /// Generates a unique filename for π results
    fn generate_pi_filename(&self, digits: usize, algorithm_name: &str) -> String;
}
