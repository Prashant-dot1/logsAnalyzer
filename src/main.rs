use ingest::{file_source::FileLogSource, LogSource};

pub mod ingest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file_source = FileLogSource::new("./example.log");
    file_source.init().await?;

    while let Some(log_line) = file_source.read_line().await? {
        println!("[{}] {}: {}",
            log_line.timestamp,
            log_line.source,
            log_line.content
        );
    }

    file_source.close().await?;
    
    Ok(())
}
