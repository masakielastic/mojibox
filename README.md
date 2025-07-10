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

# Convert string to hexadecimal
mojibox bin2hex "🍣"

# Convert hexadecimal back to string
mojibox hex2bin "F09F8DA3"

# Scrub invalid UTF-8 sequences
mojibox scrub --input-format hex "F09F8D"

# Escape string to Unicode escape sequences
mojibox escape "🍣🍺"

# Unescape Unicode escape sequences
mojibox unescape '\u{1F363}\u{1F37A}'
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

#### bin2hex command
- `--lower`: Use lowercase hex format
- `--format`, `-f`: Output format
  - `default` - Continuous hex string (default)
  - `spaced` - Space-separated hex bytes
  - `escaped` - Escaped format with \x prefix

#### hex2bin command
- Automatically detects input format (continuous, spaced, or escaped)

#### scrub command
- `--input-format`: Input format
  - `binary` - Binary data format (default)
  - `hex` - Hexadecimal format

#### escape command
- `--format`, `-f`: Output format
  - `default` - Default \u{XXXX} format (default)
  - `json` - JSON-compatible \uXXXX format with surrogate pairs

#### unescape command
- Automatically detects and handles both \u{XXXX} and \uXXXX formats
- Properly processes UTF-16 surrogate pairs

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

#### Binary to Hexadecimal Conversion
```bash
# Default format (uppercase, continuous)
$ mojibox bin2hex "🍣"
F09F8DA3

# Lowercase format
$ mojibox bin2hex --lower "🍣"
f09f8da3

# Space-separated format
$ mojibox bin2hex --format spaced "🍣"
F0 9F 8D A3

# Escaped format
$ mojibox bin2hex --format escaped "🍣"
\xF0\x9F\x8D\xA3
```

#### Hexadecimal to Binary Conversion
```bash
# Default format
$ mojibox hex2bin "F09F8DA3"
🍣

# Space-separated format
$ mojibox hex2bin "F0 9F 8D A3"
🍣

# Escaped format
$ mojibox hex2bin "\xF0\x9F\x8D\xA3"
🍣

# Roundtrip conversion
$ mojibox bin2hex "🍣" | mojibox hex2bin
🍣
```

#### Invalid UTF-8 Sequence Scrubbing
```bash
# Scrub invalid UTF-8 from hex data (incomplete emoji)
$ mojibox scrub --input-format hex "F09F8D"
�

# Scrub valid emoji + invalid byte
$ mojibox scrub --input-format hex "F09F8DA3FF"
🍣�

# Scrub overlong encoding
$ mojibox scrub --input-format hex "C080"
��

# Scrub valid UTF-8 text (no changes, binary format is default)
$ mojibox scrub "Hello, 世界!"
Hello, 世界!

# Mixed valid and invalid UTF-8
$ mojibox scrub --input-format hex "48656C6C6F FF 576F726C64"
Hello�World
```

#### Unicode Escape and Unescape
```bash
# Escape string to Unicode escape sequences (default format)
$ mojibox escape "🍣🍺"
\u{1F363}\u{1F37A}

# Escape string to JSON-compatible format with surrogate pairs
$ mojibox escape --format json "🍣🍺"
\uD83C\uDF63\uD83C\uDF7A

# Unescape Unicode escape sequences
$ mojibox unescape '\u{1F363}\u{1F37A}'
🍣🍺

# Unescape JSON-compatible format (surrogate pairs)
$ mojibox unescape '\uD83C\uDF63\uD83C\uDF7A'
🍣🍺

# Handle invalid escape sequences (replaced with replacement character)
$ mojibox unescape '\uD83C'
�

# Handle reversed surrogate pairs
$ mojibox unescape '\uDF63\uD83C'
��

# Handle out-of-range Unicode code points
$ mojibox unescape '\u{110000}'
�
```

## Features

- **Accurate Unicode handling**: Uses ICU4X for precise grapheme cluster segmentation
- **Multi-language support**: Handles Japanese, emoji, and combining characters correctly
- **Flexible processing modes**: Choose between grapheme, codepoint, or byte-level processing
- **Unicode analysis**: Comprehensive dump command for analyzing Unicode structure with multiple output formats
- **Binary/Hex conversion**: Convert strings to hexadecimal representation and back with multiple output formats
- **UTF-8 validation and repair**: Scrub invalid UTF-8 sequences and replace them with replacement characters
- **Unicode escape/unescape**: Convert strings to Unicode escape sequences with support for both default and JSON-compatible formats
- **Surrogate pair handling**: Proper processing of UTF-16 surrogate pairs with error handling for invalid sequences
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