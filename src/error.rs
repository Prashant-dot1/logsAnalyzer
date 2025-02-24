use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogAnalyzerError { 

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Json parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Parser not found for the given log format")]
    ParserNotFound,

    #[error("Source not initialised")]
    SourceNotInitialized,

    #[error("Invalid log format: {0}")]
    LogFromatInvalid(String),

    #[error("network error: {0}")]
    NetworkError(String)
}