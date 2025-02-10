use std::error::Error;
use async_trait::async_trait;

/* inner modules */
pub mod file_source;

pub struct LogLine {
    pub content: String,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>
}

#[async_trait]
pub trait LogSource {
    // some way to initlialise the log source
    async fn init(&mut self) -> Result<(), Box<dyn Error>>;

    async fn read_line(&mut self) -> Result<Option<LogLine>, Box<dyn Error>>;

    async fn close(&mut self) -> Result<(), Box<dyn Error>>;
}