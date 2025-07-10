# mojibox

A CLI tool for flexible Unicode string manipulation and analysis.

## Overview

**mojibox** supports processing text at three different levels:
- **Grapheme clusters** (書記素クラスター) - handles emoji, combining characters correctly (default)
- **Unicode code points** - processes by individual Unicode characters
- **Byte level** - processes by individual bytes

The tool is designed for handling diverse Japanese and multilingual text use cases.

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Iterate by grapheme clusters (default)
mojibox iter "あいうえお🍣🍺"

# Iterate by Unicode code points
mojibox iter --mode codepoint "あいうえお🍣🍺"

# Iterate by bytes
mojibox iter --mode byte "hello"

# Analyze Unicode structure of text
mojibox dump "あいう🍣👨‍💻"
```

### Command Options

#### iter command
- `--mode`, `-m`: Processing mode
  - `grapheme` - Grapheme clusters (default)
  - `codepoint` - Unicode code points
  - `byte` - Bytes
- `--engine`, `-e`: Segmentation engine
  - `icu4x` - ICU4X segmentation engine (default)
  - `unicode` - Unicode segmentation engine (not yet implemented)

#### dump command
- `--format`, `-f`: Output format
  - `text` - Human-readable text format (default)
  - `json` - JSON format
  - `jsonl` - JSON Lines format

### Examples

#### Grapheme Cluster Mode (Default)
```bash
$ mojibox iter "あいうえお🍣🍺"
あ
い
う
え
お
🍣
🍺
```

#### Complex Emoji Handling
```bash
$ mojibox iter "👨‍💻👩‍🍳"
👨‍💻
👩‍🍳
```

#### Codepoint Mode
```bash
$ mojibox iter --mode codepoint "あいう"
あ
い
う
```

#### Byte Mode
```bash
$ mojibox iter --mode byte "hello"
h
e
l
l
o
```

#### Unicode Analysis with dump Command
```bash
# Analyze grapheme clusters and Unicode codepoints
$ mojibox dump "あいう🍣👨‍💻"
Cluster 0: あ (1 codepoint)
  [0] あ    U+3042  HIRAGANA LETTER A

Cluster 1: い (1 codepoint)
  [0] い    U+3044  HIRAGANA LETTER I

Cluster 2: う (1 codepoint)
  [0] う    U+3046  HIRAGANA LETTER U

Cluster 3: 🍣 (1 codepoint)
  [0] 🍣    U+1F363  SUSHI

Cluster 4: 👨‍💻 (3 codepoints)
  [0] 👨    U+1F468  MAN
  [1] \u200d    U+200D  ZERO WIDTH JOINER
  [2] 💻    U+1F4BB  PERSONAL COMPUTER
```

#### JSON Output Format
```bash
$ mojibox dump --format json "👨‍💻"
[
  {
    "cluster_index": 0,
    "grapheme": "👨‍💻",
    "codepoints": [
      {
        "char": "👨",
        "codepoint": "U+1F468",
        "name": "MAN"
      },
      {
        "char": "\u200d",
        "codepoint": "U+200D",
        "name": "ZERO WIDTH JOINER"
      },
      {
        "char": "💻",
        "codepoint": "U+1F4BB",
        "name": "PERSONAL COMPUTER"
      }
    ]
  }
]
```

## Features

- **Accurate Unicode handling**: Uses ICU4X for precise grapheme cluster segmentation
- **Multi-language support**: Handles Japanese, emoji, and combining characters correctly
- **Flexible processing modes**: Choose between grapheme, codepoint, or byte-level processing
- **Unicode analysis**: Comprehensive dump command for analyzing Unicode structure with multiple output formats
- **Command-line interface**: Simple and intuitive CLI with clap argument parsing

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_iter_grapheme_emoji
```

### Formatting and Linting

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## Technical Details

### Dependencies

- **icu_segmenter**: For Unicode-compliant grapheme cluster segmentation
- **clap**: For command-line argument parsing
- **anyhow**: For error handling

### Architecture

The tool processes text through different segmentation strategies:

1. **Grapheme mode**: Uses ICU4X's `GraphemeClusterSegmenter` for Unicode-compliant boundary detection
2. **Codepoint mode**: Iterates through Rust's `char` iterator (Unicode scalar values)
3. **Byte mode**: Processes individual UTF-8 bytes

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

## Contributing

Contributions are welcome! Please ensure all tests pass and follow the existing code style.