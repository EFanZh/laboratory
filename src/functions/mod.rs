pub mod left;
pub mod right;

pub trait FnOnce1<T> {
    type Output;

    fn call_once(self, arg: T) -> Self::Output;
}

pub trait FnMut1<T>: FnOnce1<T> {
    fn call_mut(&mut self, arg: T) -> Self::Output;
}

pub trait Fn1<T>: FnMut1<T> {
    fn call(&self, arg: T) -> Self::Output;
}
