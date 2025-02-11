use std::error::Error;

use crate::ingest::LogLine;

pub mod plain_text;


#[derive(Debug)]
pub struct ParsedLog {
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub level: Option<Level>,
    pub message: String,
    pub metadata: serde_json::Value
}

#[derive(Debug)]
pub enum Level {
    Info,
    Error,
    Warn
}

#[async_trait::async_trait]
pub trait LogParser {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error>>;
}