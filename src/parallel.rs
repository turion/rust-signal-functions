use crate::*;

pub struct Parallel<Left, Right> {
    pub left: Left,
    pub right: Right,
}

impl<C: Copy, Left: StreamFunction<Clock=C>, Right: StreamFunction<Clock=C>> StreamFunction for Parallel<Left, Right> {
    type Input = (Left::Input, Right::Input);
    type Output = (Left::Output, Right::Output);
    type Clock = C;

    fn step(&mut self, input: Self::Input, clock: Self::Clock) -> Self::Output {
        let left = self.left.step(input.0, clock);
        let right = self.right.step(input.1, clock);
        (left, right)
    }
}
