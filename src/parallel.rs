use crate::*;

pub struct Parallel<Left, Right> {
    pub left: Left,
    pub right: Right,
}

impl<C: Copy, E, Left: StreamFunction<Clock=C, Error=E>, Right: StreamFunction<Clock=C, Error=E>> StreamFunction for Parallel<Left, Right> {
    type Input = (Left::Input, Right::Input);
    type Output = (Left::Output, Right::Output);
    type Clock = C;
    type Error = E;

    fn step(&mut self, input: Self::Input, clock: Self::Clock) -> Result<Self::Output, Self::Error> {
        let left = self.left.step(input.0, clock)?;
        let right = self.right.step(input.1, clock)?;
        Ok((left, right))
    }
}
