use futures::task::AtomicWaker;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll};

struct Inner {
    waker: AtomicWaker,
}

impl Drop for Inner {
    fn drop(&mut self) {
        self.waker.wake()
    }
}

pub struct Wait {
    inner: Weak<Inner>,
}

impl Drop for Wait {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.waker.take();
        }
    }
}

impl Future for Wait {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if let Some(inner) = self.inner.upgrade() {
            inner.waker.register(cx.waker());

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

#[derive(Clone)]
pub struct UniqueOneshotEvent {
    inner: Arc<Inner>,
}

impl UniqueOneshotEvent {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                waker: AtomicWaker::new(),
            }),
        }
    }

    pub fn wait(&self) -> Wait {
        Wait {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

impl Default for UniqueOneshotEvent {
    fn default() -> Self {
        Self::new()
    }
}
