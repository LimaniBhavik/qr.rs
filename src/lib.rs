pub mod error;
pub mod formats;
pub mod generator;
pub mod utils;

pub use error::QRError;
pub use formats::{ContactData, QRData};
pub use generator::{QRBuilder, QRGenerator};

// Re-export qrcode for convenience
pub use image;
pub use qrcode;
