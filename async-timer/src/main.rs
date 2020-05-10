mod timer;
use {
    futures::{
        future::{FutureExt, BoxFuture},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{Context, Poll},
        time::Duration,
    },
    // The timer we wrote in the previous section:
    timer::TimerFuture,
};

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    /// 生成一个任务并发送到 task channel
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取future，如果future 没有完成，poll future并尝试完成它(is still Some),
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 从任务本身创建 `LocalWaker` 对象
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` 是
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`的别名.
                // 通过调用它的`Pin::as_mut`方法获取 `Pin<&mut dyn Future + Send + 'static>`
                if let Poll::Pending = future.as_mut().poll(context) {
                    // 没有完成future，将future放到task，等待未来再次运行
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}