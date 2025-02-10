use std::{error::Error, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let listner = TcpListener::bind("127.0.0.1:8888").await?;

    while let Ok((mut socket , addr)) = listner.accept().await {

        print!("new client connected: {}", addr);

        tokio::spawn(async move {
            let mut counter = 0;

            loop {
                let logging = format!(
                    "<<<Content: {} LOG-{} testing >>>\n",
                    chrono::Utc::now(),
                    counter
                );

                println!("{}",logging);

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