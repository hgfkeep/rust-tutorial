use future_chains::block_on;
use future_chains::future;
use future_chains::future::Future;
use future_chains::future::TryFuture;

fn main() {
    let f = future::ready(1)
        // map函数需要import Future trait
        .map(|v| v + 1)
        // then 函数接受返回future的闭包函数
        .then(|v| future::ready(v * 10))
        .map(Err)
        .map_err(|e| format!("error: {:?}", e))
        // and_then 输入Ok的future， 返回 一个新的future，且新的Output 是Result类型
        .and_then(|_: ()| future::ready(Ok(0)))
        // then 输入
        .then(|res| {
            future::ready(match res {
                Ok(v) => Ok(v + 10),
                err => err,
            })
        });
    println!("Output: {:?}", block_on(f));

}
