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

    fn try_extracting_json(content: &String) -> Option<(String, String)> {
        println!("Trying to extract JSON from: {}", content);
        
        let mut depth = 0;
        let mut within_json_string = false;
        let mut escape_char_next = false;
        let mut start_idx = None;
        let mut current_json = String::new();

        // First normalize the content by removing extra whitespace but preserving structure
        let normalized = content
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("");

        // finding the first opening brace
        for (i, c) in normalized.chars().enumerate() {
            if c == '{' && !within_json_string {
                start_idx = Some(i);
                break;
            }
        }

        if start_idx.is_none() {
            println!("No JSON start found");
            return None;
        }

        // start parsing from the opening brace
        for (i, c) in normalized[start_idx.unwrap()..].chars().enumerate() {
            current_json.push(c);

            if escape_char_next {
                escape_char_next = false;
                continue;
            }

            match c {
                '\\' if within_json_string => escape_char_next = true,
                '"' => within_json_string = !within_json_string,
                '{' if !within_json_string => depth += 1,
                '}' if !within_json_string => {
                    depth -= 1;
                    if depth == 0 {
                        if serde_json::from_str::<serde_json::Value>(&current_json).is_ok() {
                            let remainder = normalized[start_idx.unwrap() + i + 1..].to_string();
                            println!("Valid JSON found: {}", current_json);
                            println!("Remainder: {}", remainder);
                            return Some((current_json, remainder));
                        }
                    }
                },
                _ => {}
            }
        }

        println!("No complete JSON found");
        None

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

            // firstly check if the buffer already has something we need to check and extract
            if !self.buffer.is_empty() {
                if let Some((json, remainder)) = NetworkLogSource::try_extracting_json(&self.buffer) {
                    self.buffer = remainder;
                    return Ok(Some(LogLine { content:json, source: format!("network {}", self.address), timestamp: chrono::Utc::now() }));
                }
            }

            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                // here we need to check if the buffer now contains any json
                if !self.buffer.is_empty() {
                    let content = std::mem::take(&mut self.buffer);
                    return Ok(Some(LogLine { content, source: format!("network {}", self.address), timestamp: chrono::Utc::now() }));
                }
                return Ok(None);
            }

            self.buffer.push_str(&line);
            if let Some((json,remainder)) = NetworkLogSource::try_extracting_json(&self.buffer) {
                // found a valid json , need to update the buffer
                
                println!("remainder: {}", remainder);
                self.buffer = remainder;

                return Ok(Some(LogLine { content: json, source: format!("network {}", self.address), timestamp: chrono::Utc::now() }));
            }


            // just a check here if the buffer content doesn't have any { or  } we can have it as a plain text
            if !line.contains('{') && !line.contains('}') {
                self.buffer.clear();
                return Ok(Some(LogLine { content: line.trim().to_string(), source: format!("network {}", self.address), timestamp: chrono::Utc::now() }));
            }

            Ok(None)
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



// we have a plain text directly 
// initially accumate in the buffer