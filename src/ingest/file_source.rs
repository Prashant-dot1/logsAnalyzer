use std::error::Error;
use std::path::{Path, PathBuf};

use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};

use crate::error::LogAnalyzerError;

use super::LogSource;
use super::LogLine;
use async_trait;

pub struct FileLogSource {
    path: PathBuf,
    reader: Option<BufReader<File>>,
    buffer: String
}

impl FileLogSource {
    pub fn new<P>(path: P) -> Self 
    where P: AsRef<Path> {
        
        Self 
        { path: path.as_ref().to_owned(),
          reader: None,
            buffer: String::new()
        }
    }

    fn is_valid_json(content : &String) -> bool {
        serde_json::from_str::<serde_json::Value>(&content).is_ok()
    }
}

#[async_trait::async_trait]
impl LogSource for FileLogSource {

    async fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {

        let file = tokio::fs::File::open(&self.path).await
            .map_err(|e| LogAnalyzerError::Io(e))?;


        self.reader = Some(BufReader::new(file));

        Ok(())
    }
    
    async fn read_line (&mut self) -> Result<Option<LogLine>, Box<dyn Error + Send + Sync>> {


        if let Some(reader) = &mut self.reader {

            loop {
                let mut line = String::new();
                let bytes_read =reader.read_line(&mut line).await
                    .map_err(|e| LogAnalyzerError::Io(e))?;

                if bytes_read == 0 {
                    return Ok(None);
                }

                self.buffer.push_str(&line);
                if FileLogSource::is_valid_json(&self.buffer) {
                    let content = std::mem::take(&mut self.buffer);
                    return Ok(Some(LogLine {
                        content : content.trim().to_string(),
                        source: self.path.to_string_lossy().to_string(),
                        timestamp: chrono::Utc::now()
                    }));
                }
            }
        }
        else {
            Err(Box::new(LogAnalyzerError::SourceNotInitialized))
        }
    }

    async fn close(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.reader = None;
        Ok(())
    }
}