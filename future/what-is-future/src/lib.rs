use std::cell::RefCell;

// 局部线程的信号变量，当NOTIFY=true时，表示唤醒了，可以进行计算，否则等待唤醒。
thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

pub struct Context<'a> {
    waker: &'a Waker,
}

/// Future poll时的上下文信息
impl<'a> Context<'a> {

    /// 通过Waker 创建一个Context
    pub fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    /// 获取Context中的waker
    pub fn waker(&self) -> &'a Waker {
        &self.waker
    }
}

/// 唤醒器
pub struct Waker;

impl Waker {
    /// 唤醒future 的方法
    pub fn wake(&self) {
        // 任何时刻调用wake方法，均能唤醒future， 实际开发时不会这样
        NOTIFY.with(|f| *f.borrow_mut() = true) 
    }
}

/// 模拟std::task::Poll
/// 表示future 的当前计算状态
pub enum Poll<T> {
    Ready(T),
    Pending,
}

/// 模拟std::future::Future
///
/// trait 是rust中共享行为的方式，能定义类型必须实现的功能，也可以实现默认的行为
pub trait Future {
    type Output; // 关联类型，Future计算的结果类型

    /// future 核心的poll方法，
    ///
    /// params:
    ///     ctx: &Context 该对象具有对**Waker**的引用，该**Waker**用于通知运行时, future已准备好再次poll。
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
}

/// 自定义实现的Future的执行器
///
/// 接受一个Future 作为参数，并自动调度和运行Future
pub fn run<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        // 不断的循环执行
        if *n.borrow() {
            // 如果唤醒了
            *n.borrow_mut() = false; // 设置状态
            let ctx = Context::from_waker(&Waker); // 设置context
            if let Poll::Ready(val) = f.poll(&ctx) {
                // 调用poll 执行计算并尝试获取值
                return val; // 计算完成，返回值
            }
        }
    })
}
