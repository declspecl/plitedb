use std::{iter::Peekable, str::Chars};

pub struct PeekingCursor<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Iterator for PeekingCursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        return self.chars.next();
    }
}

impl<'a> PeekingCursor<'a> {
    pub fn new(input: &'a str) -> Self {
        return PeekingCursor {
            chars: input.chars().peekable()
        };
    }

    pub fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }

    pub fn peek_and_take_while<P>(
        &mut self,
        predicate: P
    ) -> String
    where
        P: Fn(&char) -> bool
    {
        let mut matches = String::new();

        while let Some(next) = self.peek() {
            if predicate(next) {
                matches.push(self.chars.next().unwrap());
            }
            else {
                break;
            }
        }

        return matches;
    }
}
