use crate::*;

use std::marker::PhantomData;

pub struct Accum<Acc, Input, C, F> {
    acc: Acc,
    f: F,
    phantom: PhantomData<Input>,
    phantom_c: PhantomData<C>,
}

impl<Acc, Input, C, F: FnMut(&mut Acc, Input)> Accum<Acc, Input, C, F> {
    pub fn new(acc: Acc, f: F) -> Self {
        Accum {
            acc,
            f,
            phantom: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<Acc: Clone, Input, C, F: FnMut(&mut Acc, Input)> StreamFunction for Accum<Acc, Input, C, F> {
    type Input = Input;
    type Output = Acc;
    type Clock = C;

    fn step(&mut self, input: Self::Input, _: Self::Clock) -> Self::Output {
        let acc = self.acc.clone();
        (self.f)(&mut self.acc, input);
        acc
    }
}
