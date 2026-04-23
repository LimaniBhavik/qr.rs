use crate::error::QRError;
use crate::formats::{format_url, generate_geo_uri, generate_vcard, generate_wifi, QRData};
use crate::utils::{BLACK, WHITE};
use std::borrow::Cow;
use image::{DynamicImage, ImageFormat, Luma, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use log::{debug, info};
use qrcode::QrCode;
use std::io::Cursor;

const DEFAULT_ERROR_CORRECTION: qrcode::EcLevel = qrcode::EcLevel::H;
const DEFAULT_FOREGROUND_COLOR: Rgba<u8> = Rgba(BLACK);
const DEFAULT_BACKGROUND_COLOR: Rgba<u8> = Rgba(WHITE);

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
            error_correction: DEFAULT_ERROR_CORRECTION,
            foreground_color: DEFAULT_FOREGROUND_COLOR,
            background_color: DEFAULT_BACKGROUND_COLOR,
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

    pub fn wifi(self, wifi: crate::formats::WifiData) -> Self {
        self.data(QRData::Wifi(wifi))
    }

    pub fn location(self, location: crate::formats::LocationData) -> Self {
        self.data(QRData::Location(location))
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
        let qr_data = self
            .data
            .ok_or_else(|| QRError::InvalidData("No data provided".to_string()))?;

        Ok(QRGenerator {
            data: qr_data,
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
        let builder = QRBuilder::default();
        QRGenerator {
            data,
            error_correction: builder.error_correction,
            foreground_color: builder.foreground_color,
            background_color: builder.background_color,
        }
    }

    pub fn generate(&self) -> Result<QrCode, QRError> {
        debug!("Generating QR code for data: {:?}", self.data);

        let content = match &self.data {
            QRData::URL(url) => Cow::Owned(format_url(url)),
            QRData::Text(text) => Cow::Borrowed(text.as_str()),
            QRData::Contact(contact) => {
                contact.validate()?;
                Cow::Owned(generate_vcard(contact))
            }
            QRData::Wifi(wifi) => {
                wifi.validate()?;
                Cow::Owned(generate_wifi(wifi))
            }
            QRData::Location(location) => {
                location.validate()?;
                Cow::Owned(generate_geo_uri(location))
            }
        };

        QrCode::with_error_correction_level(content.as_ref(), self.error_correction)
            .map_err(QRError::QrGenerationError)
    }

    pub fn to_image(
        &self,
        size: u32,
        logo: Option<&DynamicImage>,
    ) -> Result<DynamicImage, QRError> {
        let qr = self.generate()?;

        let qr_image = qr.render::<Luma<u8>>().min_dimensions(size, size).build();
        let width = qr_image.width();
        let height = qr_image.height();

        let fg = self.foreground_color.0;
        let bg = self.background_color.0;

        let qr_raw = qr_image.as_raw();

        let mut buffer = Vec::with_capacity(qr_raw.len() * 4);
        for &luma in qr_raw {
            let color = if luma == 0 { fg } else { bg };
            buffer.extend_from_slice(&color);
        }

        let mut image = RgbaImage::from_raw(width, height, buffer).ok_or_else(|| {
            QRError::GenerationError("Failed to create image from raw buffer".to_string())
        })?;

        if let Some(logo_img) = logo {
            info!("Adding logo to QR code");
            // Calculate logo size (e.g., 20% of QR code size)
            let logo_size = (width as f32 * 0.2) as u32;
            let logo_resized =
                logo_img.resize(logo_size, logo_size, image::imageops::FilterType::Triangle);

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

        let fg_hex = format!(
            "#{:02X}{:02X}{:02X}",
            self.foreground_color.0[0], self.foreground_color.0[1], self.foreground_color.0[2]
        );
        let bg_hex = format!(
            "#{:02X}{:02X}{:02X}",
            self.background_color.0[0], self.background_color.0[1], self.background_color.0[2]
        );

        let mut binding = qr.render::<qrcode::render::svg::Color>();
        let builder = binding
            .min_dimensions(200, 200)
            .dark_color(qrcode::render::svg::Color(&fg_hex))
            .light_color(qrcode::render::svg::Color(&bg_hex));

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

        let generator_with_protocol =
            QRGenerator::new(QRData::URL("https://example.com".to_string()));
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
    fn test_generate_wifi() {
        let wifi = crate::formats::WifiData {
            ssid: "TestNet".to_string(),
            password: "pass".to_string(),
            ..Default::default()
        };
        let generator = QRGenerator::new(QRData::Wifi(wifi));
        let result = generator.generate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_location() {
        let location = crate::formats::LocationData {
            latitude: 34.0,
            longitude: -118.0,
        };
        let generator = QRGenerator::new(QRData::Location(location));
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
                .expect("QRBuilder should build successfully with valid data");
            let result = generator.generate();
            assert!(result.is_ok(), "Failed for EC level {:?}", level);
        }
    }

    #[test]
    fn test_builder_missing_data_error() {
        let builder = QRBuilder::new();
        let result = builder.build();

        assert!(result.is_err());
        match result {
            Err(QRError::InvalidData(msg)) => assert_eq!(msg, "No data provided"),
            _ => panic!("Expected QRError::InvalidData"),
        }
    }

    #[test]
    fn test_to_png() {
        let generator = QRGenerator::new(QRData::Text("PNG Test".to_string()));
        let result = generator.to_png(200, None);

        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());

        // Verify it's a valid PNG
        let image_result = image::load_from_memory_with_format(&bytes, ImageFormat::Png);
        assert!(image_result.is_ok());
    }

    #[test]
    fn test_to_png_with_logo() {
        let generator = QRGenerator::new(QRData::Text("PNG with Logo Test".to_string()));

        // Create a small 10x10 red square as a dummy logo
        let mut logo_image = RgbaImage::new(10, 10);
        for pixel in logo_image.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[255, 0, 0, 255]);
        }
        let logo = DynamicImage::ImageRgba8(logo_image);

        let result = generator.to_png(200, Some(&logo));

        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());

        // Verify it's a valid PNG
        let image_result = image::load_from_memory_with_format(&bytes, ImageFormat::Png);
        assert!(image_result.is_ok());
    }

    #[test]
    fn test_to_svg() {
        let generator = QRGenerator::new(QRData::Text("SVG Test".to_string()));
        let result = generator.to_svg();

        assert!(result.is_ok());
        let svg = result.unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("http://www.w3.org/2000/svg"));
    }

    #[test]
    fn test_to_svg_wifi() {
        let wifi = crate::formats::WifiData {
            ssid: "TestNet".to_string(),
            password: "pass".to_string(),
            ..Default::default()
        };
        let generator = QRGenerator::new(QRData::Wifi(wifi));
        let result = generator.to_svg();

        assert!(result.is_ok());
        let svg = result.unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }

    #[test]
    fn test_to_svg_custom_colors() {
        let fg = [255, 0, 0, 255]; // Red
        let bg = [0, 255, 0, 255]; // Green

        let generator = QRBuilder::new()
            .text("SVG Custom Colors Test")
            .colors(fg, bg)
            .build()
            .expect("Should build with custom colors");

        let result = generator.to_svg();
        assert!(result.is_ok());
        let svg = result.unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("#FF0000")); // Check for foreground color
        assert!(svg.contains("#00FF00")); // Check for background color
    }
}
