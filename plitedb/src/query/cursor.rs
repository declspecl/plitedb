use std::iter::Peekable;

pub struct PeekingCursor<I, T>
where
    I: Iterator<Item = T>
{
    items: Peekable<I>
}

impl<I, T> Iterator for PeekingCursor<I, T>
where
    I: Iterator<Item = T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        return self.items.next();
    }
}

impl<I, T> PeekingCursor<I, T>
where
    I: Iterator<Item = T>
{
    pub fn new(input: I) -> Self {
        return PeekingCursor { items: input.peekable() };
    }

    pub fn peek(&mut self) -> Option<&T> {
        return self.items.peek();
    }

    pub fn peek_and_take_while<P>(
        &mut self,
        predicate: P
    ) -> Vec<T>
    where
        P: Fn(&T) -> bool
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
