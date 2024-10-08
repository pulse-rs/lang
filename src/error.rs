use crate::lexer::span::TextSpan;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PulseError {
    #[error("Couldn't find pulse.toml in current directory or any parent directory")]
    ProjectNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Project at this path already exists")]
    ProjectAlreadyExists,
    #[error("Invalid project structure. Expected src/main.pulse or src/lib.pulse")]
    InvalidProjectStructure,
    #[error("Found both src/main.pulse and src/lib.pulse. Only one is allowed")]
    MultipleEntryPoints,
    #[error("Invalid token: {0}")]
    InvalidToken(String, TextSpan),
    #[error("Expected {0}.")]
    ExpectedToken(String, String, TextSpan),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String, TextSpan),
    #[error("{0}")]
    SemanticError(String, TextSpan),
    #[error("Semantic error: {0}")]
    ResolverError(String),
}
