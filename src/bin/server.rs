use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // Buat receiver baru khusus untuk koneksi klien ini
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            maybe_msg = ws_stream.next() => {
                match maybe_msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            let text = msg.as_text().unwrap().to_string();

                            println!("[LOG] Pesan dari {}: {}", addr, text);
                            
                            let formatted_msg = format!("{}: {}", addr, text);
                            
                            let _ = bcast_tx.send(formatted_msg);
                        }
                    }
                    _ => break, 
                }
            }

            maybe_bcast = bcast_rx.recv() => {
                match maybe_bcast {
                    Ok(msg_str) => {
                        if let Some((sender_addr_str, actual_msg)) = msg_str.split_once(": ") {
                            if sender_addr_str == addr.to_string() {
                                continue;
                            }
                            
                            ws_stream.send(Message::text(format!("{}: {}", sender_addr_str, actual_msg))).await?;
                        }
                    }
                    Err(_e) => break, 
                }
            }
        }
    }

    println!("Connection closed for {addr:?}");
    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}