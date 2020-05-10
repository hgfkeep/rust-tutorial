extern crate futures;
extern crate tokio;

use futures::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::runtime::Runtime;

struct HelloWorld;

impl Future for HelloWorld {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Poll::from("hello world".to_string())
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.spawn(async{
        println!("{}", HelloWorld.await);
    });
}
