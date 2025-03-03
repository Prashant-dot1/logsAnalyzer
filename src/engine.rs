use std::{error::Error, sync::Arc};


use futures::lock::Mutex;
use tokio::sync::mpsc;

use crate::{analytics::LogAnalytics, ingest::LogSource, parser::{LogParser, ParsedLog}};

pub struct Engine {
    sources : Vec<Box<dyn LogSource>>,
    parser_registry : Arc<Box<dyn LogParser>>,
    analytics: Arc<Mutex<LogAnalytics>>
}

impl Engine {
    pub fn new(parser_registry : Box<dyn LogParser>) -> Self {
        Self { sources: Vec::new(), 
            parser_registry: Arc::new(parser_registry) , 
            analytics : Arc::new(Mutex::new(LogAnalytics::new(100)))}
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
            let analytics_clone = self.analytics.clone();

            tokio::spawn(async move {
                let mut batch = Vec::with_capacity(100);

                while let Ok(Some(log_line)) = source.read_line().await {

                    batch.push(log_line);

                    if batch.len() >= 100 {

                        let futures = batch.iter()
                        .map(|log_line| {
                            let parser = parser_clone.clone();
                            async move {
                                parser.parse(log_line.clone()).await
                            }
                        }).collect::<Vec<_>>();

                        let results = futures::future::join_all(futures).await;


                        for res in results {
                            if let Ok(parsed_log) = res {
                                let analytics = analytics_clone.clone();
                                let log_clone = parsed_log.clone();
                                
                                tokio::spawn(async move {
                                    let analytics_lock = analytics.lock();

                                    analytics_lock.await.process_log(log_clone);
                                });

                                if tx_clone.send(parsed_log).await.is_err() {
                                    break;
                                }
                            }   
                        }

                        batch.clear();
                    }
                }
                let _ = source.close().await;
            });

        }

        drop(tx);

        Ok(rx)
    }
}