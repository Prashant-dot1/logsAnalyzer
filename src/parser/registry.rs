use std::{any::Any, error::Error};

use crate::ingest::LogLine;

use super::{json::JsonParser, plain_text::PlainTextParser, LogParser, ParsedLog};

pub struct ParserRegistry {
    parsers: Vec<Box<dyn LogParser>>
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self { parsers: Vec::new() }
    }

    pub fn register(&mut self, parser : impl LogParser) {
        self.parsers.push(Box::new(parser));
    }


    // this is basically trying to parse and check the logLine content to a json value
    fn try_parse_json(content : &str) -> bool {
        serde_json::from_str::<serde_json::Value>(content).is_ok()
    }

    fn select_parser(&self, log_line : &LogLine) -> Option<&Box<dyn LogParser>> {

        if ParserRegistry::try_parse_json(&log_line.content) {
            println!("Parser selected : JsonParser");
            self.parsers.iter().find(|p| p.as_any().is::<JsonParser>())
        }
        else {
            println!("Parser selected : PlainTextParser");
            self.parsers.iter().find(|p| p.as_any().is::<PlainTextParser>())
        }
    }
}


#[async_trait::async_trait]
impl LogParser for ParserRegistry {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error + Send + Sync>> {
        let parser = self.select_parser(&log_line).ok_or("no parser found")?;

        parser.parse(log_line).await
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::{ingest::{file_source::FileLogSource, LogSource}, parser::registry};

    use super::*;


    #[tokio::test]
    async fn test_registry() {

        let mut registry = ParserRegistry::new();
        registry.register(JsonParser::new());
        registry.register(PlainTextParser::new());

        
        let mut file = FileLogSource::new("./example.log");
        file.init().await.unwrap();

        while let Some(log_line) = file.read_line().await.unwrap() {
            let res = registry.parse(log_line).await.unwrap();

            println!("The parsed log is: {:?}", res);
        }
    }
    
}