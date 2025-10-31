// Copyright Rob Gage 2025

/// Implementors represent input that can be consumed by parsers
pub trait Input<'a> {

    /// The type of item that is stored in the `Input`
    type Item;

    /// The type of slice that can be taken from this `Input`
    type Slice;

    /// Advances the cursor by one position increment
    fn advance(&self);

    /// Returns the next `Self::Item` in `Self` if it exists
    fn peek(&'a self) -> Option<Self::Item>;

    /// Gets a slice from a given start cursor to a given end cursor, panicking on bounds errors
    fn slice(&'a self, start: usize, end: usize) -> Self::Slice;

    /// Restores the cursor of this `Input` to a given position
    fn move_cursor(&self, position: usize);

    /// Saves the position of the cursor of this `Input`
    fn store_cursor(&self) -> usize;

}