use std::error::Error;

use serde_json::Value;

use crate::ingest::LogLine;

use super::{Level, LogParser, ParsedLog};

pub struct JsonParser;

impl JsonParser {
    pub fn new() -> Self {
        Self
    }

    fn level_parse(level_str : &str) -> Option<Level> {
        match level_str.to_lowercase().as_str() {
            "info" => Some(Level::Info),
            "error" => Some(Level::Error),
            "warn" | "warning" => Some(Level::Warn),
            _ => None
        }
    }
}

#[async_trait::async_trait]
impl LogParser for JsonParser {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error>> {

        let json_value = serde_json::from_str::<Value>(&log_line.content)?;

        let message = json_value.get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

        
        let level = json_value.get("level")
                        .and_then(|v| v.as_str())
                        .and_then(JsonParser::level_parse);


        let timestamp = json_value.get("timestamp")
                            .and_then(|v| v.as_str())
                            .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
                            .map(|dt| dt.with_timezone(&chrono::Utc))
                            .or(Some(log_line.timestamp));



        
        let mut metadata = json_value;

        if let Value::Object(map) = &mut metadata {
            map.remove("message");
            map.remove("level");
            map.remove("timestamp");
        }

        Ok(ParsedLog {
            timestamp,
            level,
            message,
            metadata
        })


    }
}