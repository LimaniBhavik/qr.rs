use eframe::{egui, App, Frame};
use qr_rs::{ContactData, QRData, QRGenerator};
use qr_rs::qrcode::EcLevel;

#[derive(PartialEq, Debug)]
enum Mode {
    Url,
    Text,
    Contact,
}

struct QRApp {
    mode: Mode,
    url_input: String,
    text_input: String,
    contact: ContactData,
    qr_texture: Option<egui::TextureHandle>,
    generator: QRGenerator,

    // Customization state
    ec_level: EcLevel,
    foreground_color: [u8; 3],
    background_color: [u8; 3],
}

impl Default for QRApp {
    fn default() -> Self {
        Self {
            mode: Mode::Url,
            url_input: String::new(),
            text_input: String::new(),
            contact: ContactData::default(),
            qr_texture: None,
            generator: QRGenerator::new(),
            ec_level: EcLevel::H,
            foreground_color: [0, 0, 0],
            background_color: [255, 255, 255],
        }
    }
}

impl App for QRApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QR.RS - QR Code Generator");

            // Tab selection
            ui.horizontal(|ui| {
                if ui
                    .selectable_value(&mut self.mode, Mode::Url, "URL")
                    .clicked()
                {
                    self.update_qr(ctx);
                }
                if ui
                    .selectable_value(&mut self.mode, Mode::Text, "Text")
                    .clicked()
                {
                    self.update_qr(ctx);
                }
                if ui
                    .selectable_value(&mut self.mode, Mode::Contact, "Contact")
                    .clicked()
                {
                    self.update_qr(ctx);
                }
            });

            ui.separator();

            let mut changed = false;

            // Input area based on mode
            match self.mode {
                Mode::Url => {
                    ui.label("Enter URL:");
                    if ui.text_edit_singleline(&mut self.url_input).changed() {
                        changed = true;
                    }
                }
                Mode::Text => {
                    ui.label("Enter Text:");
                    if ui.text_edit_multiline(&mut self.text_input).changed() {
                        changed = true;
                    }
                }
                Mode::Contact => {
                    ui.label("Contact Info:");
                    if ui
                        .text_edit_singleline(&mut self.contact.first_name)
                        .changed()
                    {
                        changed = true;
                    }
                    if ui
                        .text_edit_singleline(&mut self.contact.last_name)
                        .changed()
                    {
                        changed = true;
                    }
                    if ui.text_edit_singleline(&mut self.contact.email).changed() {
                        changed = true;
                    }
                    if ui.text_edit_singleline(&mut self.contact.phone).changed() {
                        changed = true;
                    }
                    if ui
                        .text_edit_singleline(&mut self.contact.organization)
                        .changed()
                    {
                        changed = true;
                    }
                    if ui.text_edit_singleline(&mut self.contact.website).changed() {
                        changed = true;
                    }
                }
            }

            ui.separator();
            ui.heading("Customization");

            ui.horizontal(|ui| {
                ui.label("Error Correction:");
                egui::ComboBox::from_id_source("ec_level")
                    .selected_text(format!("{:?}", self.ec_level))
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(&mut self.ec_level, EcLevel::L, "L (Low)").clicked() { changed = true; }
                        if ui.selectable_value(&mut self.ec_level, EcLevel::M, "M (Medium)").clicked() { changed = true; }
                        if ui.selectable_value(&mut self.ec_level, EcLevel::Q, "Q (Quartile)").clicked() { changed = true; }
                        if ui.selectable_value(&mut self.ec_level, EcLevel::H, "H (High)").clicked() { changed = true; }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Foreground:");
                if ui.color_edit_button_srgb(&mut self.foreground_color).changed() {
                    changed = true;
                }
                ui.label("Background:");
                if ui.color_edit_button_srgb(&mut self.background_color).changed() {
                    changed = true;
                }
            });

            if changed {
                self.update_qr(ctx);
            }

            ui.separator();

            // QR code preview
            if let Some(texture) = &self.qr_texture {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });
    }
}

impl QRApp {
    fn update_qr(&mut self, ctx: &egui::Context) {
        let data = match self.mode {
            Mode::Url => QRData::URL(self.url_input.clone()),
            Mode::Text => QRData::Text(self.text_input.clone()),
            Mode::Contact => QRData::Contact(self.contact.clone()),
        };

        // Configure generator
        let fg = [self.foreground_color[0], self.foreground_color[1], self.foreground_color[2], 255];
        let bg = [self.background_color[0], self.background_color[1], self.background_color[2], 255];

        self.generator = QRGenerator::new()
            .with_error_correction(self.ec_level)
            .with_colors(fg, bg);

        match self.generator.to_image(&self.generator.generate(&data).unwrap_or(qr_rs::qrcode::QrCode::new(b"").unwrap()), 200, None) {
            Ok(image) => {
                let size = [image.width() as usize, image.height() as usize];
                let pixels = image.to_rgba8().into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                self.qr_texture =
                    Some(ctx.load_texture("qr-code", color_image, egui::TextureOptions::default()));
            }
            Err(_) => {
                self.qr_texture = None;
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("QR.RS", options, Box::new(|_cc| Box::new(QRApp::default())))
}
