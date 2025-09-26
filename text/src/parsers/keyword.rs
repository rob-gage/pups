// // Copyright Rob Gage 2025
//
// use crate::{
//     Character,
//     TextInput
// };
// use pups::{
//     Input,
//     ParseResult::{
//         self,
//         Failure,
//         Success,
//     },
//     Parser,
// };
//
// /// See `keyword`
// struct Keyword (String);
//
// impl<I, T> Parser<I> for Keyword where
//     I: Input<Item = T> + TextInput,
//     T: Character,
// {
//
//     type Error = ();
//
//     type Message ();
//
//     type Output = ();
//
// }
//
// /// Parses a keyword
// pub fn keyword(keyword: &str) -> Keyword { Keyword (keyword.to_string()) }