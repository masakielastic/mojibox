# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**mojibox** is a CLI tool for flexible Unicode string manipulation and analysis. It supports processing text at three different levels:
- Grapheme clusters (書記素クラスター) - default mode
- Unicode code points (char mode) 
- Byte level (byte mode)

The tool is designed for handling diverse Japanese and multilingual text use cases.

## Core Commands

### Subcommands
- `iter`: Expand strings one unit at a time by specified mode
- `len`: Count string length by specified mode  
- `take`: Extract N units from the beginning
- `drop`: Skip N units from the beginning and extract the rest

### Options
- `--mode`, `-m`: `grapheme` (default), `char`, `byte`
- `--engine`, `-e`: `unicode` (unicode-segmentation), `icu4x` (icu_segmenter)

## Development Commands

Since this is a Rust project, use standard Cargo commands:

```sh
# Build the project
cargo build

# Run tests
cargo test

# Run with arguments
cargo run -- <subcommand> <args>

# Build optimized release
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Key Dependencies (as specified)

- **unicode-segmentation**: For grapheme cluster, word, and sentence segmentation (UAX #29)
- **icu_segmenter (icu4x)**: For Unicode standard compliance and accurate multilingual segmentation
- **clap**: For command-line argument parsing and subcommand management
- **anyhow**: For simple error handling
- **thiserror**: For custom error implementations
- **encoding_rs**: For character encoding conversion when needed

## Architecture Notes

The tool processes text input through different segmentation modes:
1. **Grapheme mode**: Uses Unicode grapheme cluster boundaries (handles emoji, combining characters correctly)
2. **Char mode**: Processes by Unicode code points
3. **Byte mode**: Processes by individual bytes

Engine selection allows switching between unicode-segmentation and icu4x implementations for different accuracy/performance trade-offs.

## Development Considerations

- Handle UTF-8 incompatible systems/terminals gracefully
- Be aware of segmentation differences between unicode-segmentation and icu4x
- Consider performance and binary size implications, especially with ICU4X
- Test with complex Unicode text including Japanese, emoji, and combining characters

## Commit Guidelines

Follow **Conventional Commits** format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat: add iter subcommand for grapheme mode`
- `fix: handle empty string input in len command`
- `docs: update usage examples in spec.md`