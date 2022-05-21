use tauri;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[tauri::command]
pub async fn start_server() {
    println!("image server start server");
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    loop {
        let (client, _) = listener.accept().await.unwrap();
        let mut frame_reader = FramedRead::with_capacity(client, LengthDelimitedCodec::new(), 1024 * 1024 * 8);
        while let Some(frame) = frame_reader.next().await {
            match frame {
                Ok(data) => println!("received: {:?}", data[0]),
                Err(err) => eprintln!("error: {:?}", err),
            }
        }
    }
}