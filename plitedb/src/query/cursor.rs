use std::iter::Peekable;

pub struct PeekingCursor<I>
where
    I: Iterator
{
    items: Peekable<I>
}

impl<I> Iterator for PeekingCursor<I>
where
    I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        return self.items.next();
    }
}

impl<I> PeekingCursor<I>
where
    I: Iterator
{
    pub fn new(input: I) -> Self {
        return PeekingCursor { items: input.peekable() };
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        return self.items.peek();
    }

    pub fn peek_and_take_while<P>(
        &mut self,
        predicate: P
    ) -> Vec<I::Item>
    where
        P: Fn(&I::Item) -> bool
    {
        let mut matches = Vec::new();

        while let Some(next) = self.peek() {
            if predicate(next) {
                matches.push(self.items.next().unwrap());
            }
            else {
                break;
            }
        }

        return matches;
    }
}
