pub type SCError = Box<dyn std::error::Error + Send + Sync>;

// Import necessary traits
use std::error::Error;
use std::fmt;

// Define the custom error type
#[derive(Debug)]
pub enum SuperlightError {
    Parsing(String),
}

impl fmt::Display for SuperlightError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuperlightError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

// custom error type
impl Error for SuperlightError {}

pub fn err(e: SuperlightError) -> Box<SuperlightError> {
    Box::new(e)
}