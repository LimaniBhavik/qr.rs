use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use image::ImageReader;
use indicatif::{ProgressBar, ProgressStyle};
use qr_rs::utils::parse_hex_color;
use qr_rs::{ContactData, QRBuilder, QRData};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

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
        foreground: Option<String>,
        #[arg(long)]
        background: Option<String>,
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

fn configure_builder(
    ec_level: EcLevel,
    foreground: Option<String>,
    background: Option<String>,
) -> QRBuilder {
    let mut builder = QRBuilder::new().error_correction(ec_level.into());

    if let Some(fg) = foreground {
        if let Some(color) = parse_hex_color(&fg) {
            let bg = if let Some(bg_str) = background.clone() {
                parse_hex_color(&bg_str).unwrap_or([255, 255, 255, 255])
            } else {
                [255, 255, 255, 255]
            };
            builder = builder.colors(color, bg);
        }
    } else if let Some(bg) = background {
        if let Some(color) = parse_hex_color(&bg) {
            builder = builder.colors([0, 0, 0, 255], color);
        }
    }

    builder
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Url {
            url,
            output,
            ec_level,
            foreground,
            background,
            logo,
        }) => {
            let builder = configure_builder(ec_level, foreground, background).url(url);
            generate(builder, output, logo);
        }
        Some(Commands::Text {
            text,
            output,
            ec_level,
            foreground,
            background,
            logo,
        }) => {
            let builder = configure_builder(ec_level, foreground, background).text(text);
            generate(builder, output, logo);
        }
        Some(Commands::Contact {
            first_name,
            last_name,
            email,
            phone,
            organization,
            website,
            output,
            ec_level,
            foreground,
            background,
            logo,
        }) => {
            let contact = ContactData {
                first_name: first_name.unwrap_or_default(),
                last_name: last_name.unwrap_or_default(),
                email: email.unwrap_or_default(),
                phone: phone.unwrap_or_default(),
                organization: organization.unwrap_or_default(),
                website: website.unwrap_or_default(),
            };
            let builder =
                configure_builder(ec_level, foreground, background).data(QRData::Contact(contact));
            generate(builder, output, logo);
        }
        Some(Commands::Interactive) | None => {
            run_interactive();
        }
    }
}

fn generate(builder: QRBuilder, output: Option<PathBuf>, logo_path: Option<PathBuf>) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Generating QR Code...");
    pb.enable_steady_tick(Duration::from_millis(100));

    match builder.build() {
        Ok(generator) => match generator.generate() {
            Ok(qr) => {
                pb.finish_with_message("Generated!");

                if let Some(path) = output {
                    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

                    if extension.eq_ignore_ascii_case("svg") {
                        match generator.to_svg() {
                            Ok(svg) => {
                                if let Err(e) = fs::write(&path, svg) {
                                    eprintln!("{} {}", "Error saving SVG:".red(), e);
                                } else {
                                    println!("{} {}", "Saved to".green(), path.display());
                                }
                            }
                            Err(e) => eprintln!("{} {}", "Error generating SVG:".red(), e),
                        }
                    } else {
                        let logo_img = if let Some(l_path) = logo_path {
                            match ImageReader::open(&l_path)
                                .map_err(|e| e.to_string())
                                .and_then(|r| r.decode().map_err(|e| e.to_string()))
                            {
                                Ok(img) => Some(img),
                                Err(e) => {
                                    eprintln!("{} {}", "Warning: Failed to load logo:".yellow(), e);
                                    None
                                }
                            }
                        } else {
                            None
                        };

                        match generator.to_png(300, logo_img.as_ref()) {
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
                    let string = qr
                        .render::<qr_rs::qrcode::render::unicode::Dense1x2>()
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
        },
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

    let builder = QRBuilder::new();

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

            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(builder.url(url), path, None);
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
            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(builder.text(text), path, None);
        }
        2 => {
            let first_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("First Name")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let last_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Last Name")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let email: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Email")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let phone: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Phone")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let organization: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Organization")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let website: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Website")
                .allow_empty(true)
                .interact_text()
                .unwrap();

            let contact = ContactData {
                first_name,
                last_name,
                email,
                phone,
                organization,
                website,
            };

            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional)")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(builder.data(QRData::Contact(contact)), path, None);
        }
        _ => {}
    }
}
