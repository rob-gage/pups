// Copyright Rob Gage 2025

use pups_core::{
    implement_modes,
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

impl<'a> Parser<'a, &'a TestItem, TestError<'a>, (), TestInput> for TestItem {

    fn apply<_Mode: Mode>(
        &self,
        input: &'a TestInput
    ) -> ModeResult<&'a TestItem, TestError<'a>, (), _Mode> {
        let cursor: usize = input.save_cursor();
        let Some(item) = input.next() else {
            input.restore_cursor(cursor);
            return ModeResult::Failure (
                _Mode::convert_error(TestError {
                    encountered_item: None,
                    expected_item: self.clone(),
                    position: input.cursor
                }),
                _Mode::new_message_container()
            )
        };
        if *item == *self {
            ModeResult::Success(
                _Mode::convert_output(item),
                _Mode::new_message_container()
            )
        } else {
            input.restore_cursor(cursor);
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

    implement_modes!('a, &'a TestItem, TestError<'a>, (), TestInput);

}


struct TestError<'a> {
    encountered_item: Option<&'a TestItem>,
    expected_item: TestItem,
    position: usize,
}


struct TestInput {
    items: Vec<TestItem>,
    cursor: usize,
}

impl<'a> Input<'a> for TestInput {

    type Item = &'a TestItem;

    type Slice = &'a [TestItem];

    fn advance(&self) {
        unsafe {
            let mutable: *mut Self = self as *const Self as *mut Self;
            (*mutable).cursor += 1;
        }
    }

    fn consume(&'a self, length: usize) -> Option<Self::Slice> {
        if length > (self.items.len() - self.cursor) { None } else {
            let slice: Self::Slice = &self.items[self.cursor .. self.cursor + length];
            unsafe {
                let mutable: *mut Self = self as *const Self as *mut Self;
                (*mutable).cursor += length;
            }
            Some (slice)
        }
    }

    fn peek(&'a self) -> Option<Self::Item> { self.items.get(self.cursor) }

    fn restore_cursor(&self, position: usize) {
        unsafe {
            let mutable: *mut Self = self as *const Self as *mut Self;
            (*mutable).cursor = position;
        }
    }

    fn save_cursor(&self) -> usize { self.cursor }

}