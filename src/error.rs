use thiserror::Error;

#[derive(Error, Debug)]
pub enum QRError {
    #[error("QR Code generation failed: {0}")]
    GenerationError(String),
}
