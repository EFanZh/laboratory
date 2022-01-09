use futures::future::FusedFuture;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project_lite::pin_project! {
    pub struct Select<A, B>
    where
        A: Future,
        B: Future<Output = A::Output>,
    {
        #[pin]
        future_1: A,
        #[pin]
        future_2: B,
    }
}

impl<A, B> Select<A, B>
where
    A: Future,
    B: Future<Output = A::Output>,
{
    pub fn new(future_1: A, future_2: B) -> Self {
        Self { future_1, future_2 }
    }
}

impl<A, B> Future for Select<A, B>
where
    A: Future,
    B: Future<Output = A::Output>,
{
    type Output = A::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        match this.future_1.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result),
            Poll::Pending => this.future_2.poll(cx),
        }
    }
}

impl<A, B> FusedFuture for Select<A, B>
where
    A: FusedFuture,
    B: FusedFuture<Output = A::Output>,
{
    fn is_terminated(&self) -> bool {
        self.future_1.is_terminated() || self.future_2.is_terminated()
    }
}
