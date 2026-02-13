use crate::error::QRError;
use crate::formats::{format_url, generate_vcard, QRData};
use image::{ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;

pub struct QRGenerator {
    pub error_correction: qrcode::EcLevel,
}

impl Default for QRGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl QRGenerator {
    pub fn new() -> Self {
        Self {
            error_correction: qrcode::EcLevel::H,
        }
    }

    pub fn generate(&self, data: &QRData) -> Result<QrCode, QRError> {
        let content = match data {
            QRData::URL(url) => format_url(url),
            QRData::Text(text) => text.clone(),
            QRData::Contact(contact) => generate_vcard(contact),
        };

        QrCode::with_error_correction_level(content, self.error_correction)
            .map_err(|e| QRError::GenerationError(e.to_string()))
    }

    pub fn to_png(&self, qr: &QrCode, size: u32) -> Result<Vec<u8>, QRError> {
        let image = qr.render::<Luma<u8>>().min_dimensions(size, size).build();

        let mut bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        image
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| QRError::GenerationError(e.to_string()))?;

        Ok(bytes)
    }

    pub fn to_svg(&self, qr: &QrCode) -> String {
        qr.render::<qrcode::render::svg::Color>()
            .min_dimensions(200, 200)
            .build()
    }
}
