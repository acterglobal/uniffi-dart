use uniffi;

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

/// Non-blocking timer future.
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        // Let's mimic an event coming from somewhere else, like the system.
        thread::spawn(move || {
            thread::sleep(duration);

            let mut shared_state: MutexGuard<_> = thread_shared_state.lock().unwrap();
            shared_state.completed = true;

            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        Self { shared_state }
    }
}

// /// Non-blocking timer future.
pub struct BrokenTimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for BrokenTimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl BrokenTimerFuture {
    pub fn new(duration: Duration, fail_after: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        // Let's mimic an event coming from somewhere else, like the system.
        thread::spawn(move || {
            thread::sleep(duration);

            let mut shared_state: MutexGuard<_> = thread_shared_state.lock().unwrap();
            shared_state.completed = true;

            if let Some(waker) = shared_state.waker.take() {
                // Do not consume `waker`.
                waker.wake_by_ref();

                // And this is the important part. We are going to call
                // `wake()` a second time. That's incorrect, but that's on
                // purpose, to see how foreign languages will react.
                if fail_after.is_zero() {
                    waker.wake();
                } else {
                    thread::spawn(move || {
                        thread::sleep(fail_after);
                        waker.wake();
                    });
                }
            }
        });

        Self { shared_state }
    }
}

#[uniffi::export]
pub fn greet(who: String) -> String {
    format!("Hello, {who}")
}

#[uniffi::export]
pub async fn always_ready() -> bool {
    true
}

#[uniffi::export]
pub async fn void_function() {}

#[uniffi::export]
pub async fn say() -> String {
    TimerFuture::new(Duration::from_secs(2)).await;

    "Hello, Future!".to_string()
}

#[uniffi::export]
pub async fn say_after(ms: u16, who: String) -> String {
    TimerFuture::new(Duration::from_millis(ms.into())).await;

    format!("Hello, {who}!")
}

#[uniffi::export]
pub async fn sleep(ms: u16) -> bool {
    TimerFuture::new(Duration::from_millis(ms.into())).await;

    true
}

// Our error.
// #[derive(uniffi::Error, Debug)]
// pub enum MyError {
//     Foo,
// }

// // An async function that can throw.
// // An async function that can throw.
// #[uniffi::export]
// pub async fn fallible_me(do_fail: bool) -> Result<u8, MyError> {
//     if do_fail {
//         Err(MyError::Foo)
//     } else {
//         Ok(42)
//     }
// }

#[uniffi::export(async_runtime = "tokio")]
pub async fn say_after_with_tokio(ms: u16, who: String) -> String {
    tokio::time::sleep(Duration::from_millis(ms.into())).await;
    format!("Hello, {who} (with Tokio)!")
}

#[derive(uniffi::Record, Clone)]
pub struct MyRecord {
    pub a: String,
    pub b: u32,
}

#[uniffi::export]
pub async fn new_my_record(a: String, b: u32) -> MyRecord {
    MyRecord { a, b }
}

#[uniffi::export]
pub async fn broken_sleep(ms: u16, fail_after: u16) {
    BrokenTimerFuture::new(
        Duration::from_millis(ms.into()),
        Duration::from_millis(fail_after.into()),
    )
    .await;
}

uniffi::include_scaffolding!("api");
