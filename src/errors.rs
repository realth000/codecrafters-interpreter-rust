use thiserror::Error;

pub type AppResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("unknown token \"{token}\" at {pos}")]
    UnknownToken { pos: usize, token: String },
}
