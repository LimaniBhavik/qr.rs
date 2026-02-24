pub const DEFAULT_QR_SIZE: u32 = 300;
pub const MIN_QR_SIZE: u32 = 100;
pub const MAX_QR_SIZE: u32 = 2000;
pub const DEFAULT_QUIET_ZONE: u32 = 4;

pub fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }

    let mut rgba = [0, 0, 0, 255];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let chunk_str = std::str::from_utf8(chunk).ok()?;
        rgba[i] = u8::from_str_radix(chunk_str, 16).ok()?;
    }
    Some(rgba)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        // 6 digits
        assert_eq!(parse_hex_color("#FFFFFF"), Some([255, 255, 255, 255]));
        assert_eq!(parse_hex_color("000000"), Some([0, 0, 0, 255]));
        assert_eq!(parse_hex_color("#FF0000"), Some([255, 0, 0, 255]));

        // 8 digits
        assert_eq!(parse_hex_color("#FFFFFF00"), Some([255, 255, 255, 0]));
        assert_eq!(parse_hex_color("000000FF"), Some([0, 0, 0, 255]));
        assert_eq!(parse_hex_color("#11223344"), Some([17, 34, 51, 68]));

        // Invalid
        assert_eq!(parse_hex_color("#FFF"), None);
        assert_eq!(parse_hex_color("GGGGGG"), None);
        assert_eq!(parse_hex_color("#1234567"), None);
        assert_eq!(parse_hex_color(""), None);
    }
}
