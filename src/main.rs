use ingest::{file_source::FileLogSource, network_source::NetworkLogSource, LogSource};
use parser::{json::JsonParser, plain_text::PlainTextParser, registry::ParserRegistry, LogParser};


pub mod ingest;
pub mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // we register the 2 different types of parsers we have
    let mut registry = ParserRegistry::new();
    registry.register(PlainTextParser::new());
    registry.register(JsonParser::new());

    let mut file_source = FileLogSource::new("./example.log");
    file_source.init().await?;

    while let Some(log_line) = file_source.read_line().await? {
        // println!("[{}] {}: {}",
        //     log_line.timestamp,
        //     log_line.source,
        let parsed_log = registry.parse(log_line).await?;
        println!("parsed log line: {:?}" , parsed_log);
    }

    file_source.close().await?;


    // in case of network
    let mut network_source = NetworkLogSource::new("127.0.0.1:8888".to_string());

    network_source.init().await?;

    for _ in 0..5 {
        if let Some(log_line) = network_source.read_line().await? {
            // println!("[{}] {}: {}",
            //     log_line.timestamp,
            //     log_line.source,
            //     log_line.content
            // );

            let parsed_log = registry.parse(log_line).await?;
            println!("parsed log line network: {:?}", parsed_log);
        }
    }

    network_source.close().await?;
    
    Ok(())
}
