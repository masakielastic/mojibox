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
}
