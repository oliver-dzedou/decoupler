use clap::Parser;
mod app;
mod markdown;
use crate::app::find_coupling;

#[derive(Parser)]
struct Cli {
    #[arg(
        long = "path",
        short = 'p',
        default_value = ".",
        help = "Path to a git repository to analyse."
    )]
    path: std::path::PathBuf,
    #[arg(
        long = "minimum-frequency",
        short = 'f',
        default_value_t = 5,
        help = "Sets the minimum frequency threshold. Any sets of commits occurring less than this threshold will be ignored."
    )]
    minimum_frequency: u32,
    #[arg(
        long = "minimum-size",
        short = 's',
        default_value_t = 5,
        help = "Sets the minimum set size threshold. Any sets of commits smaller than this threshold will be ignored."
    )]
    minimum_size: u32,
    #[arg(long = "ignore", short='i', default_values_t = ["git".to_string()] , num_args = 1.., help = 
        "Files to ignore. All files containing the specified paths will be ignored.
        \nExample: \"decoupler --ignore git main.rs .ts\" will ignore \".gitignore\", \".git/\", \"main.rs\" and all \".ts\" files")]
    ignore: Vec<String>,
}

/// Entrypoint to the application
fn main() {
    let args = Cli::parse();
    find_coupling(
        &args.path,
        &args.minimum_frequency,
        &args.minimum_size,
        &args.ignore,
    );
}
