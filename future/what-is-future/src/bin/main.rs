/// 此处涉及到了cargo 命名支持 `-` 但是rust 命名不支持, 会自动转为`_`
use what_is_future::{run, Context, Future, Poll};

////////////////////////////实现一个Future /////////////////////////////

#[derive(Default)] // 为MyFuturre 生成default 方法，默认的count=0
struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    // 实现Future trait
    type Output = u32; // 设置关联类型的类型，为Future计算的结果类型

    /// 实现Future trait的poll方法
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {

        // 如果是3则计算Ready，否则继续Pending，等待下次poll
        match self.count {
            3 => Poll::Ready(3), // future计算完成， 返回Ready
            _ => {
                self.count += 1; // 计算部分完成
                ctx.waker().wake(); // 唤醒准备下次poll
                Poll::Pending // 当前的Future的状态还是Pending中，没有计算完成
            }
        }
    }
}

///////////////////////  展示Future的能力：将多个Future 链起来 ////////////////////////////
struct AddOneFuture<T>(T); //AddOneFuture 包装了一个Future，这样可以观测到内部Future的状态情况

impl<T> Future for AddOneFuture<T>
// 实现Future trait
where
    T: Future,                                   // T 实现了Future trait
    T::Output: std::ops::Add<u32, Output = u32>, // T::Output 类型支持Add u32操作，且返回u32
{
    type Output = u32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        // 在前一个Future结果基础上执行+1操作
        match self.0.poll(ctx) {
            Poll::Ready(v) => Poll::Ready(v + 1), // 前一个操作完成了，则+1
            Poll::Pending => Poll::Pending,       // 前一个操作未完成，则Pending
        }
    }
}

fn main() {
    let f = MyFuture::default();
    // println!("Output: {}!", run(f));
    println!("Output: {}!", run(AddOneFuture(f)));
}
