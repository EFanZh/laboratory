use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Defer {
    pub count: usize,
}

impl Defer {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Future for Defer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready(())
        } else {
            self.count -= 1;

            cx.waker().wake_by_ref();

            Poll::Pending
        }
    }
}
