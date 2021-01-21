use crate::*;

pub struct Apply<I, SF> {
    pub iterator: I,
    pub stream_function: SF,
}

impl<I: Iterator, SF: StreamFunction<Input=I::Item>> Iterator for Apply<I, SF> {
    type Item = SF::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.iterator.next();
        input.map(|input| self.stream_function.step(input))
    }
}
