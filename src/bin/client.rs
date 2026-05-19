use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    println!("Connected to the chat server! Type your message and press Enter:");

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            maybe_line = stdin.next_line() => {
                match maybe_line {
                    Ok(Some(line)) => {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            ws_stream.send(Message::text(trimmed.to_string())).await?;
                        }
                    }
                    _ => break, 
                }
            }

            maybe_msg = ws_stream.next() => {
                match maybe_msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            println!("{}", msg.as_text().unwrap());
                        }
                    }
                    _ => {
                        println!("Disconnected from server.");
                        break; 
                    }
                }
            }
        }
    }

    Ok(())
}