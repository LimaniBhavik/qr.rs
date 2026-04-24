pub const DEFAULT_QR_SIZE: u32 = 300;
pub const MIN_QR_SIZE: u32 = 100;
pub const MAX_QR_SIZE: u32 = 2000;
pub const DEFAULT_QUIET_ZONE: u32 = 4;

pub const BLACK: [u8; 4] = [0, 0, 0, 255];
pub const WHITE: [u8; 4] = [255, 255, 255, 255];

pub fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }

    let mut rgba = BLACK;
    let bytes = hex.as_bytes();
    for i in 0..(hex.len() / 2) {
        let b1 = bytes[i * 2];
        let b2 = bytes[i * 2 + 1];

        let h1 = match b1 {
            b'0'..=b'9' => b1 - b'0',
            b'a'..=b'f' => b1 - b'a' + 10,
            b'A'..=b'F' => b1 - b'A' + 10,
            _ => return None,
        };
        let h2 = match b2 {
            b'0'..=b'9' => b2 - b'0',
            b'a'..=b'f' => b2 - b'a' + 10,
            b'A'..=b'F' => b2 - b'A' + 10,
            _ => return None,
        };
        rgba[i] = (h1 << 4) | h2;
    }
    Some(rgba)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_valid() {
        // 6 digits, uppercase
        assert_eq!(parse_hex_color("#FFFFFF"), Some(WHITE));
        assert_eq!(parse_hex_color("#FF0000"), Some([255, 0, 0, 255]));
        assert_eq!(parse_hex_color("#00FF00"), Some([0, 255, 0, 255]));
        assert_eq!(parse_hex_color("#0000FF"), Some([0, 0, 255, 255]));

        // 6 digits, lowercase & mixed
        assert_eq!(parse_hex_color("#ffffff"), Some(WHITE));
        assert_eq!(parse_hex_color("#aBcDeF"), Some([171, 205, 239, 255]));

        // 8 digits, uppercase
        assert_eq!(parse_hex_color("#FFFFFF00"), Some([255, 255, 255, 0]));
        assert_eq!(parse_hex_color("#11223344"), Some([17, 34, 51, 68]));

        // 8 digits, lowercase & mixed
        assert_eq!(parse_hex_color("#1a2b3c4d"), Some([26, 43, 60, 77]));
        assert_eq!(parse_hex_color("#1A2b3C4d"), Some([26, 43, 60, 77]));

        // Without hash
        assert_eq!(parse_hex_color("000000"), Some(BLACK));
        assert_eq!(parse_hex_color("000000FF"), Some(BLACK));
        assert_eq!(parse_hex_color("aBcDeF"), Some([171, 205, 239, 255]));

        // With multiple hashes (since trim_start_matches removes all leading matches)
        assert_eq!(parse_hex_color("##FFFFFF"), Some(WHITE));
    }

    #[test]
    fn test_parse_hex_color_invalid() {
        // Invalid length
        assert_eq!(parse_hex_color("#FFF"), None); // Too short (3)
        assert_eq!(parse_hex_color("#FFFF"), None); // Too short (4)
        assert_eq!(parse_hex_color("#FFFFF"), None); // Too short (5)
        assert_eq!(parse_hex_color("#1234567"), None); // Incorrect length (7)
        assert_eq!(parse_hex_color("#123456789"), None); // Too long (9)
        assert_eq!(parse_hex_color(""), None); // Empty string
        assert_eq!(parse_hex_color("#"), None); // Only hash

        // Invalid characters
        assert_eq!(parse_hex_color("GGGGGG"), None); // Non-hex letters
        assert_eq!(parse_hex_color("#FG0000"), None); // Mixed valid and non-hex letters
        assert_eq!(parse_hex_color("#FF 000"), None); // Space in the middle
        assert_eq!(parse_hex_color(" #FFFFFF"), None); // Leading space (trim_start_matches only removes '#')
        assert_eq!(parse_hex_color("#FFFFFF "), None); // Trailing space

        // Non-ASCII characters
        assert_eq!(parse_hex_color("#FF🚀000"), None); // Emoji
        assert_eq!(parse_hex_color("##ññññññ"), None); // Non-ascii characters
    }
}
