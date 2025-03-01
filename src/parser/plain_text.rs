use super::{LogParser, ParsedLog};
use crate::ingest::LogLine;
use std::error::Error;
use std::any::Any;

pub struct PlainTextParser;

impl PlainTextParser {
    pub fn new() -> Self {
        Self
    }
}


#[async_trait::async_trait]
impl LogParser for PlainTextParser {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error + Send + Sync>> {

        Ok(ParsedLog {
            timestamp : Some(chrono::Utc::now()),
            level: None,
            message: log_line.content,
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            ..ParsedLog::default()
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}