use super::{Diagnostics, Error};
use crate::position::{Position, Span};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone)]
struct SomeError;

impl fmt::Display for SomeError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Algum erro aconteceu")
    }
}

impl StdError for SomeError {}

#[derive(Debug, Clone)]
struct SevereError {
    level: u32,
}

impl fmt::Display for SevereError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Um erro severo de nível {} aconteceu", self.level)
    }
}

impl StdError for SevereError {}

#[test]
fn error_simple() {
    let error = Error::new(
        SomeError,
        Span {
            start: Position {
                line: 9,
                column: 4,
                utf8_index: 39,
                utf16_index: 39,
            },
            end: Position {
                line: 9,
                column: 11,
                utf8_index: 46,
                utf16_index: 46,
            },
        },
    );

    assert_eq!(
        error.to_string(),
        "Algum erro aconteceu, da linha 9 e coluna 4, até a coluna 10"
    );

    assert_eq!(
        error.span(),
        Some(Span {
            start: Position {
                line: 9,
                column: 4,
                utf8_index: 39,
                utf16_index: 39,
            },
            end: Position {
                line: 9,
                column: 11,
                utf8_index: 46,
                utf16_index: 46,
            },
        })
    );

    assert_eq!(error.cause().to_string(), "Algum erro aconteceu");
}

#[test]
fn error_complex() {
    let error = Error::new(
        SevereError { level: 3 },
        Span {
            start: Position {
                line: 4,
                column: 7,
                utf8_index: 20,
                utf16_index: 20,
            },
            end: Position {
                line: 4,
                column: 8,
                utf8_index: 21,
                utf16_index: 21,
            },
        },
    );

    assert_eq!(
        error.to_string(),
        "Um erro severo de nível 3 aconteceu, na linha 4 e coluna 7"
    );

    assert_eq!(
        error.span(),
        Some(Span {
            start: Position {
                line: 4,
                column: 7,
                utf8_index: 20,
                utf16_index: 20,
            },
            end: Position {
                line: 4,
                column: 8,
                utf8_index: 21,
                utf16_index: 21,
            },
        })
    );

    assert_eq!(
        error.cause().to_string(),
        "Um erro severo de nível 3 aconteceu"
    );
}

#[test]
fn error_no_span() {
    let error = Error::with_no_span(SomeError);
    assert_eq!(error.to_string(), "Algum erro aconteceu");
    assert_eq!(error.span(), None);
    assert_eq!(error.cause().to_string(), "Algum erro aconteceu");

    let error = Error::with_no_span(SevereError { level: 4 });
    assert_eq!(error.to_string(), "Um erro severo de nível 4 aconteceu");
    assert_eq!(error.span(), None);
    assert_eq!(
        error.cause().to_string(),
        "Um erro severo de nível 4 aconteceu"
    );
}

#[test]
fn diagnostics_is_ok() {
    let diagnostics = Diagnostics::new();
    assert!(diagnostics.is_ok());
    assert!(!diagnostics.is_err());
}

#[test]
fn diagnostics_is_err() {
    let mut diagnostics = Diagnostics::new();

    diagnostics.raise(Error::new(
        SomeError,
        Span {
            start: Position {
                line: 1,
                column: 10,
                utf8_index: 9,
                utf16_index: 9,
            },
            end: Position {
                line: 1,
                column: 14,
                utf8_index: 13,
                utf16_index: 13,
            },
        },
    ));
    assert!(diagnostics.is_err());
    assert!(!diagnostics.is_ok());

    diagnostics.raise(Error::with_no_span(SevereError { level: 5 }));
    assert!(diagnostics.is_err());
    assert!(!diagnostics.is_ok());
}

#[test]
fn diagnostics_iter_empty() {
    let diagnostics = Diagnostics::new();
    assert_eq!(diagnostics.iter().next().map(|error| error.to_string()), None);
    assert_eq!(
        diagnostics.into_iter().next().map(|error| error.to_string()),
        None
    );
}

#[test]
fn diagnostics_iter_non_empty() {
    let mut diagnostics = Diagnostics::new();

    diagnostics.raise(Error::new(
        SomeError,
        Span {
            start: Position {
                line: 1,
                column: 10,
                utf8_index: 9,
                utf16_index: 9,
            },
            end: Position {
                line: 1,
                column: 14,
                utf8_index: 13,
                utf16_index: 13,
            },
        },
    ));
    diagnostics.raise(Error::with_no_span(SevereError { level: 5 }));

    let expected_msgs = [
        "Algum erro aconteceu, da linha 1 e coluna 10, até a coluna 13",
        "Um erro severo de nível 5 aconteceu",
    ];

    let mut expected_iter = expected_msgs.into_iter();
    for error in &diagnostics {
        assert_eq!(Some(error.to_string().as_ref()), expected_iter.next());
    }
    assert_eq!(expected_iter.next(), None);

    let mut expected_iter = expected_msgs.into_iter();
    for error in diagnostics {
        assert_eq!(Some(error.to_string().as_ref()), expected_iter.next());
    }
    assert_eq!(expected_iter.next(), None);
}
