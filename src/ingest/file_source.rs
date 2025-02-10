use std::error::Error;
use std::path::{Path, PathBuf};

use chrono::Utc;
use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};

use super::LogSource;
use super::LogLine;
use async_trait;

pub struct FileLogSource {
    path: PathBuf,
    reader: Option<BufReader<File>>
}

impl FileLogSource {
    pub fn new<P>(path: P) -> Self 
    where P: AsRef<Path> {
        
        Self 
        { path: path.as_ref().to_owned(),
          reader: None
        }
    }
}

#[async_trait::async_trait]
impl LogSource for FileLogSource {
    
    async fn read_line (&mut self) -> Result<Option<LogLine>, Box<dyn Error>> {


        if let Some(reader) = &mut self.reader {

            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0{
                return Ok(None)
            }

            Ok(Some(
                LogLine {
                    content: line.trim().to_string(),
                    source: self.path.to_string_lossy().to_string(), 
                    timestamp: Utc::now()
                }
            ))
        }
        else {
            Err("Source not initialised".into())
        }
        
    }
}