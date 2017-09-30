use std::cell::Cell;
use std::mem;
use std::rc::Rc;

use futures::{Async, Future, IntoFuture, Poll};

pub enum FutureCell<F>
where
    F: Future,
{
    Unavailable(F),
    Available(Rc<Cell<F::Item>>),
    Processing,
}

impl<I> From<I> for FutureCell<I::Future>
where
    I: IntoFuture,
{
    fn from(future_to_be: I) -> FutureCell<I::Future> {
        FutureCell::Unavailable(future_to_be.into_future())
    }
}

impl<F> FutureCell<F>
where
    F: Future,
{
    pub fn is_available(&self) -> bool {
        match *self {
            FutureCell::Unavailable(_) => false,
            FutureCell::Available(_) => true,
            FutureCell::Processing => {
                unreachable!("FutureCell must not stay in Processing state");
            }
        }
    }
}

impl<F> Future for FutureCell<F>
where
    F: Future,
{
    type Item = Rc<Cell<F::Item>>;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let this = mem::replace(self, FutureCell::Processing);

        let (poll_result, this) = match this {
            FutureCell::Unavailable(mut future) => {
                let poll_result = future.poll();

                match poll_result {
                    Ok(Async::Ready(item)) => {
                        let item_ref = Rc::new(Cell::new(item));
                        let poll_result = Ok(Async::Ready(item_ref.clone()));
                        let this = FutureCell::Available(item_ref);

                        (poll_result, this)
                    }
                    Ok(Async::NotReady) => {
                        let poll_result = Ok(Async::NotReady);
                        let this = FutureCell::Unavailable(future);

                        (poll_result, this)
                    }
                    Err(error) => {
                        let poll_result = Err(error);
                        let this = FutureCell::Unavailable(future);

                        (poll_result, this)
                    }
                }
            }
            FutureCell::Available(item_ref) => {
                let poll_result = Ok(Async::Ready(item_ref.clone()));
                let this = FutureCell::Available(item_ref);

                (poll_result, this)
            }
            FutureCell::Processing => {
                unreachable!("FutureCell must not stay in Processing state");
            }
        };

        mem::replace(self, this);

        poll_result
    }
}
