
use serde::{Serialize, Deserialize};
use tauri::{self, Manager, async_runtime::spawn};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::SinkExt;
use base64;

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
    message: String
}

pub fn start_server() {
    spawn( async move {
        println!("image server start server");
        let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
        let ws_listener = TcpListener::bind("127.0.0.1:54321").await.unwrap();
        println!("bind");

        let (stream, _) = ws_listener.accept().await.unwrap();

        let mut ws_stream = accept_async(stream).await.expect("msFailed to accept");
        println!("accept ws");

        loop {
            let (client, _) = listener.accept().await.unwrap();
            println!("accept");
            let mut frame_reader = FramedRead::with_capacity(client, LengthDelimitedCodec::new(), 1024 * 1024 * 8);
            while let Some(frame) = frame_reader.next().await {
                match frame {
                    Ok(data) => {
                        println!("received: {:?}", data[0]);
                        ws_stream.send(Message::Text(base64::encode(data))).await.unwrap();
                    },
                    Err(err) => eprintln!("error: {:?}", err),
                }
            }
            println!("disconnect");
        }
    });
}