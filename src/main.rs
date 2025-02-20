use ingest::{file_source::FileLogSource, network_source::NetworkLogSource};
use parser::{json::JsonParser, plain_text::PlainTextParser, registry::ParserRegistry};
use engine::Engine;

pub mod ingest;
pub mod parser;
pub mod engine;
pub mod analytics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // we register the 2 different types of parsers we have
    let mut registry = ParserRegistry::new();
    registry.register(PlainTextParser::new());
    registry.register(JsonParser::new());

    // Create engine
    let mut engine = Engine::new(Box::new(registry));

    // Add sources , need to figure out a way by which this add_source can be done dynamically
    // engine.add_source(Box::new(FileLogSource::new("./example.log")));
    engine.add_source(Box::new(NetworkLogSource::new("127.0.0.1:8888".to_string())));

    // Run engine and get receiver
    let mut rx = engine.run().await?;

    // Process parsed logs
    while let Some(parsed_log) = rx.recv().await {
        println!("Parsed log: {:?}", parsed_log);
    }

    Ok(())
}
