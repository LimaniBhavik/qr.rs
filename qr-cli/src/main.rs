use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use qr_rs::utils::parse_hex_color;
use qr_rs::{ContactData, QRBuilder, QRData};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "qr-cli")]
#[command(about = "A CLI utility to encode URLs or text into QR codes in various formats and colors.", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    /// String to encode
    #[arg(index = 1)]
    input: Option<String>,

    /// Output file (supported file extensions: jpeg, jpg, png, svg); omit to print QR code to console
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Force output, i.e. overwrite without user confirmation
    #[arg(short = 'F', long)]
    force: bool,

    /// Background color (hex code)
    #[arg(short = 'f', long, default_value = "#000")]
    fg: String,

    /// Foreground color (hex code)
    #[arg(short = 'b', long, default_value = "#FFF")]
    bg: String,

    /// Border size (expressed in unit blocks)
    #[arg(short = 'B', long, default_value = "1")]
    border: u32,

    /// QR error orrection level
    #[arg(short = 'l', long, default_value = "medium")]
    error_correction_level: EcLevel,

    /// Scale factor (raster image output only)
    #[arg(short, long, default_value = "25")]
    scale: u32,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EcLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl From<EcLevel> for qr_rs::qrcode::EcLevel {
    fn from(level: EcLevel) -> Self {
        match level {
            EcLevel::Low => qr_rs::qrcode::EcLevel::L,
            EcLevel::Medium => qr_rs::qrcode::EcLevel::M,
            EcLevel::Quartile => qr_rs::qrcode::EcLevel::Q,
            EcLevel::High => qr_rs::qrcode::EcLevel::H,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Url {
        url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "medium")]
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
        #[arg(long, default_value = "medium")]
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
        #[arg(long, default_value = "medium")]
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

    // If input is provided, run in simpler mode
    if let Some(ref input_string) = cli.input {
        run_simple_mode(&cli, input_string.clone());
        return;
    }

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
            generate(builder, output, logo, None, None);
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
            generate(builder, output, logo, None, None);
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
            generate(builder, output, logo, None, None);
        }
        Some(Commands::Interactive) | None => {
            if let Err(e) = run_interactive() {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        }
    }
}

fn run_simple_mode(cli: &Cli, input: String) {
    let fg_color = parse_hex_color(&cli.fg).unwrap_or([0, 0, 0, 255]);
    let bg_color = parse_hex_color(&cli.bg).unwrap_or([255, 255, 255, 255]);

    let builder = QRBuilder::new()
        .text(input)
        .error_correction(cli.error_correction_level.into())
        .colors(fg_color, bg_color);

    if let Some(path) = &cli.output {
        if path.exists() && !cli.force {
            eprintln!(
                "Error: File '{}' already exists. Use --force to overwrite.",
                path.display()
            );
            std::process::exit(1);
        }
    }

    generate(
        builder,
        cli.output.clone(),
        None,
        Some(cli.scale),
        Some(cli.border),
    );
}

fn generate(
    builder: QRBuilder,
    output: Option<PathBuf>,
    logo_path: Option<PathBuf>,
    scale: Option<u32>,
    border: Option<u32>,
) {
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
                        save_svg(&generator, &path);
                    } else {
                        save_png(&generator, &qr, &path, logo_path, scale, border);
                    }
                } else {
                    print_terminal_qr(&qr);
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


fn save_svg(generator: &qr_rs::generator::QRGenerator, path: &PathBuf) {
    match generator.to_svg() {
        Ok(svg) => {
            if let Err(e) = std::fs::write(path, svg) {
                eprintln!("{} {}", "Error writing SVG:".red(), e);
            }
        }
        Err(e) => eprintln!("{} {}", "Error generating SVG:".red(), e),
    }
}

fn save_png(
    generator: &qr_rs::generator::QRGenerator,
    _qr: &qr_rs::qrcode::QrCode,
    path: &PathBuf,
    logo_path: Option<PathBuf>,
    scale: Option<u32>,
    _border: Option<u32>,
) {
    let size = scale.unwrap_or(25);

    let mut loaded_logo = None;
    if let Some(l_path) = logo_path {
        match image::ImageReader::open(&l_path).and_then(|r| r.decode().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))) {
            Ok(img) => loaded_logo = Some(img),
            Err(e) => eprintln!("{} {}", "Warning: Failed to load logo image:".yellow(), e),
        }
    }

    match generator.to_png(size, loaded_logo.as_ref()) {
        Ok(bytes) => {
            if let Err(e) = std::fs::write(path, bytes) {
                eprintln!("{} {}", "Error writing PNG:".red(), e);
            }
        }
        Err(e) => eprintln!("{} {}", "Error generating PNG:".red(), e),
    }
}

fn print_terminal_qr(qr: &qr_rs::qrcode::QrCode) {
    let string = qr
        .render::<qr_rs::qrcode::render::unicode::Dense1x2>()
        .quiet_zone(false)
        .build();
    println!("{}", string);
}

fn prompt_for_output(prompt: &str) -> Result<Option<PathBuf>, String> {

    let output: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;

    if output.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(output)))
    }
}

fn handle_interactive_url(builder: QRBuilder) -> Result<(), String> {
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter URL")
        .interact_text()
        .map_err(|e| e.to_string())?;

    let path = prompt_for_output("Output file (optional, leave empty for terminal)")?;
    generate(builder.url(url), path, None, None, None);
    Ok(())
}

fn handle_interactive_text(builder: QRBuilder) -> Result<(), String> {
    let text: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Text")
        .interact_text()
        .map_err(|e| e.to_string())?;

    let path = prompt_for_output("Output file (optional)")?;
    generate(builder.text(text), path, None, None, None);
    Ok(())
}

fn handle_interactive_contact(builder: QRBuilder) -> Result<(), String> {
    let first_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("First Name")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;
    let last_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Last Name")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;
    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;
    let phone: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Phone")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;
    let organization: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Organization")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;
    let website: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Website")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;

    let contact = ContactData {
        first_name,
        last_name,
        email,
        phone,
        organization,
        website,
    };

    let path = prompt_for_output("Output file (optional)")?;
    generate(
        builder.data(QRData::Contact(contact)),
        path,
        None,
        None,
        None,
    );
    Ok(())
}

fn run_interactive() -> Result<(), String> {
    let selections = &["URL", "Text", "Contact"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select QR Code Type")
        .default(0)
        .items(&selections[..])
        .interact()
        .map_err(|e| e.to_string())?;

    let builder = QRBuilder::new();

    match selection {
        0 => handle_interactive_url(builder),
        1 => handle_interactive_text(builder),
        2 => handle_interactive_contact(builder),
        _ => Ok(()),
    }
}
