use thiserror::Error;

#[repr(u32)]
#[derive(Error, Debug)]
pub enum ExecutorchError {
    #[error("CxxInternal")]
    CxxInternal = 0x01,
    #[error("CxxInvalidState")]
    CxxInvalidState = 0x02,
    #[error("CxxEndOfMethod")]
    CxxEndOfMethod = 0x03,
    #[error("CxxNotSupported")]
    CxxNotSupported = 0x04,
    #[error("CxxNotImplemented")]
    CxxNotImplemented = 0x10,
    #[error("CxxInvalidArgument")]
    CxxInvalidArgument = 0x11,
    #[error("CxxInvalidType")]
    CxxInvalidType = 0x12,
    #[error("CxxOperationMissing")]
    CxxOperationMissing = 0x13,
    #[error("CxxNotFound")]
    CxxNotFound = 0x14,
    #[error("CxxMemoryAllocationFailed")]
    CxxMemoryAllocationFailed = 0x20,
    #[error("CxxAccessFailed")]
    CxxAccessFailed = 0x21,
    #[error("CxxInvalidProgram")]
    CxxInvalidProgram = 0x22,
    #[error("CxxDelegateInvalidCompatibility")]
    CxxDelegateInvalidCompatibility = 0x30,
    #[error("CxxDelegateMemoryAllocationFailed")]
    CxxDelegateMemoryAllocationFailed = 0x31,
    #[error("CxxDelegateInvalidHandle")]
    CxxDelegateInvalidHandle = 0x32,
    #[error("MismatchShape: expected: {expected:?}, found: {found:?}")]
    ShapeMismatch { expected: Vec<i32>, found: Vec<i32> } = 0x40,
    #[error("FailedToForward: {0}")]
    FailedToForward(i32) = 0x41,
    #[error("UnknownError")]
    UnknownError = 0xFF,
}

impl ExecutorchError {
    pub(crate) const fn from_i32(value: i32) -> Self {
        match value {
            0x01 => Self::CxxInternal,
            0x02 => Self::CxxInvalidState,
            0x03 => Self::CxxEndOfMethod,
            0x04 => Self::CxxNotSupported,
            0x10 => Self::CxxNotImplemented,
            0x11 => Self::CxxInvalidArgument,
            0x12 => Self::CxxInvalidType,
            0x13 => Self::CxxOperationMissing,
            0x14 => Self::CxxNotFound,
            0x20 => Self::CxxMemoryAllocationFailed,
            0x21 => Self::CxxAccessFailed,
            0x22 => Self::CxxInvalidProgram,
            0x30 => Self::CxxDelegateInvalidCompatibility,
            0x31 => Self::CxxDelegateMemoryAllocationFailed,
            0x32 => Self::CxxDelegateInvalidHandle,
            0x40 => Self::ShapeMismatch {
                expected: vec![],
                found: vec![],
            },
            0x41 => Self::FailedToForward(0),
            _ => Self::UnknownError,
        }
    }
}
