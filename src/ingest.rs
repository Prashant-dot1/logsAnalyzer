use async_trait::async_trait;
use std::error::Error;

/* inner modules */
pub mod file_source;
pub mod network_source;

#[derive(Clone)]
pub struct LogLine {
    pub content: String,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>
}

#[async_trait]
pub trait LogSource : Send + Sync {
    // some way to initlialise the log source
    async fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn read_line(&mut self) -> Result<Option<LogLine>, Box<dyn Error + Send + Sync>>;

    async fn close(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
}