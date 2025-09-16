/// Copyright Rob Gage 2025

/// Implementors represent a unicode character
pub trait Character where
    Self: Sized
{

    /// Returns `true` if this `Character` is an ASCII decimal digit
    fn is_ascii_decimal(&self) -> bool;

    /// Returns `true` if this `Character` is a newline
    fn is_newline(&self) -> bool;

    /// Returns `true` if this `Character` is whitespace
    fn is_whitespace(&self) -> bool;

    /// The length of this `Character` in bytes
    fn length(&self) -> usize;

    /// Returns the next `Character` from a given string slice
    fn next(string: &str) -> Option<Self>;

    /// Writes the character at the end of a `&mut String`
    fn write(&self, buffer: &mut String);

}

impl Character for char {

    fn is_ascii_decimal(&self) -> bool { self.is_ascii_digit() }

    fn is_newline(&self) -> bool {
        matches!(self,
            '\u{000A}' | // line feed (LF)
            '\u{000B}' | // vertical tab (VT)
            '\u{000C}' | // form feed (FF)
            '\u{000D}' | // carriage return (CR)
            '\u{0085}' | // next line (NEL)
            '\u{2028}' | // line separator
            '\u{2029}'   // paragraph separator
        )
    }

    fn is_whitespace(&self) -> bool { char::is_whitespace(*self) }

    fn length(&self) -> usize { self.len_utf8() }

    fn next(string: &str) -> Option<Self> { string.chars().next() }

    fn write(&self, buffer: &mut String) { buffer.push(*self) }

}