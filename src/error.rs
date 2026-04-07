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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_display() {
        assert_eq!(
            QRError::InvalidData("test".to_string()).to_string(),
            "Invalid data format: test"
        );
        assert_eq!(
            QRError::DataOverflow.to_string(),
            "Data too large for selected error correction level"
        );
        assert_eq!(
            QRError::InvalidColor("#G12345".to_string()).to_string(),
            "Invalid color format: #G12345"
        );
        assert_eq!(
            QRError::GenerationError("Something went wrong".to_string()).to_string(),
            "Generation error: Something went wrong"
        );
        assert_eq!(
            QRError::VCardError {
                field: "FN".to_string(),
                reason: "Empty".to_string()
            }
            .to_string(),
            "Invalid vCard data: FN - Empty"
        );
    }

    #[test]
    fn test_error_conversions() {
        // Test IoError conversion
        let io_err = io::Error::new(io::ErrorKind::Other, "test io error");
        let qr_err: QRError = io_err.into();
        assert_eq!(qr_err.to_string(), "IO error: test io error");

        // Test QrError conversion
        let qrc_err = QrError::DataTooLong;
        let qr_err: QRError = qrc_err.into();
        assert_eq!(
            qr_err.to_string(),
            format!("QR code generation failed: {}", QrError::DataTooLong)
        );

        // Test ImageError conversion
        // ImageError is a bit more complex, but we can verify it's wrapped
        // using a dummy error if possible, or just checking the from implementation.
        // For simplicity, we can use one of its variants.
        let img_err = ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ));
        let qr_err: QRError = img_err.into();
        assert!(qr_err.to_string().starts_with("Image generation failed:"));
    }
}
