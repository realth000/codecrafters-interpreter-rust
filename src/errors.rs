use thiserror::Error;

pub type AppResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("[line {line}] Error: Unexpected character: {token}")]
    UnexpectedChar { line: usize, token: String },

    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: usize },
}
