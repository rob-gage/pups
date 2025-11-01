/// Copyright Rob Gage 2025

use unicode_ident::{
    is_xid_continue,
    is_xid_start
};

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

    /// Returns true if the `Character` is a valid beginning to a Unicode identifier
    fn is_unicode_identifier_start(&self) -> bool;

    /// Returns true if the `Character` is a valid continuation of a Unicode identifier
    fn is_unicode_identifier_continuation(&self) -> bool;

    /// Writes the character at the end of a `&mut String`
    fn write(&self, buffer: &mut String);

}


impl Character for char {

    fn is_ascii_decimal(&self) -> bool { self.is_ascii_digit() }

    fn is_newline(&self) -> bool {
        char::is_whitespace(*self)
    }

    fn is_whitespace(&self) -> bool { char::is_whitespace(*self) }

    fn length(&self) -> usize { self.len_utf8() }

    fn is_unicode_identifier_start(&self) -> bool { is_xid_start(*self) }

    fn is_unicode_identifier_continuation(&self) -> bool { is_xid_continue(*self) }

    fn write(&self, buffer: &mut String) { buffer.push(*self) }

}