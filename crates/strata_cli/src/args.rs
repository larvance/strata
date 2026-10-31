//! Zero-dependency CLI argument parser, command models, and help printer.

use std::path::PathBuf;

/// Compilation and runtime platform target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildTarget {
    #[default]
    Desktop,
    Android,
    Ios,
    MacOs,
    Windows,
    Linux,
    WebWasm,
    WebJs,
}

impl BuildTarget {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "desktop" => Ok(Self::Desktop),
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::Ios),
            "macos" => Ok(Self::MacOs),
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            "web-wasm" | "wasm" => Ok(Self::WebWasm),
            "web-js" | "js" => Ok(Self::WebJs),
            unknown => Err(format!(
                "Invalid target '{}'. Valid options: desktop, android, ios, macos, windows, linux, web-wasm, web-js",
                unknown
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Android => "android",
            Self::Ios => "ios",
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::WebWasm => "web-wasm",
            Self::WebJs => "web-js",
        }
    }
}

/// Project template options for `strata new`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TemplateKind {
    #[default]
    Blank,
    Saas,
    Ecommerce,
    Studio,
}

impl TemplateKind {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "blank" => Ok(Self::Blank),
            "saas" => Ok(Self::Saas),
            "ecommerce" => Ok(Self::Ecommerce),
            "studio" => Ok(Self::Studio),
            unknown => Err(format!(
                "Invalid template '{}'. Valid options: blank, saas, ecommerce, studio",
                unknown
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blank => "blank",
            Self::Saas => "saas",
            Self::Ecommerce => "ecommerce",
            Self::Studio => "studio",
        }
    }
}

/// Native installer and store packaging format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageFormat {
    Apk,
    Aab,
    Ipa,
    Dmg,
    App,
    Msi,
    Exe,
    AppImage,
    Deb,
    TarGz,
}

impl PackageFormat {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "apk" => Ok(Self::Apk),
            "aab" => Ok(Self::Aab),
            "ipa" => Ok(Self::Ipa),
            "dmg" => Ok(Self::Dmg),
            "app" => Ok(Self::App),
            "msi" => Ok(Self::Msi),
            "exe" => Ok(Self::Exe),
            "appimage" => Ok(Self::AppImage),
            "deb" => Ok(Self::Deb),
            "tar.gz" | "targz" | "tar" => Ok(Self::TarGz),
            unknown => Err(format!(
                "Invalid packaging format '{}'. Valid options: apk, aab, ipa, dmg, app, msi, exe, appimage, deb, tar.gz",
                unknown
            )),
        }
    }

    pub fn default_for_target(target: BuildTarget) -> Self {
        match target {
            BuildTarget::Android => Self::Aab,
            BuildTarget::Ios => Self::Ipa,
            BuildTarget::MacOs => Self::Dmg,
            BuildTarget::Windows => Self::Msi,
            BuildTarget::Linux => Self::AppImage,
            BuildTarget::Desktop => {
                #[cfg(target_os = "macos")]
                return Self::Dmg;
                #[cfg(target_os = "windows")]
                return Self::Msi;
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                return Self::AppImage;
            }
            BuildTarget::WebWasm | BuildTarget::WebJs => Self::TarGz,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Apk => "apk",
            Self::Aab => "aab",
            Self::Ipa => "ipa",
            Self::Dmg => "dmg",
            Self::App => "app",
            Self::Msi => "msi",
            Self::Exe => "exe",
            Self::AppImage => "AppImage",
            Self::Deb => "deb",
            Self::TarGz => "tar.gz",
        }
    }
}

/// Parsed CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    New {
        name: String,
        template: TemplateKind,
        target_dir: Option<PathBuf>,
    },
    Dev {
        target: BuildTarget,
        port: u16,
        watch: bool,
    },
    Build {
        target: BuildTarget,
        release: bool,
        out_dir: Option<PathBuf>,
    },
    Package {
        target: BuildTarget,
        format: Option<PackageFormat>,
        out_dir: Option<PathBuf>,
    },
    Test {
        parity: bool,
        verbose: bool,
    },
    Help,
    Version,
}

/// CLI Argument Parser.
pub struct CliArgs;

impl CliArgs {
    pub fn print_banner() {
        println!("\x1b[1;38;5;99m⚡ Strata Universal Frontend Engine & Toolchain v0.1.0\x1b[0m");
        println!("\x1b[90m   Zero Chromium. Pure Rust. Pixel-Perfect CSS & Flexbox/Grid.\x1b[0m\n");
    }

    pub fn print_help() {
        Self::print_banner();
        println!("Usage: strata <COMMAND> [OPTIONS]\n");
        println!("Commands:");
        println!("  new <name> [--template <tpl>]   Scaffold a new Strata project (blank, saas, ecommerce, studio)");
        println!("  dev [--target <target>]         Start local dev server with state-preserving HMR");
        println!("  build [--target <target>]       Compile optimized binary (desktop, android, ios, macos, windows, linux, web-wasm, web-js)");
        println!("  package [--format <fmt>]        Package into native installer (apk, aab, ipa, dmg, msi, appimage, tar.gz)");
        println!("  test [--parity chromium]        Run automated Chromium pixel-parity visual diff suite");
        println!("  help                            Print this help message");
        println!("  version                         Print version information\n");
        println!("Targets:");
        println!("  desktop (default), android, ios, macos, windows, linux, web-wasm, web-js\n");
    }

    pub fn parse() -> Result<CliCommand, String> {
        Self::parse_from(std::env::args())
    }

    pub fn parse_from<I, T>(iter: I) -> Result<CliCommand, String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let raw_args: Vec<String> = iter.into_iter().map(|s| s.as_ref().to_string()).collect();
        if raw_args.len() < 2 {
            return Ok(CliCommand::Help);
        }

        let subcommand = &raw_args[1];
        let sub_args = &raw_args[2..];

        match subcommand.as_str() {
            "new" => Self::parse_new(sub_args),
            "dev" => Self::parse_dev(sub_args),
            "build" => Self::parse_build(sub_args),
            "package" => Self::parse_package(sub_args),
            "test" => Self::parse_test(sub_args),
            "help" | "--help" | "-h" => Ok(CliCommand::Help),
            "version" | "--version" | "-v" => Ok(CliCommand::Version),
            unknown => Err(format!(
                "Unknown command: '{}'. Run 'strata help' for available commands.",
                unknown
            )),
        }
    }

    fn parse_new(args: &[String]) -> Result<CliCommand, String> {
        let mut name = String::new();
        let mut template = TemplateKind::default();
        let mut target_dir = None;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--template" || arg == "-t" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected template name after '--template'".to_string());
                }
                template = TemplateKind::parse(&args[i])?;
            } else if arg == "--dir" || arg == "-d" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected path after '--dir'".to_string());
                }
                target_dir = Some(PathBuf::from(&args[i]));
            } else if !arg.starts_with('-') && name.is_empty() {
                name = arg.clone();
            }
            i += 1;
        }

        if name.is_empty() {
            name = "my-strata-app".to_string();
        }

        Ok(CliCommand::New {
            name,
            template,
            target_dir,
        })
    }

    fn parse_dev(args: &[String]) -> Result<CliCommand, String> {
        let mut target = BuildTarget::Desktop;
        let mut port = 3000;
        let mut watch = true;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--target" || arg == "-t" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected target after '--target'".to_string());
                }
                target = BuildTarget::parse(&args[i])?;
            } else if arg == "--port" || arg == "-p" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected port after '--port'".to_string());
                }
                port = args[i].parse::<u16>().map_err(|_| "Invalid port number")?;
            } else if arg == "--no-watch" {
                watch = false;
            }
            i += 1;
        }

        Ok(CliCommand::Dev {
            target,
            port,
            watch,
        })
    }

    fn parse_build(args: &[String]) -> Result<CliCommand, String> {
        let mut target = BuildTarget::Desktop;
        let mut release = false;
        let mut out_dir = None;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--target" || arg == "-t" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected target after '--target'".to_string());
                }
                target = BuildTarget::parse(&args[i])?;
            } else if arg == "--release" || arg == "-r" {
                release = true;
            } else if arg == "--out-dir" || arg == "-o" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected output directory after '--out-dir'".to_string());
                }
                out_dir = Some(PathBuf::from(&args[i]));
            }
            i += 1;
        }

        Ok(CliCommand::Build {
            target,
            release,
            out_dir,
        })
    }

    fn parse_package(args: &[String]) -> Result<CliCommand, String> {
        let mut target = BuildTarget::Desktop;
        let mut format = None;
        let mut out_dir = None;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--target" || arg == "-t" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected target after '--target'".to_string());
                }
                target = BuildTarget::parse(&args[i])?;
            } else if arg == "--format" || arg == "-f" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected format after '--format'".to_string());
                }
                format = Some(PackageFormat::parse(&args[i])?);
            } else if arg == "--out-dir" || arg == "-o" {
                i += 1;
                if i >= args.len() {
                    return Err("Expected output directory after '--out-dir'".to_string());
                }
                out_dir = Some(PathBuf::from(&args[i]));
            }
            i += 1;
        }

        Ok(CliCommand::Package {
            target,
            format,
            out_dir,
        })
    }

    fn parse_test(args: &[String]) -> Result<CliCommand, String> {
        let mut parity = false;
        let mut verbose = false;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--parity" {
                parity = true;
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    i += 1; // optional value e.g. "chromium"
                }
            } else if arg == "--verbose" || arg == "-v" {
                verbose = true;
            }
            i += 1;
        }

        Ok(CliCommand::Test { parity, verbose })
    }
}
