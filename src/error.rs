
#[derive(Debug)]
pub enum LogAnalyzerError {
    Io(#[from] std::io::Error),
    Json(#[from] std::io::Error)
}