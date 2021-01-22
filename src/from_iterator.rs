use crate::*;

use std::iter::*;

pub struct FromIter<I> {
    i: I,
}

impl<I: Iterator> StreamFunction for FromIter<I> {
    type Input = ();
    type Output = I::Item;
    type Clock = ();
    type Error = ();

    fn step(&mut self, _: Self::Input, _: Self::Clock) -> Result<Self::Output, Self::Error> {
        self.i
            .next()
            .ok_or(())
    }
}
