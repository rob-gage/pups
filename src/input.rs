// Copyright Rob Gage 2025

/// Implementors represent input that can be consumed by parsers
pub trait Input {

    /// The type of item that is stored in the `Input`
    type Item;

    /// Returns the cursor position in this `Input`
    fn cursor(&self) -> usize;

    /// Returns the next `Self::Item` in `Self` if it exists and advances the cursor
    fn next(&mut self) -> Option<Self::Item> {
        if let Some (item) = self.peek() {
            self.set_cursor(self.cursor() + 1);
            Some (item)
        } else { None }
    }

    /// Returns the next `Self::Item` in `Self` if it exists
    fn peek(&self) -> Option<Self::Item>;

    /// Sets the cursor to a new position
    fn set_cursor(&mut self, position: usize);

}