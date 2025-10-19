use crate::ports::output::FileWriter;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

/// File storage adapter for writing π computation results
pub struct FileStorage;

impl FileStorage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileWriter for FileStorage {
    async fn write_file(&self, filename: &str, content: &str) -> Result<(), String> {
        fs::write(filename, content)
            .await
            .map_err(|e| format!("Ошибка при записи файла: {}", e))
    }

    async fn file_exists(&self, filename: &str) -> bool {
        Path::new(filename).exists()
    }

    fn generate_pi_filename(&self, digits: usize, algorithm_name: &str) -> String {
        let safe_algorithm_name = algorithm_name
            .replace(" ", "_")
            .replace("(", "")
            .replace(")", "")
            .replace("🚀", "")
            .replace("💾", "")
            .replace("🚨", "")
            .trim()
            .to_string();

        format!("pi_{}_digits_{}.txt", digits, safe_algorithm_name)
    }
}

