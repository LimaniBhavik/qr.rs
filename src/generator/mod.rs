use crate::error::QRError;
use crate::formats::{format_url, generate_vcard, QRData};
use image::{DynamicImage, ImageFormat, Luma, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use qrcode::QrCode;
use std::io::Cursor;
use log::{debug, info};

pub struct QRBuilder {
    data: Option<QRData>,
    error_correction: qrcode::EcLevel,
    foreground_color: Rgba<u8>,
    background_color: Rgba<u8>,
}

impl Default for QRBuilder {
    fn default() -> Self {
        Self {
            data: None,
            error_correction: qrcode::EcLevel::H,
            foreground_color: Rgba([0, 0, 0, 255]),
            background_color: Rgba([255, 255, 255, 255]),
        }
    }
}

impl QRBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(mut self, data: QRData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn url(self, url: impl Into<String>) -> Self {
        self.data(QRData::URL(url.into()))
    }

    pub fn text(self, text: impl Into<String>) -> Self {
        self.data(QRData::Text(text.into()))
    }

    pub fn error_correction(mut self, level: qrcode::EcLevel) -> Self {
        self.error_correction = level;
        self
    }

    pub fn colors(mut self, foreground: [u8; 4], background: [u8; 4]) -> Self {
        self.foreground_color = Rgba(foreground);
        self.background_color = Rgba(background);
        self
    }

    pub fn build(self) -> Result<QRGenerator, QRError> {
        if self.data.is_none() {
            return Err(QRError::InvalidData("No data provided".to_string()));
        }

        Ok(QRGenerator {
            data: self.data.unwrap(),
            error_correction: self.error_correction,
            foreground_color: self.foreground_color,
            background_color: self.background_color,
        })
    }
}

pub struct QRGenerator {
    data: QRData,
    error_correction: qrcode::EcLevel,
    foreground_color: Rgba<u8>,
    background_color: Rgba<u8>,
}

impl QRGenerator {
    pub fn new(data: QRData) -> Self {
        Self {
            data,
            error_correction: qrcode::EcLevel::H,
            foreground_color: Rgba([0, 0, 0, 255]),
            background_color: Rgba([255, 255, 255, 255]),
        }
    }

    pub fn generate(&self) -> Result<QrCode, QRError> {
        debug!("Generating QR code for data: {:?}", self.data);

        let content = match &self.data {
            QRData::URL(url) => format_url(url),
            QRData::Text(text) => text.clone(),
            QRData::Contact(contact) => generate_vcard(contact),
        };

        QrCode::with_error_correction_level(&content, self.error_correction)
            .map_err(QRError::QrGenerationError)
    }

    pub fn to_image(&self, size: u32, logo: Option<&DynamicImage>) -> Result<DynamicImage, QRError> {
        let qr = self.generate()?;

        let qr_image = qr.render::<Luma<u8>>().min_dimensions(size, size).build();
        let width = qr_image.width();
        let height = qr_image.height();

        let mut image = RgbaImage::new(width, height);

        for (target_pixel, pixel) in image.pixels_mut().zip(qr_image.pixels()) {
            *target_pixel = if pixel.0[0] == 0 {
                self.foreground_color
            } else {
                self.background_color
            };
        }

        if let Some(logo_img) = logo {
            info!("Adding logo to QR code");
            // Calculate logo size (e.g., 20% of QR code size)
            let logo_size = (width as f32 * 0.2) as u32;
            let logo_resized = logo_img.resize(logo_size, logo_size, image::imageops::FilterType::Lanczos3);

            let x_offset = (width - logo_resized.width()) / 2;
            let y_offset = (height - logo_resized.height()) / 2;

            // Draw background for logo using background color
            let rect = Rect::at(x_offset as i32, y_offset as i32)
                .of_size(logo_resized.width(), logo_resized.height());
            draw_filled_rect_mut(&mut image, rect, self.background_color);

            image::imageops::overlay(&mut image, &logo_resized, x_offset as i64, y_offset as i64);
        }

        Ok(DynamicImage::ImageRgba8(image))
    }

    pub fn to_png(&self, size: u32, logo: Option<&DynamicImage>) -> Result<Vec<u8>, QRError> {
        let image = self.to_image(size, logo)?;

        let mut bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        image
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(QRError::ImageError)?;

        Ok(bytes)
    }

    pub fn to_svg(&self) -> Result<String, QRError> {
        let qr = self.generate()?;
        let mut binding = qr.render::<qrcode::render::svg::Color>();
        let builder = binding.min_dimensions(200, 200);

        // Use default colors for now due to lifetime issues with custom colors in SVG builder
        Ok(builder.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::ContactData;

    #[test]
    fn test_generate_text() {
        let generator = QRGenerator::new(QRData::Text("Hello World".to_string()));
        let result = generator.generate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_url() {
        let generator = QRGenerator::new(QRData::URL("example.com".to_string()));
        let result = generator.generate();
        assert!(result.is_ok());

        let generator_with_protocol = QRGenerator::new(QRData::URL("https://example.com".to_string()));
        let result_with_protocol = generator_with_protocol.generate();
        assert!(result_with_protocol.is_ok());
    }

    #[test]
    fn test_generate_contact() {
        let contact = ContactData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            ..Default::default()
        };
        let generator = QRGenerator::new(QRData::Contact(contact));
        let result = generator.generate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_error_correction() {
        let data = QRData::Text("Test EC".to_string());

        let levels = [
            qrcode::EcLevel::L,
            qrcode::EcLevel::M,
            qrcode::EcLevel::Q,
            qrcode::EcLevel::H,
        ];

        for &level in &levels {
            let generator = QRBuilder::new()
                .data(data.clone())
                .error_correction(level)
                .build()
                .unwrap();
            let result = generator.generate();
            assert!(result.is_ok(), "Failed for EC level {:?}", level);
        }
    }
}
