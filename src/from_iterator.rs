use crate::*;

use std::iter::*;

pub struct FromIter<I> {
    i: I,
}

impl<I: Iterator> StreamFunction for FromIter<I> {
    type Input = ();
    type Output = Result<I::Item, ()>;

    fn step(&mut self, _: Self::Input) -> Self::Output {
        self.i
            .next()
            .ok_or(())
    }
}
