# rust异步变成

## Future trait

标准库中关于异步编程的包位于[`std::future`](https://doc.rust-lang.org/std/future/index.html) 和 [`std::task`](https://doc.rust-lang.org/std/task/index.html)中。

其中`std::future::Future` trait 表示异步计算的结果（可能尚未计算出结果）,它的声明如下：

```rust
#[must_use = "futures do nothing unless you `.await` or poll them"]#[lang = "future_trait"]
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

* `Output`：完成计算时产生的值类型。
* `poll()` : 试图将future计算为最终结果。如果值没有Ready，不会阻塞。
  * Context: 完整类型为`std::task::Context`，是异步任务的上下文。当前，上下文仅用于提供对`&Waker`的访问，该访问可用于**唤醒**当前任务。
  * Poll: 枚举类型，表示future的计算状态和结果。

### helloworld future

```rust
struct HelloWorld;

impl Future for HelloWorld {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Poll::from("hello world".to_string())
    }
}
```


### 标准库执行furure

### tokio runtime 执行future

