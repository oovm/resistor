use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// An error that can occur when building a resistor.
#[derive(Clone, Debug)]
pub struct ResistorError {
    message: String,
}

impl ResistorError {
    /// Create a new error with a custom message.
    pub fn custom<S: ToString>(message: S) -> Self {
        Self { message: message.to_string() }
    }
}

impl Error for ResistorError {}

impl Display for ResistorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
