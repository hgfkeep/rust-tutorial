use std::cell::RefCell;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

// 模拟std::task
pub mod task {
    use crate::NOTIFY;

    pub struct Context<'a> {
        waker: &'a Waker,
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            Context { waker }
        }

        pub fn waker(&self) -> &'a Waker {
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            NOTIFY.with(|f| *f.borrow_mut() = true)
        }
    }

    pub enum Poll<T> {
        Ready(T),
        Pending,
    }
}

/// 模拟std::future::Future
pub mod future {
    use crate::task::*;

    pub trait Future {
        type Output;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;

        /// Future 支持 map 函数 将一个future map为另一个future，
        ///
        /// 好处：
        ///     1. 不需要为每个计算函数都实现future
        ///     2. 零开销的抽象
        ///
        /// ⚠️注意：** map 无法处理嵌套的future**
        fn map<U, F>(self, f: F) -> Map<Self, F>
        where
            // 给范型类型添加限制
            // FnOnce 保证 仅可以调用一次，因为编译器可以保证该函数仅被调用一次。它消耗并获取所使用的环境值的所有权。
            // Fn和FnMut不变地或可变地借用了对环境的引用。 FnMut 可变引用，可调用多次，可以改变环境中的值； Fn 共享引用， 可调用多次，不可转移不可改变值
            F: FnOnce(Self::Output) -> U,
            Self: Sized, // 保证调用map 的Map trait实现类型是Sized， 非Sized 类型是不能作为参数和返回值的，无法调用map函数。
        {
            Map {
                future: self,
                f: Some(f),
            }
        }

        /// then 方法 接受返回future 的闭包函数
        ///
        /// 类似Future<Output= Future<Output=SomeType>> 这种嵌套，先完成外层future的计算，返回内层future
        ///
        /// ⚠️注意： map 中闭包函数返回的是任意类型； 而then 中闭包函数返回的是 future
        ///
        /// 参数：
        ///    f: 内层future 逻辑。 一个闭包函数
        fn then<Fut, F>(self, f: F) -> Then<Self, F>
        where
            F: FnOnce(Self::Output) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            Then {
                future: self,
                f: Some(f),
            }
        }
    }

    /////////////////////////////////////////////////////////

    /// 声明Map 结构
    /// 包含了 future 和 future 计算到结果后 需要执行的闭包
    pub struct Map<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, U> Future for Map<Fut, F>
    where
        Fut: Future,
        F: FnOnce(Fut::Output) -> U,
    {
        type Output = U;

        fn poll(&mut self, ctx: &Context) -> Poll<U> {
            // 判断future 计算结果，计算成功后，通过 闭包函数 映射 计算结果
            match self.future.poll(ctx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap(); // 通过take 保证闭包函数 仅会执行一次， 特别是当出现这种情形：future 已经Poll::Ready,但是还是可能会被poll 一次。本实现中会panic，符合逾期，避免反复执行
                    Poll::Ready(f(val))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    /////////////////////////////////////////////////////////
    /// Then 结构体
    /// 关联Future 可能的计算结果和闭包函数，返回一个Future
    pub struct Then<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    /// Then 结构体实现Future trait
    ///
    /// NextFut 为 内层Future类型
    impl<Fut, NextFut, F> Future for Then<Fut, F>
    where
        Fut: Future,
        NextFut: Future,
        F: FnOnce(Fut::Output) -> NextFut,
    {
        type Output = NextFut::Output;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.poll(ctx) {
                // 外层future 计算到结果后，执行闭包函数，返回新的future，并poll 新的内层future
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    f(val).poll(ctx)
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    /////////////////////////////////////////////////////////

    /// 能立即返回结果的future
    pub struct Ready<T>(Option<T>);

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(&mut self, _: &Context) -> Poll<Self::Output> {
            // poll 时使用take 保证仅会执行一次poll
            Poll::Ready(self.0.take().unwrap())
        }
    }

    pub fn ready<T>(val: T) -> Ready<T> {
        Ready(Some(val))
    }

    /////////////////////////////////////////////////////////结果组合器

    /// Future 每次poll的时候，可能Ready 也可能是错误或者Pending，为了更好的处理错误类型，增加了TryFuture trait
    ///
    /// 为每个Output类型是Result的Future  都默认实现了 TryFuture trait。
    /// 这样每个Output类型为Result的Future 均可以访问正确或错误的状态，而不必在map 或者 then 组合器方法中显示的使用Ok Err匹配方法了。
    ///
    ///
    pub trait TryFuture {
        type Ok;
        type Error;

        /// TryFuture 中的方法是try_poll， 类似Future trait的poll方法
        fn try_poll(&mut self, ctx: &Context) -> Poll<Result<Self::Ok, Self::Error>>;

        /// 实现and_then combinator,
        ///
        /// 仅在上游future 执行成功的情况下，执行下游future，闭包函数接受 Ok参数
        fn and_then<Fut, F>(self, f: F) -> AndThen<Self, F>
        where
            Fut: Future,
            F: FnOnce(Self::Ok) -> Fut,
            Self: Sized,
        {
            AndThen {
                future: self,
                f: Some(f),
            }
        }

        /// 映射error 信息，当上游future 出现error时，执行闭包函数
        fn map_err<E, F>(self, f: F) -> MapErr<Self, F>
        where
            F: FnOnce(Self::Error) -> E,
            Self: Sized,
        {
            MapErr {
                future: self,
                f: Some(f),
            }
        }
    }

    /// 为所有Output类型是 Result 的 Future，默认实现 TryFuture trait。
    impl<F, T, E> TryFuture for F
    where
        F: Future<Output = Result<T, E>>,
    {
        type Ok = T;
        type Error = E;

        fn try_poll(&mut self, ctx: &Context) -> Poll<F::Output> {
            self.poll(ctx)
        }
    }

    ////////////////////////////////////////////////////////

    /// AndThen 支持 仅在上次future 成功的情况下执行 后面的future。
    pub struct AndThen<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    /// 为 AndThen 实现Future trait， AndThen Future的Output类型是Rersult 类型，默认实现了TryFuture trait，具有try_poll方法。
    ///
    /// AndThen的上游future，Fut 是 TryFuture， 即不确定future 是否能执行成功。
    ///          下游future，NextFut是TryFuture<Error = Fut::Error>，保持Error类型与上有future 一致。
    ///        闭包计算 接受上游Fut ，输出下游NextFut
    impl<Fut, NextFut, F> Future for AndThen<Fut, F>
    where
        Fut: TryFuture,
        NextFut: TryFuture<Error = Fut::Error>,
        F: FnOnce(Fut::Ok) -> NextFut,
    {
        type Output = Result<NextFut::Ok, NextFut::Error>;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(ctx) {
                Poll::Ready(Ok(val)) => {
                    let f = self.f.take().unwrap();
                    f(val).try_poll(ctx) // 一旦引入了TryFuture， 整个链可能都会出错，使用try_poll 而不是poll
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            }
        }
    }

    ////////////////////////////////////////////////////////
    
    pub struct MapErr<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, E> Future for MapErr<Fut, F>
    where
        Fut: TryFuture,
        F: FnOnce(Fut::Error) -> E,
    {
        type Output = Result<Fut::Ok, E>;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(ctx) {
                Poll::Ready(result) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(result.map_err(f))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

use crate::future::*;
use crate::task::*;

/// future执行器， 在futures-preview crate中使用的是 LocalPool 中的方法：阻塞当前线程直到future 运行完成。
pub fn block_on<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            if let Poll::Ready(val) = f.poll(&ctx) {
                return val;
            }
        }
    })
}
