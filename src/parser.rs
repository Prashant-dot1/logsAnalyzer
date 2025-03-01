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
    pub metadata: serde_json::Value,
    pub service_name: Option<String>,     // Name of the service generating the log
    pub trace_id: Option<String>,         // For distributed tracing
    pub span_id: Option<String>,          // For tracking specific operations
    pub duration_ms: Option<f64>,         // Operation duration if applicable
    pub host: Option<String>,             // Host machine identifier
    pub environment: Option<String>,      // e.g., "production", "staging"
    pub version: Option<String>,          // Application version
}

#[derive(Debug, PartialEq, Clone)]
pub enum Level {
    Info,
    Error,
    Warn,
    Debug,     // For detailed debugging information
    Trace,     // For very detailed diagnostic information
    Critical,  // For critical errors that need immediate attention
    Fatal      // For errors that cause the application to crash
}

#[async_trait::async_trait]
pub trait LogParser : 'static + Send + Sync {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error + Send + Sync>>;
    fn as_any(&self) -> &dyn Any;
}

impl ParsedLog {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "timestamp": self.timestamp.map(|t| t.to_rfc3339()),
            "level": self.level.as_ref().map(|l| format!("{:?}", l)),
            "message": self.message,
            "metadata": self.metadata,
            "service_name": self.service_name,
            "trace_id": self.trace_id,
            "span_id": self.span_id,
            "duration_ms": self.duration_ms,
            "host": self.host,
            "environment": self.environment,
            "version": self.version,
        })
    }

    pub fn severity_level(&self) -> u8 {
        match self.level {
            Some(Level::Fatal) => 0,
            Some(Level::Critical) => 1,
            Some(Level::Error) => 2,
            Some(Level::Warn) => 3,
            Some(Level::Info) => 4,
            Some(Level::Debug) => 5,
            Some(Level::Trace) => 6,
            None => 7,
        }
    }
}

impl Default for ParsedLog {
    fn default() -> Self {
        Self {
            timestamp: None,
            level: None,
            message: String::new(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            service_name: None,
            trace_id: None,
            span_id: None,
            duration_ms: None,
            host: None,
            environment: None,
            version: None,
        }
    }
}

impl ParsedLog {
    pub fn new(message: String) -> Self {
        Self {
            message,
            timestamp: Some(chrono::Utc::now()),
            ..Default::default()
        }
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_service(mut self, service: impl Into<String>) -> Self {
        self.service_name = Some(service.into());
        self
    }

    pub fn with_trace_context(mut self, trace_id: impl Into<String>, span_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self.span_id = Some(span_id.into());
        self
    }
}

pub trait LogFormatter {
    fn format(&self, log: &ParsedLog) -> String;
}

// Example implementations could be added for different formats:
pub struct JsonFormatter;
pub struct PlainTextFormatter;
pub struct CEFFormatter;  // Common Event Format