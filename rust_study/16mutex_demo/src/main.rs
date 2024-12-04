use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //原子引用计数 Arc<T>,原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //
    //     *num += 1;
    // });
    // handles.push(handle);
    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();
    //
    //     *num2 += 1;
    // });
    // handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
