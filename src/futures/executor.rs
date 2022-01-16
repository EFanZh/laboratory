use crossbeam::queue::SegQueue;
use futures::future::BoxFuture;
use futures::FutureExt;
use parking_lot::Mutex;
use std::future::Future;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Wake};
use std::thread::{self, Thread};

struct ThreadWaker {
    thread: Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

struct Inner {
    ready: SegQueue<Arc<Task>>,
    pending_threads: SegQueue<Thread>,
}

impl Inner {
    fn schedule_task(&self, task: Arc<Task>) {
        self.ready.push(task);

        if let Some(thread) = self.pending_threads.pop() {
            thread.unpark();
        }
    }

    fn spawn(self: &Arc<Self>, future: BoxFuture<'static, ()>) {
        self.schedule_task(Arc::new(Task {
            inner: Arc::downgrade(self),
            future: Mutex::new(future),
        }));
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        let thread = thread::current();

        futures::pin_mut!(future);

        let waker = Arc::new(ThreadWaker {
            thread: thread.clone(),
        })
        .into();

        let mut cx = Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    while let Some(task) = self.ready.pop() {
                        let waker = Arc::clone(&task).into();

                        match task
                            .future
                            .lock()
                            .poll_unpin(&mut Context::from_waker(&waker))
                        {
                            Poll::Ready(()) => {
                                // The task is done.
                            }
                            Poll::Pending => {
                                // The waker should be save by the future, which means the future itself is saved.
                            }
                        }
                    }

                    self.pending_threads.push(thread.clone());

                    thread::park();
                }
            }
        }
    }
}

struct Task {
    inner: Weak<Inner>,
    future: Mutex<BoxFuture<'static, ()>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        if let Some(inner) = self.inner.upgrade() {
            inner.schedule_task(self);
        }
    }
}

pub struct Executor {
    inner: Arc<Inner>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                ready: SegQueue::new(),
                pending_threads: SegQueue::new(),
            }),
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.inner.spawn(future.boxed());
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.inner.block_on(future)
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Executor;
    use crate::futures::defer::Defer;
    use crate::futures::sync::oneshot_event::OneshotEvent;

    #[test]
    fn test_block_on() {
        let executor = Executor::new();

        assert_eq!(
            executor.block_on(async {
                Defer::new(2).await;

                3
            }),
            3
        );
    }

    #[test]
    fn test_nested_block_on() {
        let executor = Executor::new();

        assert_eq!(
            executor.block_on(async {
                Defer::new(2).await;

                executor.block_on(async {
                    Defer::new(3).await;

                    5
                })
            }),
            5
        );
    }

    #[test]
    fn test_spawn() {
        let executor = Executor::new();

        assert_eq!(
            executor.block_on(async {
                let event = OneshotEvent::new();
                let wait = event.wait();

                executor.spawn(async {
                    Defer::new(2).await;

                    drop(event);
                });

                wait.await;

                3
            }),
            3
        );
    }
}
