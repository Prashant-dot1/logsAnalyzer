use std::{error::Error, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let listner = TcpListener::bind("127.0.0.1:8888").await?;

    while let Ok((mut socket , addr)) = listner.accept().await {

        print!("new client connected: {}", addr);

        let _handler = tokio::spawn(async move {
            let mut counter = 0;

            loop {
                // Alternate between single-line and multi-line JSON
                let logging =
                    format!(
                        "{{\"message\": \"{} LOG-first-{} testing\", \"level\": \"info\", \"service\": \"network-example-server\"}}\n",
                        chrono::Utc::now(),
                        counter
                    );

                println!("Sending: {}", logging);

                if let Err(e) = socket.write_all(logging.as_bytes()).await {
                    println!("Error writing to client: {}", e);
                    break;
                }

                counter += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;

            }
        });
    }

    Ok(())

}