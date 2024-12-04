use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    // 运行 'TimerFuture' 的任务的唤醒器。
    // 线程可以在设置 'completed = true' 后使用它来告诉
    // 'TimerFuture' 的任务唤醒时，看到 'completed = true'，以及
    // 前进。
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
            // 设置 waker，以便线程可以唤醒当前任务
            // 当计时器完成时，确保轮询 future
            // 再次看到 'completed = true'。
            // //
            // 这样做一次而不是重复克隆是很诱人的
            // 唤醒器。但是，'TimerFuture' 可以在
            // 执行程序上的任务，这可能会导致过时的唤醒程序指向
            // 到错误的任务，阻止 'TimerFuture' 唤醒
            // 正确。
            // //
            // 注意，可以使用 'Waker：：will_wake' 来检查这一点
            // 函数，但为了简单起见，我们在这里省略了它。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
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
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}