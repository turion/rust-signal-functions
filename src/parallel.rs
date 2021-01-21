use crate::*;

pub struct Parallel<Left, Right> {
    pub left: Left,
    pub right: Right,
}

impl<Left: StreamFunction, Right: StreamFunction> StreamFunction for Parallel<Left, Right> {
    type Input = (Left::Input, Right::Input);
    type Output = (Left::Output, Right::Output);

    fn step(&mut self, input: Self::Input) -> Self::Output {
        (self.left.step(input.0), self.right.step(input.1))
    }
}
