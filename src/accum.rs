use crate::*;

use std::marker::PhantomData;

pub struct Accum<Acc, Input, F> {
    acc: Acc,
    f: F,
    phantom: PhantomData<Input>,
}

impl<Acc, Input, F: FnMut(&mut Acc, Input)> Accum<Acc, Input, F> {
    pub fn new(acc: Acc, f: F) -> Self {
        Accum {
            acc,
            f,
            phantom: PhantomData,
        }
    }
}

impl<Acc: Clone, Input, F: FnMut(&mut Acc, Input)> StreamFunction for Accum<Acc, Input, F> {
    type Input = Input;
    type Output = Acc;

    fn step(&mut self, input: Self::Input) -> Self::Output {
        let acc = self.acc.clone();
        (self.f)(&mut self.acc, input);
        acc
    }
}
