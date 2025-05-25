use super::map::Map;
use super::select::Select;
use crate::functions::left::Left;
use crate::functions::right::Right;
use futures::future::{Either, FusedFuture};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type LeftFuture<A, B> = Map<A, Left<B>>;
type RightFuture<A, B> = Map<A, Right<B>>;

pin_project_lite::pin_project! {
    pub struct SelectEither<A, B>
    where
        A: Future,
        B: Future,
    {
        #[pin]
        inner: Select<LeftFuture<A, B::Output>, RightFuture<B, A::Output>>,
    }
}

impl<A, B> SelectEither<A, B>
where
    A: Future,
    B: Future,
{
    pub fn new(future_1: A, future_2: B) -> Self {
        Self {
            inner: Select::new(Map::new(future_1, Left::new()), Map::new(future_2, Right::new())),
        }
    }
}

impl<A, B> Future for SelectEither<A, B>
where
    A: Future,
    B: Future,
{
    type Output = Either<A::Output, B::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}

impl<A, B> FusedFuture for SelectEither<A, B>
where
    A: FusedFuture,
    B: FusedFuture,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
