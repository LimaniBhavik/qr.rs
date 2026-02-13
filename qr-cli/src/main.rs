use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use qr_rs::{ContactData, QRData, QRGenerator};
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

#[derive(Subcommand)]
enum Commands {
    Url {
        url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Text {
        text: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
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
    },
    Interactive,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Url { url, output }) => {
            generate(QRData::URL(url), output);
        }
        Some(Commands::Text { text, output }) => {
            generate(QRData::Text(text), output);
        }
        Some(Commands::Contact {
            first_name,
            last_name,
            email,
            phone,
            organization,
            website,
            output,
        }) => {
            let contact = ContactData {
                first_name: first_name.unwrap_or_default(),
                last_name: last_name.unwrap_or_default(),
                email: email.unwrap_or_default(),
                phone: phone.unwrap_or_default(),
                organization: organization.unwrap_or_default(),
                website: website.unwrap_or_default(),
            };
            generate(QRData::Contact(contact), output);
        }
        Some(Commands::Interactive) | None => {
            run_interactive();
        }
    }
}

fn generate(data: QRData, output: Option<PathBuf>) {
    let generator = QRGenerator::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
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
                    match generator.to_png(&qr, 300) {
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
            generate(QRData::URL(url), path);
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
            generate(QRData::Text(text), path);
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
            generate(QRData::Contact(contact), path);
        }
        _ => {}
    }
}
