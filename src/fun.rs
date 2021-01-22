use std::marker::PhantomData;

use crate::*;

pub struct Fun<T, U, C, E>{
    fun: Box<dyn Fn(T) -> U>,
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
    phantom_c: PhantomData<C>,
    phantom_e: PhantomData<E>,
}


impl<T, U, C, E> Fun<T, U, C, E> {
    pub fn new<F: 'static + Fn(T) -> U>(f: F) -> Self {
        Fun {
            fun: Box::new(f),
            phantom_t: PhantomData,
            phantom_u: PhantomData,
            phantom_c: PhantomData,
            phantom_e: PhantomData,
        }
    }
}

impl<T, U, C, E> StreamFunction for Fun<T, U, C, E>
{
    type Input = T;
    type Output = U;
    type Clock = C;
    type Error = E;

    fn step(&mut self, input: Self::Input, _: Self::Clock) -> Result<Self::Output, Self::Error> {
        Ok((self.fun)(input))
    }
}
