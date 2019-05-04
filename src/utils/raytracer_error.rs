use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;
use std::string::ToString;

#[derive(Debug)]
pub enum RayTracerError {
    ColorInvalid(String),
}

impl Display for RayTracerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: String = match self {
            RayTracerError::ColorInvalid(s) => {
                format!("color {} value not between 0 and 1 ", s).to_string()
            }
        };
        write!(f, "{}", description)
    }
}

impl StdError for RayTracerError {
    fn cause(&self) -> Option<&StdError> {
        None
    }
}
