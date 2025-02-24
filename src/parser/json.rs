use std::{any::Any, error::Error};

use serde_json::Value;

use crate::ingest::LogLine;
use crate::error::LogAnalyzerError;

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

    
    fn normalize_json(content: &str) -> String {
        // Remove any leading/trailing whitespace and newlines
        content.trim()
            .replace('\n', "")
            .replace('\r', "")
            .replace("  ", " ")
    }
}

#[async_trait::async_trait]
impl LogParser for JsonParser {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error + Send + Sync>> {
        // Try to parse the normalized JSON string
        let normalized = JsonParser::normalize_json(&log_line.content);
        let json_value = serde_json::from_str::<Value>(&normalized)
            .map_err(|e| LogAnalyzerError::LogFromatInvalid(e.to_string()))?;

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

    fn as_any(&self) -> &dyn Any {
        self
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn parse_valid_json() {
        let parser = JsonParser::new();

        let json_content = r#"{
            "message": "This is a test logging info",
            "level" : "info",
            "tags": "dev",
            "username": "Prashant",
            "timestamp": "2024-03-15T10:00:00Z"
        }"#;


        let log_line = LogLine {
            content: json_content.to_string(),
            source: "test".to_string(),
            timestamp : chrono::Utc::now()
        };

        let res = parser.parse(log_line).await.unwrap();

        assert_eq!(res.message , "This is a test logging info".to_string());
        assert_eq!(res.level , Some(Level::Info));


        if let Value::Object(map_metadata) = res.metadata {
            assert_eq!(map_metadata.get("tags").unwrap().as_str().unwrap() , "dev" );
        }
        else {
            panic!("metadata is not an object");
        }


    }
}