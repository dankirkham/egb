use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// ROM to load
    pub rom: String,
    /// ROM file to load
    pub rom_file: Option<String>,
    /// Symbol file to load
    #[arg(short, long)]
    pub symbols: Option<String>,
}
