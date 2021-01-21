use std::marker::PhantomData;

use crate::*;

pub struct Fun<T, U>{
    fun: Box<dyn Fn(T) -> U>,
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
}


impl<T, U> Fun<T, U> {
    pub fn new<F: 'static + Fn(T) -> U>(f: F) -> Self {
        Fun {
            fun: Box::new(f),
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        }
    }
}

impl<T, U> StreamFunction for Fun<T, U>
{
    type Input = T;
    type Output = U;

    fn step(&mut self, input: Self::Input) -> Self::Output {
        (self.fun)(input)
    }
}
