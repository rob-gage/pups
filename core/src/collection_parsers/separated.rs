// // Copyright Rob Gage 2025
// 
// use crate::{
//     CollectionParser,
//     Input,
//     ParseResult::{
//         self,
//         Failure,
//         Success,
//     },
//     Parser,
// };
// 
// 
// /// A combinator that applies a parser repeatedly separated by a separator parser
// pub struct Separated<P1, P2> {
//     /// The maximum number of items that can be parsed
//     pub maximum: Option<usize>,
//     /// The minimum number of items that can be parsed
//     pub minimum: usize,
//     /// The parser that is repeatedly applied
//     pub parser: P1,
//     /// The separator that is applied between parsed items
//     pub separator: P2
// }
// 
// 
// impl<E, I, O, P1, P2> CollectionParser<E, I, O> for Separated<P1, P2> where
//     Self: Parser<I, Output = O, Error = E>,
//     I: Input,
// {
// 
//     fn at_least(self, minimum: usize) -> Self {
//         let mut parser: Self = self;
//         parser.minimum = minimum;
//         parser
//     }
// 
//     fn at_most(self, maximum: usize) -> Self {
//         let mut parser: Self = self;
//         parser.maximum = Some(maximum);
//         parser
//     }
// 
// }
// 
// 
// impl<E, I, O1, O2, P1, P2> Parser<I> for Separated<P1, P2> where
//     I: Input,
//     P1: Parser<I, Error = E, Output = O1>,
//     P2: Parser<I, Error = E, Output = O2>,
// {
// 
//     type Error = E;
// 
//     type Output = Vec<O1>;
// 
//     fn apply(&self, input: &mut I) -> ParseResult<Self::Output, Self::Error> {
//         let cursor: usize = input.cursor();
//         let maximum: usize = if let Some (maximum) = self.maximum { maximum } else { usize::MAX };
//         let mut items: Vec<O1> = Vec::new();
//         let mut errors: Vec<E> = if maximum != 0 { match self.parser.apply(input) {
//             Failure (errors) => return Failure (errors),
//             Success (item, errors) => {
//                 items.push(item);
//                 errors
//             }
//         }} else { return Success (vec![], vec![]) };
//         while items.len() < maximum {
//             let item_cursor: usize = input.cursor();
//             match self.separator.apply(input) {
//                 Failure (separator_errors) => return if items.len() < self.minimum {
//                     input.set_cursor(cursor);
//                     errors.extend(separator_errors);
//                     Failure(errors)
//                 } else { Success (items, errors) },
//                 Success (_, separator_errors) => {
//                     match self.parser.apply(input) {
//                         Failure (item_errors) => return if items.len() < self.minimum {
//                             input.set_cursor(cursor);
//                             errors.extend(separator_errors);
//                             errors.extend(item_errors);
//                             Failure(errors)
//                         } else {
//                             input.set_cursor(item_cursor);
//                             Success (items, errors)
//                         },
//                         Success (item, item_errors) => {
//                             debug_assert!(input.cursor() > item_cursor);
//                             errors.extend(separator_errors);
//                             errors.extend(item_errors);
//                             items.push(item);
//                         }
//                     };
//                 }
//             };
// 
//         }
//         Success (items, errors)
//     }
// 
// }
// 
