use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use mojibox::{iter_byte, iter_codepoint, iter_grapheme_icu4x, count_units, take_units, drop_units, ProcessingMode as LibProcessingMode, dump_graphemes, DumpFormat, ord_characters, chr_from_codepoints};

#[derive(Parser)]
#[command(name = "mojibox")]
#[command(about = "A CLI tool for flexible Unicode string manipulation and analysis")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Expand strings one unit at a time by specified mode
    Iter {
        /// Processing mode
        #[arg(short, long, default_value = "grapheme")]
        mode: ProcessingMode,

        /// Segmentation engine
        #[arg(short, long, default_value = "icu4x")]
        engine: Engine,

        /// Input string to process
        input: String,
    },
    /// Count string length by specified mode
    Len {
        /// Processing mode
        #[arg(short, long, default_value = "grapheme")]
        mode: ProcessingMode,

        /// Segmentation engine
        #[arg(short, long, default_value = "icu4x")]
        engine: Engine,

        /// Input string to process
        input: String,
    },
    /// Extract N units from the beginning
    Take {
        /// Processing mode
        #[arg(short, long, default_value = "grapheme")]
        mode: ProcessingMode,

        /// Segmentation engine
        #[arg(short, long, default_value = "icu4x")]
        engine: Engine,

        /// Number of units to take
        n: usize,

        /// Input string to process
        input: String,
    },
    /// Skip N units from the beginning and extract the rest
    Drop {
        /// Processing mode
        #[arg(short, long, default_value = "grapheme")]
        mode: ProcessingMode,

        /// Segmentation engine
        #[arg(short, long, default_value = "icu4x")]
        engine: Engine,

        /// Number of units to drop
        n: usize,

        /// Input string to process
        input: String,
    },
    /// Dump detailed information about grapheme clusters and their codepoints
    Dump {
        /// Output format
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,

        /// Input string to process
        input: String,
    },
    /// Convert characters to Unicode codepoints
    Ord {
        /// Use lowercase hex format
        #[arg(long)]
        lower: bool,
        
        /// Output without 0x prefix
        #[arg(long)]
        no_0x: bool,
        
        /// Input string to process
        input: String,
    },
    /// Convert Unicode codepoints to characters  
    Chr {
        /// Unicode codepoints in hex format (with or without 0x prefix)
        codepoints: Vec<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ProcessingMode {
    /// Grapheme clusters (default)
    Grapheme,
    /// Unicode code points
    Codepoint,
    /// Bytes
    Byte,
}

#[derive(ValueEnum, Clone, Debug)]
enum Engine {
    /// ICU4X segmentation engine
    Icu4x,
    /// Unicode segmentation engine
    Unicode,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Human-readable text format
    Text,
    /// JSON format
    Json,
    /// JSON Lines format
    Jsonl,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Iter {
            mode,
            engine,
            input,
        } => {
            handle_iter(mode, engine, input)?;
        }
        Commands::Len {
            mode,
            engine,
            input,
        } => {
            handle_len(mode, engine, input)?;
        }
        Commands::Take {
            mode,
            engine,
            n,
            input,
        } => {
            handle_take(mode, engine, n, input)?;
        }
        Commands::Drop {
            mode,
            engine,
            n,
            input,
        } => {
            handle_drop(mode, engine, n, input)?;
        }
        Commands::Dump { format, input } => {
            handle_dump(format, input)?;
        }
        Commands::Ord { lower, no_0x, input } => {
            handle_ord(lower, no_0x, input)?;
        }
        Commands::Chr { codepoints } => {
            handle_chr(codepoints)?;
        }
    }

    Ok(())
}

fn convert_mode(mode: ProcessingMode) -> LibProcessingMode {
    match mode {
        ProcessingMode::Grapheme => LibProcessingMode::Grapheme,
        ProcessingMode::Codepoint => LibProcessingMode::Codepoint,
        ProcessingMode::Byte => LibProcessingMode::Byte,
    }
}

fn convert_format(format: OutputFormat) -> DumpFormat {
    match format {
        OutputFormat::Text => DumpFormat::Text,
        OutputFormat::Json => DumpFormat::Json,
        OutputFormat::Jsonl => DumpFormat::Jsonl,
    }
}

fn handle_iter(mode: ProcessingMode, engine: Engine, input: String) -> Result<()> {
    match mode {
        ProcessingMode::Grapheme => match engine {
            Engine::Icu4x => {
                let segments = iter_grapheme_icu4x(&input)?;
                for segment in segments {
                    println!("{}", segment);
                }
            }
            Engine::Unicode => {
                eprintln!("Warning: Unicode engine not implemented yet, falling back to ICU4X");
                let segments = iter_grapheme_icu4x(&input)?;
                for segment in segments {
                    println!("{}", segment);
                }
            }
        },
        ProcessingMode::Codepoint => {
            let segments = iter_codepoint(&input);
            for segment in segments {
                println!("{}", segment);
            }
        }
        ProcessingMode::Byte => {
            let segments = iter_byte(&input);
            for segment in segments {
                println!("{}", segment);
            }
        }
    }
    Ok(())
}

fn handle_len(mode: ProcessingMode, _engine: Engine, input: String) -> Result<()> {
    let lib_mode = convert_mode(mode);
    let count = count_units(&input, lib_mode)?;
    println!("{}", count);
    Ok(())
}

fn handle_take(mode: ProcessingMode, _engine: Engine, n: usize, input: String) -> Result<()> {
    let lib_mode = convert_mode(mode);
    let segments = take_units(&input, lib_mode, n)?;
    for segment in segments {
        println!("{}", segment);
    }
    Ok(())
}

fn handle_drop(mode: ProcessingMode, _engine: Engine, n: usize, input: String) -> Result<()> {
    let lib_mode = convert_mode(mode);
    let segments = drop_units(&input, lib_mode, n)?;
    for segment in segments {
        println!("{}", segment);
    }
    Ok(())
}

fn handle_dump(format: OutputFormat, input: String) -> Result<()> {
    let dump_format = convert_format(format);
    let output = dump_graphemes(&input, dump_format)?;
    print!("{}", output);
    Ok(())
}

fn handle_ord(lower: bool, no_0x: bool, input: String) -> Result<()> {
    let codepoints = ord_characters(&input, lower, no_0x);
    println!("{}", codepoints.join(" "));
    Ok(())
}

fn handle_chr(codepoints: Vec<String>) -> Result<()> {
    let result = chr_from_codepoints(&codepoints)?;
    println!("{}", result);
    Ok(())
}
