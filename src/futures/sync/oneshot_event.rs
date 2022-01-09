use futures::future::FusedFuture;
use parking_lot::Mutex;
use slotmap::{DefaultKey, HopSlotMap};
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Waker};

struct Inner {
    wakers: Mutex<HopSlotMap<DefaultKey, Waker>>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        for (_, waker) in mem::take(self.wakers.get_mut()) {
            waker.wake();
        }
    }
}

pub struct Wait {
    inner: Weak<Inner>,
    key: Option<DefaultKey>,
}

impl Drop for Wait {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() {
            if let Some(inner) = self.inner.upgrade() {
                inner.wakers.lock().remove(key);
            }
        }
    }
}

impl Clone for Wait {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: None,
        }
    }
}

impl Future for Wait {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if let Some(inner) = self.inner.upgrade() {
            let waker = cx.waker().clone(); // Clone waker first.
            let mut wakers = inner.wakers.lock(); // Acquire lock later.

            let old_waker = if let Some(key) = self.key {
                Some(mem::replace(&mut wakers[key], waker))
            } else {
                self.as_mut().key = Some(wakers.insert(waker));

                None
            };

            drop(wakers); // Drop locks first.
            drop(old_waker); // Drop waker later.

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl FusedFuture for Wait {
    fn is_terminated(&self) -> bool {
        self.inner.upgrade().is_none()
    }
}

pub struct OneshotEvent {
    inner: Arc<Inner>,
}

impl OneshotEvent {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                wakers: Mutex::new(HopSlotMap::new()),
            }),
        }
    }

    pub fn wait(&self) -> Wait {
        Wait {
            inner: Arc::downgrade(&self.inner),
            key: None,
        }
    }
}

impl Default for OneshotEvent {
    fn default() -> Self {
        Self::new()
    }
}
