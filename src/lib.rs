pub mod error;
pub mod formats;
pub mod generator;
pub mod utils;

pub use error::QRError;
pub use formats::{ContactData, LocationData, QRData, WifiData, WifiEncryption};
pub use generator::{QRBuilder, QRGenerator};

// Re-export qrcode for convenience
pub use image;
pub use qrcode;
