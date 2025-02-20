use std::error::Error;
use std::any::Any;

use crate::ingest::LogLine;

pub mod registry;
pub mod plain_text;
pub mod json;


#[derive(Debug,PartialEq, Clone)]
pub struct ParsedLog {
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub level: Option<Level>,
    pub message: String,
    pub metadata: serde_json::Value
}

#[derive(Debug, PartialEq, Clone)]
pub enum Level {
    Info,
    Error,
    Warn
}

#[async_trait::async_trait]
pub trait LogParser : 'static + Send + Sync {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error + Send + Sync>>;
    fn as_any(&self) -> &dyn Any;
}