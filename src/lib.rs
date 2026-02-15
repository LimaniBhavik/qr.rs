pub mod error;
pub mod formats;
pub mod generator;
pub mod utils;

pub use generator::{QRBuilder, QRGenerator};
pub use formats::{QRData, ContactData};
pub use error::QRError;

// Re-export qrcode for convenience
pub use qrcode;
pub use image;
