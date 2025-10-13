// Copyright Rob Gage 2025

/// Implementors represent input that can be consumed by parsers
pub trait Input<'a> {

    /// The type of item that is stored in the `Input`
    type Item;

    fn advance(&self);

    /// Returns the next `Self::Item` in `Self` if it exists and advances the cursor
    fn next(&self) -> Option<Self::Item> {
        let item: Self::Item = self.peek()?;
        self.advance();
        Some (item)
    }

    /// Returns the next `Self::Item` in `Self` if it exists
    fn peek(&self) -> Option<Self::Item>;

    /// Restores the cursor of this `Input` to a given position
    fn restore(&self, position: usize);

    /// Saves the position of the cursor of this `Input`
    fn save(&self) -> usize;

}