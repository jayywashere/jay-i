use std::{error, fmt};

use ollama_rs::error::OllamaError;

#[derive(Debug)]
pub enum JayIError {
    OllamaError(OllamaError),
    StreamError,
}

impl fmt::Display for JayIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OllamaError(err) => {
                write!(f, "[OllamaError] {err}")
            }

            Self::StreamError => {
                write!(
                    f,
                    "[StreamError] An error occurred while streaming the response."
                )
            }
        }
    }
}

impl error::Error for JayIError {}
