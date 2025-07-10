use anyhow::Result;
use icu_segmenter::GraphemeClusterSegmenter;
use serde::{Deserialize, Serialize};
use unicode_names2;

pub fn iter_grapheme_icu4x(input: &str) -> Result<Vec<String>> {
    let segmenter = GraphemeClusterSegmenter::new();
    let breakpoints: Vec<usize> = segmenter.segment_str(input).collect();

    let mut result = Vec::new();
    for window in breakpoints.windows(2) {
        let start = window[0];
        let end = window[1];
        result.push(input[start..end].to_string());
    }

    Ok(result)
}

pub fn iter_codepoint(input: &str) -> Vec<String> {
    input.chars().map(|c| c.to_string()).collect()
}

pub fn iter_byte(input: &str) -> Vec<String> {
    input.bytes().map(|b| (b as char).to_string()).collect()
}

#[derive(Debug, Clone)]
pub enum ProcessingMode {
    Grapheme,
    Codepoint,
    Byte,
}

pub fn count_units(input: &str, mode: ProcessingMode) -> Result<usize> {
    match mode {
        ProcessingMode::Grapheme => {
            let segments = iter_grapheme_icu4x(input)?;
            Ok(segments.len())
        }
        ProcessingMode::Codepoint => {
            let segments = iter_codepoint(input);
            Ok(segments.len())
        }
        ProcessingMode::Byte => {
            let segments = iter_byte(input);
            Ok(segments.len())
        }
    }
}

pub fn take_units(input: &str, mode: ProcessingMode, n: usize) -> Result<Vec<String>> {
    match mode {
        ProcessingMode::Grapheme => {
            let segments = iter_grapheme_icu4x(input)?;
            Ok(segments.into_iter().take(n).collect())
        }
        ProcessingMode::Codepoint => {
            let segments = iter_codepoint(input);
            Ok(segments.into_iter().take(n).collect())
        }
        ProcessingMode::Byte => {
            let segments = iter_byte(input);
            Ok(segments.into_iter().take(n).collect())
        }
    }
}

pub fn drop_units(input: &str, mode: ProcessingMode, n: usize) -> Result<Vec<String>> {
    match mode {
        ProcessingMode::Grapheme => {
            let segments = iter_grapheme_icu4x(input)?;
            Ok(segments.into_iter().skip(n).collect())
        }
        ProcessingMode::Codepoint => {
            let segments = iter_codepoint(input);
            Ok(segments.into_iter().skip(n).collect())
        }
        ProcessingMode::Byte => {
            let segments = iter_byte(input);
            Ok(segments.into_iter().skip(n).collect())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepointInfo {
    pub index: usize,
    pub char: String,
    pub codepoint: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub cluster_index: usize,
    pub display: String,
    pub codepoints: Vec<CodepointInfo>,
}

#[derive(Debug, Clone)]
pub enum DumpFormat {
    Text,
    Json,
    Jsonl,
}

pub fn get_unicode_name(ch: char) -> String {
    unicode_names2::name(ch)
        .map(|name| name.to_string())
        .unwrap_or_else(|| format!("UNKNOWN CHARACTER"))
}

pub fn dump_graphemes(input: &str, format: DumpFormat) -> Result<String> {
    let segments = iter_grapheme_icu4x(input)?;
    let mut clusters = Vec::new();
    
    for (cluster_index, segment) in segments.iter().enumerate() {
        let mut codepoints = Vec::new();
        
        for (index, ch) in segment.chars().enumerate() {
            let codepoint_info = CodepointInfo {
                index,
                char: if ch.is_control() || ch == '\u{200D}' {
                    format!("\\u{:04x}", ch as u32)
                } else {
                    ch.to_string()
                },
                codepoint: format!("U+{:04X}", ch as u32),
                name: get_unicode_name(ch),
            };
            codepoints.push(codepoint_info);
        }
        
        let cluster_info = ClusterInfo {
            cluster_index,
            display: segment.clone(),
            codepoints,
        };
        clusters.push(cluster_info);
    }
    
    match format {
        DumpFormat::Text => format_text_output(&clusters),
        DumpFormat::Json => format_json_output(&clusters),
        DumpFormat::Jsonl => format_jsonl_output(&clusters),
    }
}

fn format_text_output(clusters: &[ClusterInfo]) -> Result<String> {
    let mut output = String::new();
    
    for cluster in clusters {
        output.push_str(&format!("\nCluster {}: {} ({} codepoint{})\n", 
            cluster.cluster_index, 
            cluster.display, 
            cluster.codepoints.len(),
            if cluster.codepoints.len() == 1 { "" } else { "s" }
        ));
        
        for codepoint in &cluster.codepoints {
            output.push_str(&format!("  [{}] {}    {}  {}\n", 
                codepoint.index, 
                codepoint.char,
                codepoint.codepoint,
                codepoint.name
            ));
        }
    }
    
    if !clusters.is_empty() {
        output.push('\n');
    }
    
    Ok(output)
}

fn format_json_output(clusters: &[ClusterInfo]) -> Result<String> {
    let json = serde_json::to_string_pretty(clusters)?;
    Ok(json)
}

fn format_jsonl_output(clusters: &[ClusterInfo]) -> Result<String> {
    let mut output = String::new();
    
    for cluster in clusters {
        let line = serde_json::to_string(cluster)?;
        output.push_str(&line);
        output.push('\n');
    }
    
    Ok(output)
}

pub fn ord_characters(input: &str, lowercase: bool, no_prefix: bool) -> Vec<String> {
    input.chars().map(|ch| {
        let codepoint = ch as u32;
        if no_prefix {
            if lowercase {
                format!("{:x}", codepoint)
            } else {
                format!("{:X}", codepoint)
            }
        } else {
            if lowercase {
                format!("0x{:x}", codepoint)
            } else {
                format!("0x{:X}", codepoint)
            }
        }
    }).collect()
}

pub fn chr_from_codepoints(codepoints: &[String]) -> Result<String> {
    let mut result = String::new();
    
    for cp_str in codepoints {
        let hex_str = cp_str.strip_prefix("0x")
            .or_else(|| cp_str.strip_prefix("0X"))
            .unwrap_or(cp_str);
            
        let codepoint = u32::from_str_radix(hex_str, 16)
            .map_err(|_| anyhow::anyhow!("Invalid hex format: {}", cp_str))?;
            
        let ch = char::from_u32(codepoint)
            .ok_or_else(|| anyhow::anyhow!("Invalid Unicode codepoint: U+{:X}", codepoint))?;
            
        result.push(ch);
    }
    
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum HexFormat {
    Default,
    Spaced,
    Escaped,
}

pub fn bin2hex(input: &str, lowercase: bool, format: HexFormat) -> Result<String> {
    let bytes = input.as_bytes();
    let hex_chars: Vec<String> = bytes.iter().map(|byte| {
        if lowercase {
            format!("{:02x}", byte)
        } else {
            format!("{:02X}", byte)
        }
    }).collect();

    match format {
        HexFormat::Default => Ok(hex_chars.join("")),
        HexFormat::Spaced => Ok(hex_chars.join(" ")),
        HexFormat::Escaped => Ok(hex_chars.iter().map(|h| format!("\\x{}", h)).collect::<Vec<_>>().join("")),
    }
}

pub fn hex2bin(hex_input: &str) -> Result<String> {
    let cleaned_input = if hex_input.starts_with("\\x") {
        // Handle escaped format: \xF0\x9F\x8D\xA3
        hex_input.replace("\\x", "")
    } else if hex_input.contains(' ') {
        // Handle spaced format: F0 9F 8D A3
        hex_input.replace(' ', "")
    } else {
        // Handle default format: F09F8DA3
        hex_input.to_string()
    };

    if cleaned_input.len() % 2 != 0 {
        return Err(anyhow::anyhow!("Invalid hex input: odd number of characters"));
    }

    let mut bytes = Vec::new();
    for i in (0..cleaned_input.len()).step_by(2) {
        let hex_pair = &cleaned_input[i..i+2];
        let byte = u8::from_str_radix(hex_pair, 16)
            .map_err(|_| anyhow::anyhow!("Invalid hex character in: {}", hex_pair))?;
        bytes.push(byte);
    }

    let result = String::from_utf8(bytes)
        .map_err(|_| anyhow::anyhow!("Invalid UTF-8 sequence"))?;
    
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum InputFormat {
    Binary,
    Hex,
}

pub fn scrub_invalid_utf8(input: &str, format: InputFormat) -> Result<String> {
    let bytes = match format {
        InputFormat::Binary => {
            // Treat input as binary data (UTF-8 bytes)
            input.as_bytes().to_vec()
        }
        InputFormat::Hex => {
            // Parse as hexadecimal string (similar to hex2bin)
            let cleaned_input = if input.starts_with("\\x") {
                // Handle escaped format: \xF0\x9F\x8D\xA3
                input.replace("\\x", "")
            } else if input.contains(' ') {
                // Handle spaced format: F0 9F 8D A3
                input.replace(' ', "")
            } else {
                // Handle default format: F09F8DA3
                input.to_string()
            };

            if cleaned_input.is_empty() {
                return Ok(String::new());
            }

            if cleaned_input.len() % 2 != 0 {
                return Err(anyhow::anyhow!("Invalid hex input: odd number of characters"));
            }

            let mut bytes = Vec::new();
            for i in (0..cleaned_input.len()).step_by(2) {
                let hex_pair = &cleaned_input[i..i+2];
                let byte = u8::from_str_radix(hex_pair, 16)
                    .map_err(|_| anyhow::anyhow!("Invalid hex character in: {}", hex_pair))?;
                bytes.push(byte);
            }
            bytes
        }
    };

    // Use from_utf8_lossy to replace invalid UTF-8 sequences with U+FFFD
    let result = String::from_utf8_lossy(&bytes).into_owned();
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum EscapeFormat {
    Default,
    Json,
}

pub fn escape_unicode(input: &str) -> String {
    escape_unicode_with_format(input, EscapeFormat::Default)
}

pub fn escape_unicode_with_format(input: &str, format: EscapeFormat) -> String {
    match format {
        EscapeFormat::Default => {
            input.chars()
                .map(|ch| format!("\\u{{{:X}}}", ch as u32))
                .collect::<Vec<_>>()
                .join("")
        }
        EscapeFormat::Json => {
            input.chars()
                .flat_map(|ch| {
                    let code_point = ch as u32;
                    if code_point <= 0xFFFF {
                        // BMP character - single \uXXXX
                        vec![format!("\\u{:04X}", code_point)]
                    } else {
                        // Supplementary character - surrogate pair
                        let adjusted = code_point - 0x10000;
                        let high = 0xD800 + (adjusted >> 10);
                        let low = 0xDC00 + (adjusted & 0x3FF);
                        vec![format!("\\u{:04X}", high), format!("\\u{:04X}", low)]
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        }
    }
}

pub fn unescape_unicode(input: &str) -> String {
    let mut result = String::new();
    let mut remaining = input;
    
    while !remaining.is_empty() {
        if let Some(rest) = remaining.strip_prefix("\\u{") {
            // Handle \u{...} format
            if let Some(close_pos) = rest.find('}') {
                let hex_part = &rest[..close_pos];
                remaining = &rest[close_pos + 1..];
                
                if hex_part.is_empty() {
                    result.push('\u{FFFD}');
                } else if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                    match u32::from_str_radix(hex_part, 16) {
                        Ok(code_point) => {
                            if let Some(unicode_char) = char::from_u32(code_point) {
                                result.push(unicode_char);
                            } else {
                                result.push('\u{FFFD}');
                            }
                        }
                        Err(_) => {
                            result.push('\u{FFFD}');
                        }
                    }
                } else {
                    result.push('\u{FFFD}');
                }
            } else {
                // No closing brace found - consume all remaining characters
                result.push('\u{FFFD}');
                remaining = "";
            }
        } else if let Some(rest) = remaining.strip_prefix("\\u") {
            // Handle \uXXXX format
            if rest.len() >= 4 && rest.chars().take(4).all(|c| c.is_ascii_hexdigit()) {
                let hex_part = &rest[..4];
                remaining = &rest[4..];
                
                match u16::from_str_radix(hex_part, 16) {
                    Ok(code_unit) => {
                        // Handle surrogate pairs
                        if (0xD800..=0xDBFF).contains(&code_unit) {
                            // High surrogate - look for low surrogate
                            if let Some(low_rest) = remaining.strip_prefix("\\u") {
                                if low_rest.len() >= 4 && low_rest.chars().take(4).all(|c| c.is_ascii_hexdigit()) {
                                    let low_hex = &low_rest[..4];
                                    match u16::from_str_radix(low_hex, 16) {
                                        Ok(low_surrogate) => {
                                            if (0xDC00..=0xDFFF).contains(&low_surrogate) {
                                                // Valid surrogate pair
                                                let code_point = 0x10000 + ((code_unit as u32 - 0xD800) << 10) + (low_surrogate as u32 - 0xDC00);
                                                if let Some(unicode_char) = char::from_u32(code_point) {
                                                    result.push(unicode_char);
                                                    remaining = &low_rest[4..];
                                                } else {
                                                    result.push('\u{FFFD}');
                                                }
                                            } else {
                                                // Invalid low surrogate
                                                result.push('\u{FFFD}');
                                                result.push('\u{FFFD}');
                                                remaining = &low_rest[4..];
                                            }
                                        }
                                        Err(_) => {
                                            // Invalid hex in low surrogate
                                            result.push('\u{FFFD}');
                                            result.push('\u{FFFD}');
                                            remaining = &low_rest[4..];
                                        }
                                    }
                                } else {
                                    // Incomplete low surrogate
                                    result.push('\u{FFFD}');
                                }
                            } else {
                                // No low surrogate after high surrogate
                                result.push('\u{FFFD}');
                            }
                        } else if (0xDC00..=0xDFFF).contains(&code_unit) {
                            // Lone low surrogate
                            result.push('\u{FFFD}');
                        } else {
                            // Regular BMP character
                            if let Some(unicode_char) = char::from_u32(code_unit as u32) {
                                result.push(unicode_char);
                            } else {
                                result.push('\u{FFFD}');
                            }
                        }
                    }
                    Err(_) => {
                        // Invalid hex format
                        result.push('\u{FFFD}');
                    }
                }
            } else {
                // Incomplete \uXXXX sequence
                result.push('\u{FFFD}');
                remaining = rest;
            }
        } else {
            // Regular character or non-unicode escape
            let ch = remaining.chars().next().unwrap();
            result.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_grapheme_basic() {
        let result = iter_grapheme_icu4x("hello").unwrap();
        assert_eq!(result, vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_iter_grapheme_japanese() {
        let result = iter_grapheme_icu4x("あいうえお").unwrap();
        assert_eq!(result, vec!["あ", "い", "う", "え", "お"]);
    }

    #[test]
    fn test_iter_grapheme_emoji() {
        let result = iter_grapheme_icu4x("🍣🍺").unwrap();
        assert_eq!(result, vec!["🍣", "🍺"]);
    }

    #[test]
    fn test_iter_grapheme_complex_emoji() {
        // 肌色の絵文字（結合文字）
        let result = iter_grapheme_icu4x("👨‍💻👩‍🍳").unwrap();
        assert_eq!(result, vec!["👨‍💻", "👩‍🍳"]);
    }

    #[test]
    fn test_iter_grapheme_combining_characters() {
        // 結合文字（濁点）
        let result = iter_grapheme_icu4x("が").unwrap();
        assert_eq!(result, vec!["が"]);
    }

    #[test]
    fn test_iter_grapheme_mixed() {
        let result = iter_grapheme_icu4x("あいうえお🍣🍺").unwrap();
        assert_eq!(result, vec!["あ", "い", "う", "え", "お", "🍣", "🍺"]);
    }

    #[test]
    fn test_iter_codepoint_basic() {
        let result = iter_codepoint("hello");
        assert_eq!(result, vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_iter_codepoint_japanese() {
        let result = iter_codepoint("あいう");
        assert_eq!(result, vec!["あ", "い", "う"]);
    }

    #[test]
    fn test_iter_codepoint_emoji() {
        let result = iter_codepoint("🍣🍺");
        assert_eq!(result, vec!["🍣", "🍺"]);
    }

    #[test]
    fn test_iter_byte_basic() {
        let result = iter_byte("hello");
        assert_eq!(result, vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_iter_byte_japanese() {
        // 日本語文字は複数バイトになる
        let result = iter_byte("あ");
        assert_eq!(result.len(), 3); // UTF-8で「あ」は3バイト
    }

    #[test]
    fn test_empty_string() {
        let result = iter_grapheme_icu4x("").unwrap();
        assert_eq!(result, Vec::<String>::new());

        let result = iter_codepoint("");
        assert_eq!(result, Vec::<String>::new());

        let result = iter_byte("");
        assert_eq!(result, Vec::<String>::new());
    }

    // Tests for count_units
    #[test]
    fn test_count_units_grapheme() {
        let result = count_units("あいうえお🍣🍺", ProcessingMode::Grapheme).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_count_units_codepoint() {
        let result = count_units("🍣🍺", ProcessingMode::Codepoint).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_units_byte() {
        let result = count_units("hello", ProcessingMode::Byte).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_count_units_empty() {
        let result = count_units("", ProcessingMode::Grapheme).unwrap();
        assert_eq!(result, 0);
    }

    // Tests for take_units
    #[test]
    fn test_take_units_grapheme() {
        let result = take_units("あいうえお🍣🍺", ProcessingMode::Grapheme, 3).unwrap();
        assert_eq!(result, vec!["あ", "い", "う"]);
    }

    #[test]
    fn test_take_units_codepoint() {
        let result = take_units("hello", ProcessingMode::Codepoint, 2).unwrap();
        assert_eq!(result, vec!["h", "e"]);
    }

    #[test]
    fn test_take_units_byte() {
        let result = take_units("abc", ProcessingMode::Byte, 2).unwrap();
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_take_units_n_greater_than_length() {
        let result = take_units("abc", ProcessingMode::Codepoint, 10).unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_take_units_zero() {
        let result = take_units("abc", ProcessingMode::Codepoint, 0).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_take_units_empty_string() {
        let result = take_units("", ProcessingMode::Grapheme, 5).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    // Tests for drop_units
    #[test]
    fn test_drop_units_grapheme() {
        let result = drop_units("あいうえお🍣🍺", ProcessingMode::Grapheme, 2).unwrap();
        assert_eq!(result, vec!["う", "え", "お", "🍣", "🍺"]);
    }

    #[test]
    fn test_drop_units_codepoint() {
        let result = drop_units("hello", ProcessingMode::Codepoint, 1).unwrap();
        assert_eq!(result, vec!["e", "l", "l", "o"]);
    }

    #[test]
    fn test_drop_units_byte() {
        let result = drop_units("abc", ProcessingMode::Byte, 1).unwrap();
        assert_eq!(result, vec!["b", "c"]);
    }

    #[test]
    fn test_drop_units_n_greater_than_length() {
        let result = drop_units("abc", ProcessingMode::Codepoint, 10).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_drop_units_zero() {
        let result = drop_units("abc", ProcessingMode::Codepoint, 0).unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_drop_units_empty_string() {
        let result = drop_units("", ProcessingMode::Grapheme, 5).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    // Tests for ord_characters function
    #[test]
    fn test_ord_default_format() {
        let result = ord_characters("漢字🍺", false, false);
        assert_eq!(result, vec!["0x6F22", "0x5B57", "0x1F37A"]);
    }

    #[test]
    fn test_ord_lowercase() {
        let result = ord_characters("漢字🍺", true, false);
        assert_eq!(result, vec!["0x6f22", "0x5b57", "0x1f37a"]);
    }

    #[test]
    fn test_ord_no_prefix() {
        let result = ord_characters("漢字🍺", false, true);
        assert_eq!(result, vec!["6F22", "5B57", "1F37A"]);
    }

    #[test]
    fn test_ord_lowercase_no_prefix() {
        let result = ord_characters("漢字🍺", true, true);
        assert_eq!(result, vec!["6f22", "5b57", "1f37a"]);
    }

    #[test]
    fn test_ord_ascii() {
        let result = ord_characters("ABC", false, false);
        assert_eq!(result, vec!["0x41", "0x42", "0x43"]);
    }

    #[test]
    fn test_ord_empty_input() {
        let result = ord_characters("", false, false);
        assert_eq!(result, Vec::<String>::new());
    }

    // Tests for chr_from_codepoints function
    #[test]
    fn test_chr_with_0x_prefix() {
        let result = chr_from_codepoints(&["0x6f22".to_string(), "0x5b57".to_string(), "0x1f37a".to_string()]).unwrap();
        assert_eq!(result, "漢字🍺");
    }

    #[test]
    fn test_chr_without_prefix() {
        let result = chr_from_codepoints(&["6F22".to_string(), "5B57".to_string(), "1F37A".to_string()]).unwrap();
        assert_eq!(result, "漢字🍺");
    }

    #[test]
    fn test_chr_mixed_case() {
        let result = chr_from_codepoints(&["0x6F22".to_string(), "5b57".to_string(), "1f37a".to_string()]).unwrap();
        assert_eq!(result, "漢字🍺");
    }

    #[test]
    fn test_chr_ascii() {
        let result = chr_from_codepoints(&["0x41".to_string(), "0x42".to_string(), "0x43".to_string()]).unwrap();
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_chr_invalid_hex() {
        let result = chr_from_codepoints(&["0xGGGG".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chr_invalid_codepoint() {
        let result = chr_from_codepoints(&["0x110000".to_string()]); // > U+10FFFF
        assert!(result.is_err());
    }

    #[test]
    fn test_chr_empty_input() {
        let result = chr_from_codepoints(&[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_chr_uppercase_prefix() {
        let result = chr_from_codepoints(&["0X41".to_string(), "0X42".to_string()]).unwrap();
        assert_eq!(result, "AB");
    }

    // Complex Unicode tests
    #[test]
    fn test_complex_emoji_count() {
        let result = count_units("👨‍💻👩‍🍳", ProcessingMode::Grapheme).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_complex_emoji_take() {
        let result = take_units("👨‍💻👩‍🍳", ProcessingMode::Grapheme, 1).unwrap();
        assert_eq!(result, vec!["👨‍💻"]);
    }

    #[test]
    fn test_complex_emoji_drop() {
        let result = drop_units("👨‍💻👩‍🍳", ProcessingMode::Grapheme, 1).unwrap();
        assert_eq!(result, vec!["👩‍🍳"]);
    }

    // Tests for dump functionality
    #[test]
    fn test_dump_graphemes_text_simple() {
        let result = dump_graphemes("🍣", DumpFormat::Text).unwrap();
        assert!(result.contains("Cluster 0: 🍣 (1 codepoint)"));
        assert!(result.contains("U+1F363"));
        assert!(result.contains("SUSHI"));
    }

    #[test]
    fn test_dump_graphemes_json() {
        let result = dump_graphemes("🍣", DumpFormat::Json).unwrap();
        assert!(result.contains("cluster_index"));
        assert!(result.contains("codepoints"));
        assert!(result.contains("U+1F363"));
        assert!(result.contains("SUSHI"));
    }

    #[test]
    fn test_dump_graphemes_jsonl() {
        let result = dump_graphemes("🍣🍺", DumpFormat::Jsonl).unwrap();
        let lines: Vec<&str> = result.trim().split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("🍣"));
        assert!(lines[1].contains("🍺"));
    }

    #[test]
    fn test_dump_complex_emoji() {
        let result = dump_graphemes("👩‍👩‍👧‍👦", DumpFormat::Text).unwrap();
        assert!(result.contains("Cluster 0: 👩‍👩‍👧‍👦 (7 codepoints)"));
        assert!(result.contains("WOMAN"));
        assert!(result.contains("ZERO WIDTH JOINER"));
        assert!(result.contains("GIRL"));
        assert!(result.contains("BOY"));
    }

    #[test]
    fn test_get_unicode_name() {
        assert_eq!(get_unicode_name('A'), "LATIN CAPITAL LETTER A");
        assert_eq!(get_unicode_name('🍣'), "SUSHI");
        assert_eq!(get_unicode_name('\u{200D}'), "ZERO WIDTH JOINER");
    }

    #[test]
    fn test_dump_empty_string() {
        let result = dump_graphemes("", DumpFormat::Text).unwrap();
        assert_eq!(result, "");
    }

    // Tests for bin2hex function
    #[test]
    fn test_bin2hex_default_format() {
        let result = bin2hex("🍣", false, HexFormat::Default).unwrap();
        assert_eq!(result, "F09F8DA3");
    }

    #[test]
    fn test_bin2hex_lowercase() {
        let result = bin2hex("🍣", true, HexFormat::Default).unwrap();
        assert_eq!(result, "f09f8da3");
    }

    #[test]
    fn test_bin2hex_spaced_format() {
        let result = bin2hex("🍣", false, HexFormat::Spaced).unwrap();
        assert_eq!(result, "F0 9F 8D A3");
    }

    #[test]
    fn test_bin2hex_escaped_format() {
        let result = bin2hex("🍣", false, HexFormat::Escaped).unwrap();
        assert_eq!(result, "\\xF0\\x9F\\x8D\\xA3");
    }

    #[test]
    fn test_bin2hex_ascii() {
        let result = bin2hex("ABC", false, HexFormat::Default).unwrap();
        assert_eq!(result, "414243");
    }

    #[test]
    fn test_bin2hex_empty_string() {
        let result = bin2hex("", false, HexFormat::Default).unwrap();
        assert_eq!(result, "");
    }

    // Tests for hex2bin function
    #[test]
    fn test_hex2bin_default_format() {
        let result = hex2bin("F09F8DA3").unwrap();
        assert_eq!(result, "🍣");
    }

    #[test]
    fn test_hex2bin_spaced_format() {
        let result = hex2bin("F0 9F 8D A3").unwrap();
        assert_eq!(result, "🍣");
    }

    #[test]
    fn test_hex2bin_escaped_format() {
        let result = hex2bin("\\xF0\\x9F\\x8D\\xA3").unwrap();
        assert_eq!(result, "🍣");
    }

    #[test]
    fn test_hex2bin_lowercase() {
        let result = hex2bin("f09f8da3").unwrap();
        assert_eq!(result, "🍣");
    }

    #[test]
    fn test_hex2bin_ascii() {
        let result = hex2bin("414243").unwrap();
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_hex2bin_empty_string() {
        let result = hex2bin("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_hex2bin_invalid_hex() {
        let result = hex2bin("GG");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex2bin_odd_length() {
        let result = hex2bin("F0F");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex2bin_invalid_utf8() {
        let result = hex2bin("FF");
        assert!(result.is_err());
    }

    // Roundtrip tests
    #[test]
    fn test_roundtrip_default() {
        let original = "🍣🍺漢字";
        let hex = bin2hex(original, false, HexFormat::Default).unwrap();
        let restored = hex2bin(&hex).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_roundtrip_spaced() {
        let original = "🍣🍺漢字";
        let hex = bin2hex(original, false, HexFormat::Spaced).unwrap();
        let restored = hex2bin(&hex).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_roundtrip_escaped() {
        let original = "🍣🍺漢字";
        let hex = bin2hex(original, false, HexFormat::Escaped).unwrap();
        let restored = hex2bin(&hex).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_roundtrip_ascii() {
        let original = "Hello, World!";
        let hex = bin2hex(original, false, HexFormat::Default).unwrap();
        let restored = hex2bin(&hex).unwrap();
        assert_eq!(original, restored);
    }

    // Tests for scrub_invalid_utf8 function
    #[test]
    fn test_scrub_valid_utf8() {
        let result = scrub_invalid_utf8("Hello, 世界! 🍣", InputFormat::Binary).unwrap();
        assert_eq!(result, "Hello, 世界! 🍣");
    }

    #[test]
    fn test_scrub_incomplete_sushi_emoji() {
        // 🍣 is F0 9F 8D A3, but F0 9F 8D is incomplete
        let result = scrub_invalid_utf8("F09F8D", InputFormat::Hex).unwrap();
        assert_eq!(result, "�");
    }

    #[test]
    fn test_scrub_valid_emoji_plus_invalid_byte() {
        // 🍣 (F0 9F 8D A3) + invalid byte FF
        let result = scrub_invalid_utf8("F09F8DA3FF", InputFormat::Hex).unwrap();
        assert_eq!(result, "🍣�");
    }

    #[test]
    fn test_scrub_overlong_encoding() {
        // C0 80 is overlong encoding of null byte
        let result = scrub_invalid_utf8("C080", InputFormat::Hex).unwrap();
        assert_eq!(result, "��");
    }

    #[test]
    fn test_scrub_multiple_invalid_sequences() {
        // Multiple invalid sequences
        let result = scrub_invalid_utf8("F09F8D F09F8DA3 FF C080", InputFormat::Hex).unwrap();
        assert_eq!(result, "�🍣���");
    }

    #[test]
    fn test_scrub_escaped_hex_format() {
        // Escaped format with incomplete emoji
        let result = scrub_invalid_utf8("\\xF0\\x9F\\x8D", InputFormat::Hex).unwrap();
        assert_eq!(result, "�");
    }

    #[test]
    fn test_scrub_spaced_hex_format() {
        // Spaced format with incomplete emoji
        let result = scrub_invalid_utf8("F0 9F 8D", InputFormat::Hex).unwrap();
        assert_eq!(result, "�");
    }


    #[test]
    fn test_scrub_empty_input() {
        let result = scrub_invalid_utf8("", InputFormat::Binary).unwrap();
        assert_eq!(result, "");
        
        let result = scrub_invalid_utf8("", InputFormat::Hex).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_scrub_invalid_hex_format() {
        // Invalid hex characters should cause error
        let result = scrub_invalid_utf8("GG", InputFormat::Hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_scrub_odd_length_hex() {
        // Odd length hex should cause error
        let result = scrub_invalid_utf8("F0F", InputFormat::Hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_scrub_various_invalid_sequences() {
        // Test various invalid UTF-8 sequences
        let test_cases = vec![
            ("80", "�"),           // Continuation byte without start
            ("C0", "�"),           // Incomplete 2-byte sequence
            ("E0", "�"),           // Incomplete 3-byte sequence
            ("F0", "�"),           // Incomplete 4-byte sequence
            ("FE", "�"),           // Invalid start byte
            ("FF", "�"),           // Invalid start byte
            ("C0C0", "��"),        // Two invalid bytes
            ("E080", "��"),        // Invalid 3-byte sequence
            ("F08080", "���"),     // Invalid 4-byte sequence
        ];

        for (input, expected) in test_cases {
            let result = scrub_invalid_utf8(input, InputFormat::Hex).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_scrub_mixed_valid_invalid() {
        // Test mixing valid and invalid UTF-8
        let result = scrub_invalid_utf8("48656C6C6F FF 576F726C64", InputFormat::Hex).unwrap();
        assert_eq!(result, "Hello�World");
    }

    // Tests for escape_unicode function
    #[test]
    fn test_escape_unicode_basic() {
        let result = escape_unicode("🍣🍺");
        assert_eq!(result, "\\u{1F363}\\u{1F37A}");
    }

    #[test]
    fn test_escape_unicode_ascii() {
        let result = escape_unicode("ABC");
        assert_eq!(result, "\\u{41}\\u{42}\\u{43}");
    }

    #[test]
    fn test_escape_unicode_japanese() {
        let result = escape_unicode("漢字");
        assert_eq!(result, "\\u{6F22}\\u{5B57}");
    }

    #[test]
    fn test_escape_unicode_empty() {
        let result = escape_unicode("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_escape_unicode_mixed() {
        let result = escape_unicode("A🍣B");
        assert_eq!(result, "\\u{41}\\u{1F363}\\u{42}");
    }

    // Tests for unescape_unicode function
    #[test]
    fn test_unescape_unicode_basic() {
        let result = unescape_unicode("\\u{1F363}\\u{1F37A}");
        assert_eq!(result, "🍣🍺");
    }

    #[test]
    fn test_unescape_unicode_ascii() {
        let result = unescape_unicode("\\u{41}\\u{42}\\u{43}");
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_unescape_unicode_japanese() {
        let result = unescape_unicode("\\u{6F22}\\u{5B57}");
        assert_eq!(result, "漢字");
    }

    #[test]
    fn test_unescape_unicode_empty() {
        let result = unescape_unicode("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_unescape_unicode_mixed() {
        let result = unescape_unicode("\\u{41}\\u{1F363}\\u{42}");
        assert_eq!(result, "A🍣B");
    }

    #[test]
    fn test_unescape_unicode_4digit_format() {
        let result = unescape_unicode("\\u0041\\u0042\\u0043");
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_unescape_unicode_surrogate_pair_valid() {
        // Valid surrogate pair for 🍣 (U+1F363)
        let result = unescape_unicode("\\uD83C\\uDF63");
        assert_eq!(result, "🍣");
    }

    #[test]
    fn test_unescape_unicode_surrogate_pair_reversed() {
        // Reversed surrogate pair should produce replacement characters
        let result = unescape_unicode("\\uDF63\\uD83C");
        assert_eq!(result, "��");
    }

    #[test]
    fn test_unescape_unicode_lone_high_surrogate() {
        // Lone high surrogate should produce replacement character
        let result = unescape_unicode("\\uD83C");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_lone_low_surrogate() {
        // Lone low surrogate should produce replacement character
        let result = unescape_unicode("\\uDF63");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_invalid_codepoint() {
        // Invalid Unicode code point (> U+10FFFF)
        let result = unescape_unicode("\\u{110000}");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_invalid_hex() {
        // Invalid hex characters
        let result = unescape_unicode("\\u{GGGG}");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_incomplete_sequence() {
        // Incomplete escape sequence
        let result = unescape_unicode("\\u{123");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_empty_hex() {
        // Empty hex sequence
        let result = unescape_unicode("\\u{}");
        assert_eq!(result, "�");
    }

    #[test]
    fn test_unescape_unicode_not_escape() {
        // Not a unicode escape
        let result = unescape_unicode("\\x41");
        assert_eq!(result, "\\x41");
    }

    #[test]
    fn test_unescape_unicode_backslash_at_end() {
        // Backslash at end of string
        let result = unescape_unicode("test\\");
        assert_eq!(result, "test\\");
    }

    #[test]
    fn test_unescape_unicode_mixed_with_text() {
        // Mixed with regular text
        let result = unescape_unicode("Hello \\u{1F363} World");
        assert_eq!(result, "Hello 🍣 World");
    }

    #[test]
    fn test_unescape_unicode_multiple_formats() {
        // Mix of \u{} and \u formats
        let result = unescape_unicode("\\u{41}\\u0042\\u{1F363}");
        assert_eq!(result, "AB🍣");
    }

    // Roundtrip tests
    #[test]
    fn test_escape_unescape_roundtrip() {
        let original = "🍣🍺漢字ABC";
        let escaped = escape_unicode(original);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_escape_unescape_roundtrip_complex() {
        let original = "Hello 世界! 🍣🍺 Test";
        let escaped = escape_unicode(original);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_escape_unescape_roundtrip_empty() {
        let original = "";
        let escaped = escape_unicode(original);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    // Tests for escape_unicode_with_format function
    #[test]
    fn test_escape_unicode_with_format_default() {
        let result = escape_unicode_with_format("🍣🍺", EscapeFormat::Default);
        assert_eq!(result, "\\u{1F363}\\u{1F37A}");
    }

    #[test]
    fn test_escape_unicode_with_format_json() {
        let result = escape_unicode_with_format("🍣🍺", EscapeFormat::Json);
        assert_eq!(result, "\\uD83C\\uDF63\\uD83C\\uDF7A");
    }

    #[test]
    fn test_escape_unicode_with_format_json_bmp() {
        // BMP characters should not use surrogate pairs
        let result = escape_unicode_with_format("ABC漢字", EscapeFormat::Json);
        assert_eq!(result, "\\u0041\\u0042\\u0043\\u6F22\\u5B57");
    }

    #[test]
    fn test_escape_unicode_with_format_json_mixed() {
        let result = escape_unicode_with_format("A🍣B", EscapeFormat::Json);
        assert_eq!(result, "\\u0041\\uD83C\\uDF63\\u0042");
    }

    #[test]
    fn test_escape_unicode_with_format_json_empty() {
        let result = escape_unicode_with_format("", EscapeFormat::Json);
        assert_eq!(result, "");
    }

    #[test]
    fn test_escape_unicode_with_format_json_complex_emoji() {
        // Test with various emoji that require surrogate pairs
        let result = escape_unicode_with_format("😀😁😂", EscapeFormat::Json);
        assert_eq!(result, "\\uD83D\\uDE00\\uD83D\\uDE01\\uD83D\\uDE02");
    }

    // JSON format roundtrip tests
    #[test]
    fn test_json_format_roundtrip_emoji() {
        let original = "🍣🍺";
        let escaped = escape_unicode_with_format(original, EscapeFormat::Json);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_json_format_roundtrip_mixed() {
        let original = "Hello 🍣 World!";
        let escaped = escape_unicode_with_format(original, EscapeFormat::Json);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_json_format_roundtrip_bmp_only() {
        let original = "Hello 漢字 World!";
        let escaped = escape_unicode_with_format(original, EscapeFormat::Json);
        let unescaped = unescape_unicode(&escaped);
        assert_eq!(original, unescaped);
    }

    // Test specific surrogate pair calculation
    #[test]
    fn test_surrogate_pair_calculation() {
        // U+1F363 (🍣) should become D83C DF63
        let result = escape_unicode_with_format("🍣", EscapeFormat::Json);
        assert_eq!(result, "\\uD83C\\uDF63");
        
        // U+1F37A (🍺) should become D83C DF7A  
        let result = escape_unicode_with_format("🍺", EscapeFormat::Json);
        assert_eq!(result, "\\uD83C\\uDF7A");
    }

    #[test]
    fn test_bmp_boundary_characters() {
        // Test characters at BMP boundary (U+FFFF)
        let result = escape_unicode_with_format("\u{FFFF}", EscapeFormat::Json);
        assert_eq!(result, "\\uFFFF");
        
        // Test first supplementary character (U+10000)
        let result = escape_unicode_with_format("\u{10000}", EscapeFormat::Json);
        assert_eq!(result, "\\uD800\\uDC00");
    }
}
