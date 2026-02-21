use clap::Parser as ClapParser;
use log::info;
use std::path::PathBuf;
use std::process;

mod notebook;
mod renderers;

#[derive(ClapParser, Debug)]
#[command(name = "one2md", version, about = "Convert OneNote notebooks into Markdown")]
pub struct Cli {
    /// Input file (.one or .onetoc2)
    #[arg(short, long)]
    filename: PathBuf,

    /// Target output directory (defaults to current directory)
    #[arg(short, long)]
    destination_directory: Option<PathBuf>,

    /// Output mode
    #[arg(long, default_value = "readme")]
    mode: String,

    /// Dry run — log what would be produced without writing files
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Overwrite existing files and directories
    #[arg(short, long, default_value_t = false)]
    should_overwrite: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let dest = cli
        .destination_directory
        .unwrap_or_else(|| PathBuf::from("."));

    info!("Input: {:?}", cli.filename);
    info!("Output: {:?}", dest);
    info!("Mode: {}", cli.mode);
    info!("Dry run: {}", cli.dry_run);

    let opts = notebook::ConvertOptions {
        dry_run: cli.dry_run,
        should_overwrite: cli.should_overwrite,
    };

    if let Err(e) = notebook::convert(&cli.filename, &dest, &opts) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
