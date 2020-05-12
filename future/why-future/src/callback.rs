use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// 基于 **回调** 的 多任务处理方法
/// 运行时结构体
pub struct Runtime {
    // 保存所有的回调函数
    callbacks: RefCell<HashMap<usize, Box<dyn FnOnce() -> ()>>>,

    // 准备下次 调度的 回调函数ID，
    next_id: RefCell<usize>,

    // 回调信号 发送者
    event_sender: Sender<usize>,

    // 回调信号 接受者
    event_reciever: Receiver<usize>,
}

/// 运行时的实现
impl Runtime {
    fn new() -> Self {
        let (event_sender, event_reciever) = channel();
        Runtime {
            callbacks: RefCell::new(HashMap::new()),
            next_id: RefCell::new(1),
            event_sender,
            event_reciever,
        }
    }

    fn run(&self, program: fn()) {
        program();
        for event_id in &self.event_reciever {
            let callback = self.callbacks.borrow_mut().remove(&event_id).unwrap();
            callback();

            // TODO: 此处可能会发生event_id存在但是无callback函数？他们不是成对出现的吗？
            if self.callbacks.borrow().is_empty() {
                break;
            }
        }
    }
}

thread_local!{
    pub static RT: Runtime = Runtime::new();
}

// 支持callback的任务
pub fn set_timeout(ms: u64, callback: impl FnOnce() + 'static) {
    RT.with(|rt: &Runtime| {
        // 获取当前 callback id
        let id = *rt.next_id.borrow(); 
        // 设置下个callback id， 通过指针修改
        *rt.next_id.borrow_mut() += 1;

        // 注册callback到callback map 中，id <-> callback
        rt.callbacks.borrow_mut().insert(id, Box::new(callback));
        let event_sender = rt.event_sender.clone();

        // 提交 callback 到 channel中
        thread::spawn(move || {
            // 当前任务
            thread::sleep(std::time::Duration::from_millis(ms));
            println!(" 任务完成后，发送callback可以调用了");
            event_sender.send(id).unwrap();
        });
    });
}

