use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use mojibox::{iter_byte, iter_codepoint, iter_grapheme_icu4x};

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
    }

    Ok(())
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
