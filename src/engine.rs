use std::{error::Error, sync::Arc};

use tokio::sync::mpsc;

use crate::{ingest::LogSource, parser::{LogParser, ParsedLog}};

pub struct Engine {
    sources : Vec<Box<dyn LogSource>>,
    parser_registry : Arc<Box<dyn LogParser>>
}

impl Engine {
    pub fn new(parser_registry : Box<dyn LogParser>) -> Self {
        Self { sources: Vec::new(), parser_registry: Arc::new(parser_registry) }
    }

    pub fn add_source(&mut self, source : Box<dyn LogSource>) {
        self.sources.push(source);
    }

    pub async fn run(&mut self) -> Result<mpsc::Receiver<ParsedLog>, Box<dyn Error + Send + Sync>> {

        let (tx, rx) = mpsc::channel(100);

        for source in &mut self.sources {
            source.init().await?;
        }
        

        for mut source in std::mem::take(&mut self.sources) {
            let tx_clone = tx.clone(); 
            let parser_clone = self.parser_registry.clone();

            tokio::spawn(async move {
                while let Ok(Some(log_line)) = source.read_line().await {
                    match parser_clone.parse(log_line).await {
                        Ok(parsed_log) => {
                            if tx_clone.send(parsed_log).await.is_err() {
                                break;
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing log: {}", e);
                        }
                    }
                }
                let _ = source.close().await;
            });

        }

        drop(tx);

        Ok(rx)
    }
}