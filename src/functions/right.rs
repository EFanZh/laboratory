use super::{Fn1, FnMut1, FnOnce1};
use futures::future::Either;
use std::marker::PhantomData;

pub struct Right<L> {
    _phantom: PhantomData<fn() -> L>,
}

impl<L> Right<L> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<L> Default for Right<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, L> FnOnce1<T> for Right<L> {
    type Output = Either<L, T>;

    fn call_once(mut self, arg: T) -> Self::Output {
        self.call_mut(arg)
    }
}

impl<T, R> FnMut1<T> for Right<R> {
    fn call_mut(&mut self, arg: T) -> Self::Output {
        self.call(arg)
    }
}

impl<T, R> Fn1<T> for Right<R> {
    fn call(&self, arg: T) -> Self::Output {
        Either::Right(arg)
    }
}
