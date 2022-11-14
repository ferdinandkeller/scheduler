use std::{error::Error, fmt::Display};

/// Error raised when parsing a custom schedule expression.
#[derive(Debug, Clone)]
pub enum ParsingError {
    /// the expression has too many or too few components
    InvalidExpressionLength {
        expression: String,
        expected: usize,
        got: usize,
    },

    // the provided number is not valid
    InvalidNumber {
        expression: String,
        number: String,
    },

    /// one of the list provided is invalid
    InvalidList {
        expression: String,
        list: String,
    },

    /// one of the range provided is invalid
    InvalidRange {
        expression: String,
        range: String,
    },

    /// one of the modulo provided is invalid
    InvalidModulo {
        expression: String,
        modulo: String,
    },
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidExpressionLength { expression, expected, got } => {
                write!(f, r#"The expression "{expression}" is invalid : it must contain {expected} components, but you provided {got}."#)
            },
            ParsingError::InvalidNumber { expression, number } => {
                write!(f, r#"The expression "{expression}" is invalid : the number "{number}" cannot be parsed."#)
            },
            ParsingError::InvalidList { expression, list } => {
                write!(f, r#"The expression "{expression}" is invalid : the list "{list}" is not a correct list."#)
            },
            ParsingError::InvalidRange { expression, range } => {
                write!(f, r#"The expression "{expression}" is invalid : the range {range} is not a correct range."#)
            },
            ParsingError::InvalidModulo { expression, modulo } => {
                write!(f, r#"The expression "{expression}" is invalid : the modulo {modulo} is not a correct modulo."#)
            },
        }
    }
}

impl Error for ParsingError {}
