use thiserror::{Error};

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
}