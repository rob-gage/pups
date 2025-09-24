// Copyright Rob Gage 2025

use crate::{
    CollectionParser,
    Input,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};


/// A combinator that applies a parser repeatedly
pub struct Many<P> {
    /// The maximum number of items that can be parsed
    pub maximum: Option<usize>,
    /// The minimum number of items that can be parsed
    pub minimum: usize,
    /// The parser that is repeatedly applied
    pub parser: P,
}


impl<E, I, O, P> CollectionParser<E, I, O> for Many<P> where
    Self: Parser<I, Output = O, Error = E>,
    I: Input,
{

    fn at_least(self, minimum: usize) -> Self {
        let mut parser: Self = self;
        parser.minimum = minimum;
        parser
    }

    fn at_most(self, maximum: usize) -> Self {
        let mut parser: Self = self;
        parser.maximum = Some(maximum);
        parser
    }

}


impl<E, I, O, P> Parser<I> for Many<P> where
    I: Input,
    P: Parser<I, Error = E, Output = O>,
{

    type Error = E;

    type Output = Vec<O>;

    fn apply(&self, input: &mut I) -> ParseResult<Self::Output, Self::Error> {
        let cursor: usize = input.cursor();
        let maximum: usize = if let Some (maximum) = self.maximum { maximum } else { usize::MAX };
        let mut items: Vec<O> = Vec::new();
        let mut errors: Vec<E> = Vec::new();
        while items.len() < maximum {
            let item_cursor: usize = input.cursor();
            match self.parser.apply(input) {
                Failure(item_errors) => if items.len() < self.minimum {
                    input.set_cursor(cursor);
                    errors.extend(item_errors);
                    return Failure (errors)
                } else { break },
                Success (item, item_errors) => {
                    debug_assert!(input.cursor() > item_cursor);
                    items.push(item);
                    errors.extend(item_errors);
                }
            }
        }
        Success (items, errors)
    }

}

