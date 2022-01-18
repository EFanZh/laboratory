use crate::functions::FnMut1;
use futures::future::FusedFuture;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project_lite::pin_project! {
    pub struct Map<Fut, F>
    where
        Fut: Future,
        F: FnMut1<Fut::Output>
    {
        #[pin]
        inner: Fut,
        f: F
    }
}

impl<Fut, F> Map<Fut, F>
where
    Fut: Future,
    F: FnMut1<Fut::Output>,
{
    pub fn new(future: Fut, f: F) -> Self {
        Self { inner: future, f }
    }
}

impl<Fut, F> Future for Map<Fut, F>
where
    Fut: Future,
    F: FnMut1<Fut::Output>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        this.inner.poll(cx).map(|result| this.f.call_mut(result))
    }
}

impl<Fut, F> FusedFuture for Map<Fut, F>
where
    Fut: FusedFuture,
    F: FnMut1<Fut::Output>,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
