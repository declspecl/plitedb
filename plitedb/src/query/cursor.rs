use std::{fmt, iter::Peekable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize
}

impl fmt::Display for Location {
    fn fmt(
        &self,
        f: &mut fmt::Formatter
    ) -> fmt::Result {
        return write!(f, "line {}, column {}", self.line, self.column);
    }
}

pub trait CursorTrackable {
    fn next_location(
        &self,
        location: Location
    ) -> Location;
}

pub struct PeekingCursor<I>
where
    I: Iterator,
    I::Item: CursorTrackable
{
    items: Peekable<I>,
    location: Location
}

impl<I> Iterator for PeekingCursor<I>
where
    I: Iterator,
    I::Item: CursorTrackable
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.items.next();

        if let Some(ref next) = next {
            self.location = next.next_location(self.location);
        }

        return next;
    }
}

impl<I> PeekingCursor<I>
where
    I: Iterator,
    I::Item: CursorTrackable
{
    pub fn new(input: I) -> Self {
        return PeekingCursor {
            items: input.peekable(),
            location: Location { line: 1, column: 0 }
        };
    }

    pub fn loc(&self) -> Location {
        return self.location;
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
                matches.push(self.next().unwrap());
            }
            else {
                break;
            }
        }

        return matches;
    }
}
