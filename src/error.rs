pub type Res<T> = Result<T, StampError>;

#[derive(Debug, thiserror::Error)]
pub enum StampError {
    #[error("Unknown error")]
    Unknown,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template error: {0}")]
    Template(#[from] minijinja::Error),
}
