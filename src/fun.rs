use std::marker::PhantomData;

use crate::*;

pub struct Fun<T, U, C>{
    fun: Box<dyn Fn(T) -> U>,
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
    phantom_c: PhantomData<C>,
}


impl<T, U, C> Fun<T, U, C> {
    pub fn new<F: 'static + Fn(T) -> U>(f: F) -> Self {
        Fun {
            fun: Box::new(f),
            phantom_t: PhantomData,
            phantom_u: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<T, U, C> StreamFunction for Fun<T, U, C>
{
    type Input = T;
    type Output = U;
    type Clock = C;

    fn step(&mut self, input: Self::Input, _: Self::Clock) -> Self::Output {
        (self.fun)(input)
    }
}
