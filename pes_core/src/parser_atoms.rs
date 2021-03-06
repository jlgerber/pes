//! Provides low level parsers used in crate::parser
use nom::{
    branch::alt,
    bytes::complete::{
        tag,
        take,
        take_till,
    },
    character::complete::{
        alpha1,
        alphanumeric0,
        alphanumeric1,
        multispace0,
    },
    combinator::recognize,
    error::ParseError,
    IResult,
    multi::many0,
    sequence::{
        delimited, 
        pair,
    },
};

use crate::PNResult;


/// Parse a str that starts with a letter, followed by zero or more
/// letters and/or numbers
///
/// # Example
///
/// ```
/// use pes_core::parser_atoms::alphaword;
/// use nom::combinator::complete;
///
/// let result = complete(alphaword)("a123a5");
/// assert_eq!(result, Ok(("","a123a5")));
/// ```
pub fn alphaword(input: &str) -> PNResult<&str, &str> {
    recognize(pair(alpha1, alphanumeric0))(input)
}

/// Parse a single underscore followed by an alphanum
///
/// # Example
///
/// ```
/// use pes_core::parser_atoms::underscore_word;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_word)("_1foo1");
/// assert_eq!(result, Ok(("","_1foo1")));
/// ```
pub fn underscore_word(input: &str) -> PNResult<&str, &str> {
    recognize(pair(tag("_"), alphanumeric1))(input)
}

/// Given a str starting with an alphaword, and followed by zero or more _words,
/// parse it.
///
/// # Examples
///
/// ```
/// use pes_core::parser_atoms::alphaword_many0_underscore_word;
/// use nom::combinator::complete;
///
/// let result = complete(alphaword_many0_underscore_word)("fred1_1bla_foobar");
/// assert_eq!(result, Ok(("","fred1_1bla_foobar")));
/// ```
pub fn alphaword_many0_underscore_word(input: &str) -> PNResult<&str, &str> {
    recognize(pair(alphaword, many0(underscore_word)))(input)
}

/// This parser recognizes 3 conditions:
///
/// - a '#' followed by anything, up to and including a \n
/// - a '#' followed by anything
/// - a zero or more spaces followed by an optional \n
///
/// # Examples
///
/// ## Comment
/// ```
/// use pes_core::parser_atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("# this is an example");
/// assert_eq!(result, Ok(("", "# this is an example")));
/// ```
/// ## spaces
/// ```
/// use pes_core::parser_atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("    ").unwrap();
/// assert_eq!(result, (("","    ")) );
/// ```
///
/// ## comment with newline
/// ```
/// use pes_core::parser_atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("# this is an example\n");
/// assert_eq!(result, Ok(("", "# this is an example\n")));
/// ```
/// ## spaces with newline
/// ```
/// use pes_core::parser_atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("    \n").unwrap();
/// assert_eq!(result, (("","    \n")) );
/// ```
pub fn space0_eol(input: &str) -> PNResult<&str, &str> {
    alt((
        // this one ends in a \n
        recognize(pair(
            tag("#"),
            recognize(pair(take_till(|c| c == '\n'), take(1usize))),
        )),
        // this one doesnt (like if it is the last line of the file)
        recognize(pair(tag("#"), take_till(|c| c == '\n'))),
        // this is just zero or more spaces and optionally a \n
        multispace0,
    ))(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

#[cfg(test)]
#[path = "./unit_tests/parser_atoms.rs"]
mod unit_tests;