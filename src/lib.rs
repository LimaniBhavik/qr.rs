pub mod error;
pub mod formats;
pub mod qr;

pub use error::QRError;
pub use formats::{format_url, generate_vcard, ContactData, QRData};
pub use qr::QRGenerator;

// Re-export qrcode for convenience
pub use image;
pub use qrcode;
