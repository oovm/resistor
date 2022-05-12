#[derive(Debug, Copy, Clone)]
pub struct ResistorError {
    message: String,
}

impl ResistorError {
    pub fn custom<S: ToString>(message: S) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}