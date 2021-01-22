use crate::*;

pub struct Apply<I, SF> {
    pub iterator: I,
    pub stream_function: SF,
}

impl<C, E, Item, I: Iterator<Item=(C, Item)>, SF: StreamFunction<Input=Item, Clock=C, Error=E>> Iterator for Apply<I, SF> {
    type Item = SF::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.iterator.next();
        input.map(|(clock, input)| self.stream_function.step(input, clock).ok()).flatten()
    }
}
