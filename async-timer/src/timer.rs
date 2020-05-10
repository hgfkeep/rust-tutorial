use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future 和等待线程间共享的状态信息
struct SharedState {
    /// 睡眠时间是否已过
    completed: bool,

    /// `TimerFuture`所在任务的waker.
    /// 当`completed = true` 时，线程使用该信息，通知唤醒 `TimerFuture`
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置waker，当计时器完成时，线程可以唤醒当前task, 确保future 被再次poll，观察到`completed = true`.
            //
            // 仅尝试执行一次设置而不是反复设置waker看起来很诱人，但是，`TimerFuture`可以在执行程序上的任务之间移动，这可能会导致过时的唤醒程序指向错误的任务，从而阻止`TimerFuture`正确唤醒。
            //
            // N.B. 可以使用`Waker :: will_wake`函数来检查它，但是为了使事情简单，我们在这里省略了。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// 创建一个新的`TimerFuture` 将在提供timeout之后完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 表示计时器已经完成并唤醒最后一个拥有被poll过的future的任务，如果它存在的话
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}