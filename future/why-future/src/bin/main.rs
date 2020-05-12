use std::thread;
use std::time::Duration;

use why_future::callback::{Runtime, set_timeout};

fn threads_do_multi_tasks() {
    println!("线程的方法执行多个任务");
    let t1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        println!("创建了一个任务，当前面的任务完成时，开始运行这个任务");
    });

    let t2 = thread::spawn(move || {
        // 任务1
        thread::sleep(Duration::from_millis(100));

        // 任务2
        println!("将2个任务链起来，任务1完成后完成任务2，任务2完成后完成任务3");
        let t3 = thread::spawn(move || {
            // 任务3
            thread::sleep(Duration::from_millis(50));

            //任务4
            println!("像这样，链起来的任务4");
        });
        t3.join().unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

/// set_timeout 本身就是一个任务，当任务完成后，调用回调函数
fn callback_do_multi_tasks() {
    println!(" 多个任务使用callback 方法完成");
    set_timeout(200, || {
        println!("创建一个callback");
    });

    set_timeout(100, || {
        println!("将多个任务链起来");
        set_timeout(50, || {
            println!("内部的链接任务");
        })
    });
}

fn main() {
    // threads_do_multi_tasks();
    callback_do_multi_tasks();
}
