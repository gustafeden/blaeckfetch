mod cache;
mod color;
mod info;
mod logo;
mod render;

use clap::Parser;
use color::Theme;
use info::SystemInfo;

/// A fast system fetch display, written in Rust.
#[derive(Parser)]
#[command(name = "rsfetch", version, about)]
struct Args {
    /// Color theme (green, cyan, red, magenta, yellow, blue, mono)
    #[arg(short, long, default_value = "green")]
    color: String,

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
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.clear_cache {
        cache::clear();
    }

    let info = SystemInfo::gather();

    if args.json {
        println!("{}", info.to_json());
        return Ok(());
    }

    let theme = Theme::by_name(&args.color);

    let logo = if args.no_logo {
        logo::by_name("none")
    } else if let Some(name) = &args.logo {
        logo::by_name(name)
    } else {
        logo::detect()
    };

    render::render(&info, logo.art, &theme)
}
