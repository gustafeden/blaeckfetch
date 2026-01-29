mod cache;
mod color;
mod config;
mod info;
mod logo;
mod render;

use clap::Parser;
use color::Theme;
use config::Config;
use info::SystemInfo;

/// A fast system fetch display, written in Rust.
#[derive(Parser)]
#[command(name = "rsfetch", version, about)]
struct Args {
    /// Color theme (green, cyan, red, magenta, yellow, blue, mono)
    #[arg(short, long)]
    color: Option<String>,

    /// Logo to display (apple, linux, ubuntu, arch, debian, fedora, none)
    #[arg(short, long)]
    logo: Option<String>,

    /// Hide the logo
    #[arg(long)]
    no_logo: bool,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Clear the cache and re-gather all info
    #[arg(long)]
    clear_cache: bool,

    /// Path to config file
    #[arg(long)]
    config: Option<String>,

    /// Print default config to stdout
    #[arg(long)]
    print_config: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.print_config {
        print!("{}", config::generate_default());
        return Ok(());
    }

    if args.clear_cache {
        cache::clear();
    }

    let cfg = Config::load(args.config.as_deref());

    let info = SystemInfo::gather();

    if args.json {
        println!("{}", info.to_json());
        return Ok(());
    }

    // CLI flags override config values
    let color_name = args
        .color
        .or(cfg.color.clone())
        .unwrap_or_else(|| "green".into());

    let mut theme = Theme::by_name(&color_name);

    // Apply custom color overrides from config
    if let Some(colors) = &cfg.colors {
        if let Some(c) = &colors.title {
            theme.title = c.to_color();
        }
        if let Some(c) = &colors.label {
            theme.label = c.to_color();
        }
        if let Some(c) = &colors.separator {
            theme.separator = c.to_color();
        }
        if let Some(c) = &colors.logo {
            theme.logo = c.to_color();
        }
    }

    let logo_name = if args.no_logo {
        Some("none".to_string())
    } else {
        args.logo.or(cfg.logo.clone())
    };

    let logo = match logo_name.as_deref() {
        Some("none" | "off") => logo::by_name("none"),
        Some("auto") | None => logo::detect(),
        Some(name) => logo::by_name(name),
    };

    render::render(&info, logo.art, &theme, &cfg)
}
