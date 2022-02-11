use super::{Position, Span};

#[test]
fn update_position_non_newline() {
    let mut position =
        Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 };
    position.update('a');
    assert_eq!(
        position,
        Position { line: 2, column: 4, utf8_index: 13, utf16_index: 11 }
    );
}

#[test]
fn update_position_newline() {
    let mut position =
        Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 };
    position.update('\n');
    assert_eq!(
        position,
        Position { line: 3, column: 1, utf8_index: 13, utf16_index: 11 }
    );
}

#[test]
fn update_position_unicode() {
    let mut position =
        Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 };
    position.update('ç');
    assert_eq!(
        position,
        Position { line: 2, column: 4, utf8_index: 14, utf16_index: 11 }
    );
}

#[test]
fn display_position() {
    let position =
        Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 };
    assert_eq!(position.to_string(), "linha 2 e coluna 3");
}

#[test]
fn update_span_non_newline() {
    let mut span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 3, column: 7, utf8_index: 28, utf16_index: 26 },
    };
    span.update('a');
    assert_eq!(
        span,
        Span {
            start: Position {
                line: 2,
                column: 3,
                utf8_index: 12,
                utf16_index: 10
            },
            end: Position {
                line: 3,
                column: 8,
                utf8_index: 29,
                utf16_index: 27
            },
        }
    );
}

#[test]
fn update_span_newline() {
    let mut span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 3, column: 7, utf8_index: 28, utf16_index: 26 },
    };
    span.update('\n');
    assert_eq!(
        span,
        Span {
            start: Position {
                line: 2,
                column: 3,
                utf8_index: 12,
                utf16_index: 10
            },
            end: Position {
                line: 4,
                column: 1,
                utf8_index: 29,
                utf16_index: 27
            },
        }
    );
}

#[test]
fn update_span_unicode() {
    let mut span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 3, column: 7, utf8_index: 28, utf16_index: 26 },
    };
    span.update('ç');
    assert_eq!(
        span,
        Span {
            start: Position {
                line: 2,
                column: 3,
                utf8_index: 12,
                utf16_index: 10
            },
            end: Position {
                line: 3,
                column: 8,
                utf8_index: 30,
                utf16_index: 27
            },
        }
    );
}

#[test]
fn finish_span() {
    let mut span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 3, column: 7, utf8_index: 28, utf16_index: 26 },
    };
    span.finish();
    assert_eq!(
        span,
        Span {
            start: Position {
                line: 3,
                column: 7,
                utf8_index: 28,
                utf16_index: 26
            },
            end: Position {
                line: 3,
                column: 7,
                utf8_index: 28,
                utf16_index: 26
            },
        }
    );
}

#[test]
fn display_span_length_one() {
    let span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 2, column: 4, utf8_index: 13, utf16_index: 11 },
    };
    assert_eq!(span.to_string(), "na linha 2 e coluna 3");
}

#[test]
fn display_span_same_line_big() {
    let span = Span {
        start: Position { line: 2, column: 3, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 2, column: 7, utf8_index: 13, utf16_index: 11 },
    };
    assert_eq!(span.to_string(), "da linha 2 e coluna 3, até a coluna 6");
}

#[test]
fn display_span_different_lines() {
    let span = Span {
        start: Position { line: 2, column: 8, utf8_index: 12, utf16_index: 10 },
        end: Position { line: 5, column: 2, utf8_index: 28, utf16_index: 26 },
    };
    assert_eq!(
        span.to_string(),
        "da linha 2 e coluna 8, até a linha 5 e coluna 1"
    );
}
