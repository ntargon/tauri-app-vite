use std::str::from_utf8;

use serde::{Serialize, Deserialize};
use tauri::{self, Manager, async_runtime::spawn};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
    message: String
}

pub fn start_server(app_handle: tauri::AppHandle) {
    spawn( async move {
        println!("image server start server");
        let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
        println!("bind");
        loop {
            let (client, _) = listener.accept().await.unwrap();
            println!("accept");
            let mut frame_reader = FramedRead::with_capacity(client, LengthDelimitedCodec::new(), 1024 * 1024 * 8);
            while let Some(frame) = frame_reader.next().await {
                match frame {
                    Ok(data) => {
                        println!("received: {:?}", data[0]);
                        app_handle.emit_all("event-name", Payload {message: String::from_utf8((&data).to_vec()).unwrap()}).unwrap();
                    },
                    Err(err) => eprintln!("error: {:?}", err),
                }
            }
            println!("disconnect");
        }
    });
}