use clap::{Parser, ValueEnum};
use std::{fs, io::{self, Read}};
use datadown::{convert_str, OutputFormat, ParsingMode};

#[derive(Clone, ValueEnum)]
enum Format { 
    Json, 
    Yaml, 
    Toml, 
    Xml,
}
impl From<Format> for OutputFormat {
    fn from(f: Format) -> Self {
        match f {
            Format::Json => OutputFormat::Json,
            Format::Yaml => OutputFormat::Yaml,
            Format::Toml => OutputFormat::Toml,
            Format::Xml  => OutputFormat::Xml,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Markdown → JSON/YAML/TOML/XML")]
struct Args {
    /// Input file (use '-' for stdin)
    input: String,

    /// Output format
    #[arg(short, long, default_value_t = Format::Json, value_enum)]
    format: Format,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    out: Option<String>,

    /// Use minified/config mode
    #[arg(short = 'm', long)]
    minified: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let md = if args.input == "-" {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        s
    } else {
        fs::read_to_string(&args.input)?
    };

    // Select mode based on fl
    let mode = if args.minified {
        ParsingMode::Minified
    } else {
        ParsingMode::Document
    };

    let out = convert_str(&md, args.format.into(), mode)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    if let Some(p) = args.out {
        fs::write(p, out)?;
    } else {
        println!("{out}");
    }
    Ok(())
}
