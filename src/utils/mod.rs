pub const DEFAULT_QR_SIZE: u32 = 300;
pub const MIN_QR_SIZE: u32 = 100;
pub const MAX_QR_SIZE: u32 = 2000;
pub const DEFAULT_QUIET_ZONE: u32 = 4;

#[inline(always)]
fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }

    let mut rgba = [0, 0, 0, 255];
    let bytes = hex.as_bytes();
    for (i, chunk) in bytes.chunks(2).enumerate() {
        let high = hex_val(chunk[0])?;
        let low = hex_val(chunk[1])?;
        rgba[i] = (high << 4) | low;
    }
    Some(rgba)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_valid() {
        // 6 digits, uppercase
        assert_eq!(parse_hex_color("#FFFFFF"), Some([255, 255, 255, 255]));
        assert_eq!(parse_hex_color("#FF0000"), Some([255, 0, 0, 255]));
        assert_eq!(parse_hex_color("#00FF00"), Some([0, 255, 0, 255]));
        assert_eq!(parse_hex_color("#0000FF"), Some([0, 0, 255, 255]));

        // 6 digits, lowercase & mixed
        assert_eq!(parse_hex_color("#ffffff"), Some([255, 255, 255, 255]));
        assert_eq!(parse_hex_color("#aBcDeF"), Some([171, 205, 239, 255]));

        // 8 digits, uppercase
        assert_eq!(parse_hex_color("#FFFFFF00"), Some([255, 255, 255, 0]));
        assert_eq!(parse_hex_color("#11223344"), Some([17, 34, 51, 68]));

        // 8 digits, lowercase & mixed
        assert_eq!(parse_hex_color("#1a2b3c4d"), Some([26, 43, 60, 77]));
        assert_eq!(parse_hex_color("#1A2b3C4d"), Some([26, 43, 60, 77]));

        // Without hash
        assert_eq!(parse_hex_color("000000"), Some([0, 0, 0, 255]));
        assert_eq!(parse_hex_color("000000FF"), Some([0, 0, 0, 255]));
        assert_eq!(parse_hex_color("aBcDeF"), Some([171, 205, 239, 255]));
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

        // With multiple hashes (since strip_prefix only removes one leading match)
        assert_eq!(parse_hex_color("##FFFFFF"), None);

        // Non-ASCII characters
        assert_eq!(parse_hex_color("#FF🚀000"), None); // Emoji
        assert_eq!(parse_hex_color("##ññññññ"), None); // Non-ascii characters
    }
}
