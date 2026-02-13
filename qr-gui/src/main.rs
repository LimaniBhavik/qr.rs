use eframe::{egui, App, Frame};
use qr_rs::image::Luma;
use qr_rs::{ContactData, QRData, QRGenerator};

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

        match self.generator.generate(&data) {
            Ok(qr) => {
                let image = qr.render::<Luma<u8>>().min_dimensions(200, 200).build();

                let size = [image.width() as usize, image.height() as usize];
                let pixels = image.into_raw();

                let color_image = egui::ColorImage::from_gray(size, &pixels);

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
