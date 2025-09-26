// // Copyright Rob Gage 2025
//
// use crate::{
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
// /// A combinator that applies one parser, and then another if the first fails, and returns
// /// the output from the one that succeeds or the errors from both if they fail
// pub struct Choice<P1, P2> {
//     /// The alternate parser
//     pub alternate: P2,
//     /// The primary parser
//     pub primary: P1,
// }
//
// impl<E, I, O, P1, P2> Parser<I> for Choice<P1, P2> where
//     I: Input,
//     P1: Parser<I, Error = E, Output = O>,
//     P2: Parser<I, Error = E, Output = O>,
// {
//
//     type Error = E;
//
//     type Output = O;
//
//     fn apply(&self, input: &mut I) -> ParseResult<O, E> {
//         match self.primary.apply(input) {
//             Failure (mut primary_errors) => match self.primary.apply(input) {
//                 Failure (alternate_errors) => {
//                     primary_errors.extend(alternate_errors);
//                     Failure (primary_errors)
//                 }
//                 success => success.with_errors(primary_errors),
//             }
//             success => success,
//         }
//     }
//
// }