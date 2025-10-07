// Copyright Rob Gage 2025

/// Implementors represent input that can be consumed by parsers
pub trait Input {

    /// The type of item that is stored in the `Input`
    type Item;

    fn advance(&mut self);

    /// Returns the cursor position in this `Input`
    fn cursor(&self) -> usize;

    /// Sets the cursor to a new position
    fn set_cursor(&mut self, position: usize);

    /// Returns the next `Self::Item` in `Self` if it exists and advances the cursor
    fn next(&mut self) -> Option<Self::Item> {
        let item: Self::Item = self.peek()?;
        self.advance();
        Some (item)
    }

    /// Returns the next `Self::Item` in `Self` if it exists
    fn peek(&self) -> Option<Self::Item>;

}