use std::cell::RefCell;
use std::fmt::Debug;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

fn block_on<F>(mut f: F) -> F::Output
    where
        F: Future,
{
    NOTIFY.with(|n| loop {
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            if let Poll::Ready(val) = f.poll(&ctx) {
                return val;
            }
        }
    })
}

mod task {
    use crate::NOTIFY;

    pub struct Context<'a> {
        waker: &'a Waker,
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            Context { waker }
        }

        pub fn waker(&self) -> &'a Waker {
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            NOTIFY.with(|f| *f.borrow_mut() = true)
        }
    }

}
use crate::task::*;

mod future {
    use crate::task::*;

    pub enum Poll<T> {
        Ready(T),
        Pending,
    }

    pub trait Future {
        type Output;
        fn map<U, F>(self, f: F) -> Map<Self, F>
            where
                F: FnOnce(Self::Output) -> U,
                Self: Sized,
        {
            Map {
                future: self,
                f: Some(f),
            }
        }
        fn then<Fut, F>(self, f: F) -> Then<Self, F>
            where
                F: FnOnce(Self::Output) -> Fut,
                Fut: Future,
                Self: Sized,
        {
            Then {
                future: self,
                f: Some(f),
            }
        }

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;
    }
    pub struct Ready<T>(Option<T>);

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(&mut self, _: &Context) -> Poll<Self::Output> {
            Poll::Ready(self.0.take().unwrap())
        }
    }

    pub fn ready<T: std::fmt::Debug>(val: T) -> Ready<T> {
        Ready(Some(val))
    }
    pub struct Map<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, T> Future for Map<Fut, F>
        where
            Fut: Future,
            F: FnOnce(Fut::Output) -> T,
    {
        type Output = T;

        fn poll(&mut self, cx: &Context) -> Poll<T> {
            match self.future.poll(cx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(f(val))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
    pub struct Then<Fut, F> {
        future: Fut,
        f: Option<F>,
    }
    impl<Fut, NextFut, F> Future for Then<Fut, F>
        where
            Fut: Future,
            NextFut: Future,
            F: FnOnce(Fut::Output) -> NextFut,
    {
        type Output = NextFut::Output;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output> {
            match self.future.poll(cx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    f(val).poll(cx)
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

use crate::future::*;

fn main() {
    let my_future2 = future::ready(1)
        .map(|v| v+1)
        .then(|v| future::ready(v+1))
        .map(|v| v+4)
        .then(|v| future::ready(v+1));
    println!("Output: {}", block_on(my_future2));
}