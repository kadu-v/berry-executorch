use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorchError {
    #[error("FailedToCreateModule: {0}")]
    FailedToLoad(i32),
    #[error("MismatchShape: expected: {expected:?}, found: {found:?}")]
    ShapeMismatch { expected: Vec<i32>, found: Vec<i32> },
}
