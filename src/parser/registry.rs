use std::error::Error;

use crate::ingest::LogLine;

use super::{LogParser, ParsedLog};

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

    fn select_parser(&self, log_line : &LogLine) -> Option<Box<dyn LogParser>> {

        if ParserRegistry::try_parse_json(&log_line.content) {
            todo!()
        }
        else {
            todo!()
        }
    }
}


#[async_trait::async_trait]
impl LogParser for ParserRegistry {
    async fn parse(&self, log_line : LogLine) -> Result<ParsedLog, Box<dyn Error>> {
        let parser = self.select_parser(&log_line).ok_or("no parser found")?;

        parser.parse(log_line).await
    }
}