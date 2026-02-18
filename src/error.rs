use image::ImageError;
use qrcode::types::QrError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QRError {
    #[error("Invalid data format: {0}")]
    InvalidData(String),

    #[error("Data too large for selected error correction level")]
    DataOverflow,

    #[error("Invalid color format: {0}")]
    InvalidColor(String),

    #[error("Image generation failed: {0}")]
    ImageError(#[from] ImageError),

    #[error("QR code generation failed: {0}")]
    QrGenerationError(#[from] QrError),

    #[error("Generation error: {0}")]
    GenerationError(String), // Keep for backward compatibility/generic errors

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid vCard data: {field} - {reason}")]
    VCardError { field: String, reason: String },
}
