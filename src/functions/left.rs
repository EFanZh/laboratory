use super::{Fn1, FnMut1, FnOnce1};
use futures::future::Either;
use std::marker::PhantomData;

pub struct Left<R> {
    _phantom: PhantomData<fn() -> R>,
}

impl<R> Left<R> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<R> Default for Left<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, R> FnOnce1<T> for Left<R> {
    type Output = Either<T, R>;

    fn call_once(mut self, arg: T) -> Self::Output {
        self.call_mut(arg)
    }
}

impl<T, R> FnMut1<T> for Left<R> {
    fn call_mut(&mut self, arg: T) -> Self::Output {
        self.call(arg)
    }
}

impl<T, R> Fn1<T> for Left<R> {
    fn call(&self, arg: T) -> Self::Output {
        Either::Left(arg)
    }
}
