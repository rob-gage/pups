# Pups - Pretty Understandable Parsers

Pups is a parser combinators library. It is designed to work for any type of input, whether that be text, or some other
sequenced data. It provides a number of elementary combinators that allow the easy construction of larger parsers.

## Features

### Modal Parsing

Parsers can be run in different modes, that return different information. A user may choose to collect all parsed data, 
accumulated errors, and messages, or they may choose to simply check if the input is valid for the parser. These modes 
allow for much faster parsers when not all data is needed.

### Combinator Methods

All parsers can be used in combinators such as `.or_not()`, `.then()`, `.map()` and others. This allows the composition
of complete parsers from individual components.

### Strong Error and Message Control

The Pups library makes a type-level distinction between fatal errors that immediately stop the parsing process, and
accumulated messages (useful for compiler errors and warnings that allow recovery) that are simply tracked alongside
the output.