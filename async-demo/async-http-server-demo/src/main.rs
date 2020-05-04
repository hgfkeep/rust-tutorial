use env_logger;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, Uri,
};
use log::{error, info, debug};
use std::net::SocketAddr;

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    debug!("Request: {:?}", req);
    let url_str = "http://www.rust-lang.org";
    let url = url_str.parse::<Uri>().expect("failed parse URL");
    let res = Client::new().get(url).await?;
    info!("request finished!");
    Ok(res) 
}

async fn run_server(addr: &SocketAddr) {
    info!("Listening on http://{}", addr);

    let server = Server::bind(addr).serve(make_service_fn(|_| {
        // 使用 `async serve_req` 函数来处理请求.
        async {
            {
                Ok::<_, hyper::Error>(service_fn(serve_req))
            }
        }
    }));

    if let Err(e) = server.await {
        error!("server error: {:?}", e);
    }
}

#[tokio::main]
async fn main() {
    // 设置日志
    env_logger::init();

    // 设置socker 的地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // 调用我们的 `async run_server` 函数, 它将返回一个 `future`.
    // 和 `async fn` 一样, 要让 `run_server` 执行任何操作,
    // 都需要运行返回的 `future`. 并且我们需要将返回的
    // `future` 从 `0.3` 转换为 `0.1`.
    run_server(&addr).await;
}
