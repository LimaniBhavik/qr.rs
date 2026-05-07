use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use image::ImageReader;
use indicatif::{ProgressBar, ProgressStyle};
use qr_rs::utils::{parse_hex_color, BLACK, WHITE};
use qr_rs::{ContactData, QRBuilder, QRData};
use std::fs;
use std::path::{Component, Path, PathBuf};
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

    /// Foreground color (hex code)
    #[arg(short = 'f', long, default_value = "#000000")]
    fg: String,

    /// Background color (hex code)
    #[arg(short = 'b', long, default_value = "#FFFFFF")]
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
            let bg = if let Some(bg_str) = background {
                parse_hex_color(&bg_str).unwrap_or(WHITE)
            } else {
                WHITE
            };
            builder = builder.colors(color, bg);
        }
    } else if let Some(bg) = background {
        if let Some(color) = parse_hex_color(&bg) {
            builder = builder.colors(BLACK, color);
        }
    }

    builder
}

fn main() {
    let cli = Cli::parse();

    // If input is provided, run in simpler mode
    if let Some(ref input_string) = cli.input {
        if let Err(e) = run_simple_mode(&cli, input_string.clone()) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        return;
    }

    let result = match cli.command {
        Some(Commands::Url {
            url,
            output,
            ec_level,
            foreground,
            background,
            logo,
        }) => {
            let builder = configure_builder(ec_level, foreground, background).url(url);
            generate(
                builder,
                output.as_deref(),
                logo.as_deref(),
                None,
                None,
                cli.force,
            )
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
            generate(
                builder,
                output.as_deref(),
                logo.as_deref(),
                None,
                None,
                cli.force,
            )
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
            generate(
                builder,
                output.as_deref(),
                logo.as_deref(),
                None,
                None,
                cli.force,
            )
        }
        Some(Commands::Interactive) | None => run_interactive(),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run_simple_mode(cli: &Cli, input: String) -> Result<(), String> {
    let fg_color = parse_hex_color(&cli.fg).unwrap_or(BLACK);
    let bg_color = parse_hex_color(&cli.bg).unwrap_or(WHITE);

    let builder = QRBuilder::new()
        .text(input)
        .error_correction(cli.error_correction_level.into())
        .colors(fg_color, bg_color);

    generate(
        builder,
        cli.output.as_deref(),
        None,
        Some(cli.scale),
        Some(cli.border),
        cli.force,
    )
}

fn validate_output_path(path: &Path) -> Result<(), String> {
    if path.is_absolute() {
        return Err("Absolute paths are not allowed for security reasons. Please use a relative path within the current directory.".to_string());
    }

    for component in path.components() {
        if matches!(component, Component::ParentDir) {
            return Err(
                "Path traversal (using '..') is not allowed for security reasons.".to_string(),
            );
        }
    }

    // Resolve the parent directory and ensure it does not escape the current working directory
    // or follow symlinks to arbitrary locations.
    let current_dir = std::env::current_dir()
        .and_then(|d| d.canonicalize())
        .map_err(|e| format!("Could not resolve current directory: {}", e))?;

    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let parent_path = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };

    let resolved_parent = parent_path
        .canonicalize()
        .map_err(|e| format!("Could not resolve parent directory: {}", e))?;

    if !resolved_parent.starts_with(&current_dir) {
        return Err("Output path resolves outside the current working directory.".to_string());
    }

    // Ensure the file itself is not a symlink (if it exists)
    if let Ok(metadata) = std::fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            return Err("Symlinks are not allowed for security reasons.".to_string());
        }
    }

    Ok(())
}

fn generate(
    builder: QRBuilder,
    output: Option<&Path>,
    logo_path: Option<&Path>,
    scale: Option<u32>,
    border: Option<u32>,
    force: bool,
) -> Result<(), String> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    pb.set_message("Generating QR Code...");
    pb.enable_steady_tick(Duration::from_millis(100));

    match builder.build() {
        Ok(generator) => match generator.generate() {
            Ok(qr) => {
                pb.finish_with_message("Generated!");

                if let Some(path) = output {
                    if let Err(e) = validate_output_path(&path) {
                        pb.finish_and_clear();
                        return Err(format!("{} {}", "Security Error:".red(), e));
                    }

                    if path.exists() && !force {
                        pb.finish_and_clear();
                        return Err(format!(
                            "{} File '{}' already exists. Use --force to overwrite.",
                            "Error:".red(),
                            path.display()
                        ));
                    }

                    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

                    if extension.eq_ignore_ascii_case("svg") {
                        match generator.to_svg() {
                            Ok(svg) => {
                                if let Err(e) = validate_output_path(&path) {
                                    return Err(format!("{} {}", "Security Error:".red(), e));
                                }
                                if let Err(e) = fs::write(&path, svg) {
                                    return Err(format!("{} {}", "Error saving SVG:".red(), e));
                                } else {
                                    println!("{} {}", "Saved to".green(), path.display());
                                }
                            }
                            Err(e) => {
                                return Err(format!("{} {}", "Error generating SVG:".red(), e))
                            }
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

                        let size = if let Some(s) = scale {
                            let width_modules = qr.width() as u32;
                            // `qrcode` library uses a fixed quiet zone of 4 modules on each side.
                            // The `border` parameter is currently ignored to maintain consistency
                            // with the underlying library's default rendering behavior.
                            let _ = border;

                            (width_modules + 8) * s
                        } else {
                            300
                        };

                        match generator.to_png(size, logo_img.as_ref()) {
                            Ok(bytes) => {
                                if let Err(e) = fs::write(&path, bytes) {
                                    return Err(format!("{} {}", "Error saving PNG:".red(), e));
                                } else {
                                    println!("{} {}", "Saved to".green(), path.display());
                                }
                            }
                            Err(e) => return Err(format!("{} {}", "Error encoding PNG:".red(), e)),
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
                Ok(())
            }
            Err(e) => {
                pb.finish_and_clear();
                Err(format!("{} {}", "Error:".red(), e))
            }
        },
        Err(e) => {
            pb.finish_and_clear();
            Err(format!("{} {}", "Error:".red(), e))
        }
    }
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
        0 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter URL")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional, leave empty for terminal)")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| e.to_string())?;

            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(builder.url(url), path.as_deref(), None, None, None, false)?;
        }
        1 => {
            let text: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Text")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional)")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| e.to_string())?;
            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(builder.text(text), path.as_deref(), None, None, None, false)?;
        }
        2 => {
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

            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file (optional)")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| e.to_string())?;
            let path = if output.is_empty() {
                None
            } else {
                Some(PathBuf::from(output))
            };
            generate(
                builder.data(QRData::Contact(contact)),
                path.as_deref(),
                None,
                None,
                None,
                false,
            )?;
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod security_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_validate_output_path() {
        let _ = std::fs::create_dir_all("test_safe_dir");
        // Safe paths
        assert!(validate_output_path(Path::new("test.png")).is_ok());
        assert!(validate_output_path(Path::new("test_safe_dir/test.png")).is_ok());
        assert!(validate_output_path(Path::new("./test.png")).is_ok());

        // Unsafe paths - Absolute
        assert!(validate_output_path(Path::new("/tmp/test.png")).is_err());

        // Unsafe paths - Traversal
        assert!(validate_output_path(Path::new("../test.png")).is_err());
        assert!(validate_output_path(Path::new("test_safe_dir/../../test.png")).is_err());
        assert!(validate_output_path(Path::new("test_safe_dir/..")).is_err());

        let _ = std::fs::remove_dir("test_safe_dir");
    }
}
