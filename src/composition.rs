use crate::*;

pub struct Composition<First, Second> {
    pub first: First,
    pub second: Second,
}

impl<C: Copy, First: StreamFunction<Clock=C>, Second: StreamFunction<Input=First::Output, Clock=C>> StreamFunction for Composition<First, Second> {
    type Input = First::Input;
    type Output = Second::Output;
    type Clock = C;

    fn step(&mut self, input: Self::Input, clock: C) -> Self::Output {
        let intermediate = self.first.step(input, clock);
        self.second.step(intermediate, clock)
    }
}
