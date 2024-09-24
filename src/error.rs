use thiserror::{Error};

#[derive(Error, Debug)]
pub enum PulseError {
    #[error("Couldn't find pulse.toml in current direcotr or any parent directory")]
    ProjectNotFound
}