use crate::*;

pub struct Composition<First, Second> {
    pub first: First,
    pub second: Second,
}

impl<C: Copy, E, First: StreamFunction<Clock=C, Error=E>, Second: StreamFunction<Input=First::Output, Clock=C, Error=E>> StreamFunction for Composition<First, Second> {
    type Input = First::Input;
    type Output = Second::Output;
    type Clock = C;
    type Error = E;

    fn step(&mut self, input: Self::Input, clock: C) -> Result<Self::Output, Self::Error> {
        let intermediate = self.first.step(input, clock)?;
        self.second.step(intermediate, clock)
    }
}
