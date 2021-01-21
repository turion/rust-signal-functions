use crate::*;

pub struct Composition<First, Second> {
    pub first: First,
    pub second: Second,
}

impl<First: StreamFunction, Second: StreamFunction<Input=First::Output>> StreamFunction for Composition<First, Second> {
    type Input = First::Input;
    type Output = Second::Output;

    fn step(&mut self, input: Self::Input) -> Self::Output {
        let intermediate = self.first.step(input);
        self.second.step(intermediate)
    }
}
