use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut socket) => {
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("wrote {:?} bytes", amt);
                            }
                            Err(e) => {
                                eprintln!("IO error: {:?}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    println!("accept error! {:?}", e);
                }
            }
        }
    };
    println!("server running on {}", addr);
    server.await;
}
