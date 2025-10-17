use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebrtcApmError {
    #[error("No error occurred")]
    NoError,
    #[error("Unspecified error")]
    UnspecifiedError,
    #[error("Bad parameter error")]
    BadParameterError,
    #[error("Bad sample rate error")]
    BadSampleRateError,
    #[error("Null pointer error")]
    NullPointerError,
}

impl From<i32> for WebrtcApmError {
    fn from(value: i32) -> Self {
        match value {
            0 => WebrtcApmError::NoError,
            -1 => WebrtcApmError::UnspecifiedError,
            -6 => WebrtcApmError::BadParameterError,
            -7 => WebrtcApmError::BadSampleRateError,
            -8 => WebrtcApmError::NullPointerError,
            _ => WebrtcApmError::UnspecifiedError,
        }
    }
}

impl WebrtcApmError {
    pub fn is_ok(&self) -> bool {
        matches!(self, WebrtcApmError::NoError)
    }
}
