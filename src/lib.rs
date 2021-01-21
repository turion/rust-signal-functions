pub mod accum;
pub mod apply;
pub mod composition;
pub mod fun;
pub mod from_iterator;
pub mod parallel;

use crate::apply::Apply;
use crate::fun::Fun;
use crate::composition::Composition;
use crate::parallel::Parallel;

#[allow(type_alias_bounds)]
pub type Map<T: StreamFunction, U> = Composition<T, Fun<T::Output, U>>;

pub type Iter<SF> = Apply<core::iter::Repeat<()>, SF>;

pub type FanOut<Input, S, SF> = Composition<Fun<Input, (Input, Input)>, Parallel<S, SF>>;

pub trait StreamFunction {
    type Input;
    type Output;

    fn step(&mut self, input: Self::Input) -> Self::Output;

    fn map<U, F>(self, f: F) -> Map<Self, U>
    where
        Self: Sized,
        F: 'static + Fn(Self::Output) -> U,
    {
        Composition {
            first: self,
            second: Fun::new(f),
        }
    }

    fn apply_to<I>(self, iterator: I) -> Apply<I, Self>
    where
        Self: Sized,
    {
        Apply {
            iterator,
            stream_function: self,
        }
    }

    fn iter(self) -> Iter<Self>
    where
        Self: Sized,
    {
        self.apply_to(core::iter::repeat(()))
    }

    fn and_then<SF: StreamFunction<Input=Self::Output>>(self, sf: SF) -> Composition<Self, SF>
    where
        Self: Sized,
    {
        Composition {
            first: self,
            second: sf,
        }
    }

    fn parallel<SF: StreamFunction>(self, sf: SF) -> Parallel<Self, SF>
    where
        Self: Sized,
    {
        Parallel {
            left: self,
            right: sf,
        }
    }

    fn fan_out<SF: StreamFunction<Input=Self::Input>>(self, sf: SF) -> FanOut<Self::Input, Self, SF>
    where
        Self: Sized,
        Self::Input: Clone,
    {
        Fun::new(|x: Self::Input| (x.clone(), x)).and_then(self.parallel(sf))
    }
}
