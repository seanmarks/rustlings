// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::fmt;
use std::num::ParseIntError;
use std::error::Error;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

//-----------------------------------------------------------------------------
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<CreationError> for ParsePosNonzeroError {
    fn from(err: CreationError) -> Self {
        Self::Creation(err)
    }
}
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

//-----------------------------------------------------------------------------
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}
impl Error for CreationError{}

// impl fmt::Display for ParsePosNonzeroError {}

//-----------------------------------------------------------------------------
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        s.parse::<i64>().map_err(ParsePosNonzeroError::ParseInt)
          .and_then(|x| { Self::new(x).map_err(ParsePosNonzeroError::Creation) })
    }
}

//-----------------------------------------------------------------------------
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
