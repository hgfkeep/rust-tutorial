use tokio::prelude::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("connect stream", );
    let res = stream.write(b"hello tokio\n").await;
    println!("write to stream. success={:?}", res.is_ok());
}
