use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("convolution failure")]
    FFTFailure,
}
