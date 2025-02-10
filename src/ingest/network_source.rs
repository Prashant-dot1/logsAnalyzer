use std::error::Error;

use async_trait::async_trait;
use chrono::Utc;
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpStream};

use super::{LogSource, LogLine};

pub struct NetworkLogSource {
    address: String,
    reader: Option<BufReader<TcpStream>>
}

impl NetworkLogSource {
    pub fn new(address : String) -> Self {
        Self { address, 
            reader: None 
        }
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

            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                return Ok(None)
            }

            Ok(Some(
                LogLine {
                    content: line.trim().to_string(),
                    source: format!("network:{}", self.address),
                    timestamp: Utc::now()
                }
            ))

        }
        else {
            Err("Source not initialised".into())
        }
    }

    async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.reader = None;
        Ok(())
    }
}