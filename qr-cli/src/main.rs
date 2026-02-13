use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use qr_rs::{QRGenerator, QRData, ContactData};
use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use image::ImageReader;

#[derive(Parser)]
#[command(name = "qr-cli")]
#[command(about = "QR Code Generator CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EcLevel {
    L,
    M,
    Q,
    H,
}

impl From<EcLevel> for qr_rs::qrcode::EcLevel {
    fn from(level: EcLevel) -> Self {
        match level {
            EcLevel::L => qr_rs::qrcode::EcLevel::L,
            EcLevel::M => qr_rs::qrcode::EcLevel::M,
            EcLevel::Q => qr_rs::qrcode::EcLevel::Q,
            EcLevel::H => qr_rs::qrcode::EcLevel::H,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Url {
        url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "H")]
        ec_level: EcLevel,
        #[arg(long)]
        foreground: Option<String>, // Hex color
        #[arg(long)]
        background: Option<String>, // Hex color
        #[arg(long)]
        logo: Option<PathBuf>,
    },
    Text {
        text: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "H")]
        ec_level: EcLevel,
        #[arg(long)]
        foreground: Option<String>,
        #[arg(long)]
        background: Option<String>,
        #[arg(long)]
        logo: Option<PathBuf>,
    },
    Contact {
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        organization: Option<String>,
        #[arg(long)]
        website: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "H")]
        ec_level: EcLevel,
        #[arg(long)]
        foreground: Option<String>,
        #[arg(long)]
        background: Option<String>,
        #[arg(long)]
        logo: Option<PathBuf>,
    },
    Interactive,
}

fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some([r, g, b, 255])
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some([r, g, b, a])
    } else {
        None
    }
}

fn configure_generator(
    ec_level: EcLevel,
    foreground: Option<String>,
    background: Option<String>,
) -> QRGenerator {
    let mut generator = QRGenerator::new().with_error_correction(ec_level.into());

    if let Some(fg) = foreground {
        if let Some(color) = parse_hex_color(&fg) {
            // We need access to setting colors directly or via a method.
            // Since we added with_colors method which takes [u8; 4] for both,
            // we should probably check if background is also set, or default.
            let bg = if let Some(bg_str) = background.clone() {
                 parse_hex_color(&bg_str).unwrap_or([255, 255, 255, 255])
            } else {
                 [255, 255, 255, 255]
            };
            generator = generator.with_colors(color, bg);
        }
    } else if let Some(bg) = background {
        // Foreground default black
        if let Some(color) = parse_hex_color(&bg) {
             generator = generator.with_colors([0, 0, 0, 255], color);
        }
    }

    generator
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Url { url, output, ec_level, foreground, background, logo }) => {
            let generator = configure_generator(ec_level, foreground, background);
            generate(generator, QRData::URL(url), output, logo);
        }
        Some(Commands::Text { text, output, ec_level, foreground, background, logo }) => {
            let generator = configure_generator(ec_level, foreground, background);
            generate(generator, QRData::Text(text), output, logo);
        }
        Some(Commands::Contact { first_name, last_name, email, phone, organization, website, output, ec_level, foreground, background, logo }) => {
             let contact = ContactData {
                first_name: first_name.unwrap_or_default(),
                last_name: last_name.unwrap_or_default(),
                email: email.unwrap_or_default(),
                phone: phone.unwrap_or_default(),
                organization: organization.unwrap_or_default(),
                website: website.unwrap_or_default(),
            };
            let generator = configure_generator(ec_level, foreground, background);
            generate(generator, QRData::Contact(contact), output, logo);
        }
        Some(Commands::Interactive) | None => {
            run_interactive();
        }
    }
}

fn generate(generator: QRGenerator, data: QRData, output: Option<PathBuf>, logo_path: Option<PathBuf>) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
    pb.set_message("Generating QR Code...");
    pb.enable_steady_tick(Duration::from_millis(100));

    match generator.generate(&data) {
        Ok(qr) => {
            pb.finish_with_message("Generated!");

            if let Some(path) = output {
                // Determine format based on extension or default to PNG
                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

                if extension.eq_ignore_ascii_case("svg") {
                    let svg = generator.to_svg(&qr);
                    if let Err(e) = fs::write(&path, svg) {
                         eprintln!("{} {}", "Error saving SVG:".red(), e);
                    } else {
                         println!("{} {}", "Saved to".green(), path.display());
                    }
                } else {
                    // Default PNG
                    // Load logo if provided
                    let logo_img = if let Some(l_path) = logo_path {
                         match ImageReader::open(&l_path).map_err(|e| e.to_string()).and_then(|r| r.decode().map_err(|e| e.to_string())) {
                             Ok(img) => Some(img),
                             Err(e) => {
                                 eprintln!("{} {}", "Warning: Failed to load logo:".yellow(), e);
                                 None
                             }
                         }
                    } else {
                        None
                    };

                    match generator.to_png(&qr, 300, logo_img.as_ref()) {
                        Ok(bytes) => {
                             if let Err(e) = fs::write(&path, bytes) {
                                 eprintln!("{} {}", "Error saving PNG:".red(), e);
                             } else {
                                 println!("{} {}", "Saved to".green(), path.display());
                             }
                        }
                        Err(e) => eprintln!("{} {}", "Error encoding PNG:".red(), e),
                    }
                }
            } else {
                // Terminal output
                let string = qr.render::<qr_rs::qrcode::render::unicode::Dense1x2>()
                    .dark_color(qr_rs::qrcode::render::unicode::Dense1x2::Light)
                    .light_color(qr_rs::qrcode::render::unicode::Dense1x2::Dark)
                    .build();
                println!("\n{}", string);
            }
        }
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("{} {}", "Error:".red(), e);
        }
    }
}

fn run_interactive() {
    let selections = &["URL", "Text", "Contact"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select QR Code Type")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let generator = QRGenerator::new(); // Default generator for interactive mode for now

    match selection {
        0 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter URL")
                .interact_text()
                .unwrap();
            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional, leave empty for terminal)")
                .allow_empty(true)
                .interact_text()
                .unwrap();

            let path = if output.is_empty() { None } else { Some(PathBuf::from(output)) };
            generate(generator, QRData::URL(url), path, None);
        }
        1 => {
            let text: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Text")
                .interact_text()
                .unwrap();
            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional)")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let path = if output.is_empty() { None } else { Some(PathBuf::from(output)) };
            generate(generator, QRData::Text(text), path, None);
        }
        2 => {
            let first_name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("First Name").allow_empty(true).interact_text().unwrap();
            let last_name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Last Name").allow_empty(true).interact_text().unwrap();
            let email: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Email").allow_empty(true).interact_text().unwrap();
            let phone: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Phone").allow_empty(true).interact_text().unwrap();
            let organization: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Organization").allow_empty(true).interact_text().unwrap();
            let website: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Website").allow_empty(true).interact_text().unwrap();

            let contact = ContactData {
                first_name, last_name, email, phone, organization, website
            };

            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional)")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let path = if output.is_empty() { None } else { Some(PathBuf::from(output)) };
            generate(generator, QRData::Contact(contact), path, None);
        }
        _ => {}
    }
}
