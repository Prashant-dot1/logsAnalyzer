use std::error::Error;

use async_trait::async_trait;
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpStream};

use super::{LogSource, LogLine};

pub struct NetworkLogSource {
    address: String,
    reader: Option<BufReader<TcpStream>>,
    buffer: String
}

impl NetworkLogSource {
    pub fn new(address : String) -> Self {
        Self { address, 
            reader: None ,
            buffer : String::new()
        }
    }

    fn is_valid_json(content : &String) -> bool {
        serde_json::from_str::<serde_json::Value>(content).is_ok()
    }
}


#[async_trait]
impl LogSource for NetworkLogSource {

    async fn init(&mut self) -> Result<(), Box<dyn Error>> {

        let stream = TcpStream::connect(&self.address).await?;
        self.reader = Some(BufReader::new(stream));

        Ok(())
    }

    async fn read_line(&mut self) -> Result<Option<LogLine>, Box<dyn Error>> {

        if let Some(reader) = &mut self.reader {

            loop {
                let mut line = String::new();
                let bytes_read = reader.read_line(&mut line).await?;

                if bytes_read == 0 {
                    return Ok(None);
                }

                self.buffer.push_str(&line);
                if NetworkLogSource::is_valid_json(&self.buffer) {
                    let content = std::mem::take(&mut self.buffer);
                    return Ok(Some(LogLine {
                        content: content.trim().to_string(),
                        source: format!("Network {}", self.address),
                        timestamp: chrono::Utc::now()
                    }));
                }

            }

        }
        else {
            Err("Source not initialised".into())
        }
    }

    async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.reader = None;
        self.buffer.clear();
        Ok(())
    }
}