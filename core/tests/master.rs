// Copyright Rob Gage 2025

use pups_core::{
    Input,
    prelude::*,
    Mode
};


#[derive(Clone, Eq, PartialEq)]
enum TestItem {
    A (String),
    B (usize),
    C (isize),
}

impl<'a> Parser<'a, TestInput> for TestItem {

    type Output = TestItem;

    type Error = TestError;

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut TestInput
    ) -> ModeResult<TestItem, TestError, (), _Mode> {
        let cursor: usize = input.save();
        let Some(item) = input.next() else {
            input.restore(cursor);
            return ModeResult::Failure (
                _Mode::convert_error(TestError {
                    encountered_item: None,
                    expected_item: self.clone(),
                    position: input.cursor
                }),
                _Mode::new_message_container()
            )
        };
        if item == *self {
            ModeResult::Success(
                _Mode::convert_output(item),
                _Mode::new_message_container()
            )
        } else {
            input.restore(cursor);
            ModeResult::Failure (
                _Mode::convert_error(TestError {
                    encountered_item: Some(item),
                    expected_item: self.clone(),
                    position: input.cursor
                }),
                _Mode::new_message_container()
            )
        }
    }

}


struct TestError {
    encountered_item: Option<TestItem>,
    expected_item: TestItem,
    position: usize,
}


struct TestInput {
    items: Vec<TestItem>,
    cursor: usize,
}

impl<'a> Input<'a> for TestInput {

    type Item = TestItem;

    fn advance(&mut self) { self.cursor += 1; }

    fn save(&self) -> usize { self.cursor }

    fn restore(&mut self, position: usize) { self.cursor = position }

    fn peek(&self) -> Option<Self::Item> {
        self.items.get(self.cursor).map(|item| item.clone())
    }

}