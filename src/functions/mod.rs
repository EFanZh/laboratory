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

impl<T, R> FnOnce1<T> for T
where
    T: FnOnce(T) -> R,
{
    type Output = R;

    fn call_once(self, arg: T) -> Self::Output {
        self(arg)
    }
}

impl<T, R> FnMut1<T> for T
where
    T: FnMut(T) -> R,
{
    fn call_mut(&mut self, arg: T) -> Self::Output {
        self(arg)
    }
}

impl<T, R> Fn1<T> for T
where
    T: Fn(T) -> R,
{
    fn call(&self, arg: T) -> Self::Output {
        self(arg)
    }
}
