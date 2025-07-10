use anyhow::Result;
use icu_segmenter::GraphemeClusterSegmenter;

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
}
