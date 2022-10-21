use derivative::Derivative;
use thiserror::Error;

#[derive(Derivative, Error)]
#[derivative(Debug, PartialEq, Eq)]
pub enum OtterError {
    #[error("could not parse pattern '{pattern}'")]
    RegexError { pattern: String },
}
