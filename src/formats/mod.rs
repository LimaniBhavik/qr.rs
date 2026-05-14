use crate::error::QRError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContactData {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub organization: String,
    pub website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WifiEncryption {
    #[default]
    WPA,
    WEP,
    Nopass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WifiData {
    pub ssid: String,
    pub password: String,
    pub encryption: WifiEncryption,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
}

fn get_or_init_regex(
    lock: &'static OnceLock<Result<Regex, String>>,
    pattern: &str,
) -> Result<&'static Regex, QRError> {
    lock.get_or_init(|| Regex::new(pattern).map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|e| QRError::GenerationError(format!("Invalid regex pattern: {}", e)))
}

fn is_valid_email(email: &str) -> Result<bool, QRError> {
    if email.len() > 254 {
        return Ok(false);
    }
    static EMAIL_REGEX: OnceLock<Result<Regex, String>> = OnceLock::new();
    let regex = get_or_init_regex(
        &EMAIL_REGEX,
        r"^[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$",
    )?;
    Ok(regex.is_match(email))
}

fn is_valid_phone(phone: &str) -> Result<bool, QRError> {
    // Ensure the phone number contains at least one digit and isn't just symbols/whitespace
    if !phone.chars().any(|c| c.is_ascii_digit()) {
        return Ok(false);
    }
    static PHONE_REGEX: OnceLock<Result<Regex, String>> = OnceLock::new();
    let regex = get_or_init_regex(&PHONE_REGEX, r"^\+?[0-9\s\-()\.]{7,20}$")?;
    Ok(regex.is_match(phone))
}

fn is_valid_url(url: &str) -> Result<bool, QRError> {
    static URL_REGEX: OnceLock<Result<Regex, String>> = OnceLock::new();
    let regex = get_or_init_regex(&URL_REGEX, r"^(?:https?://)?[\w-]+(?:\.[\w-]+)+(?:/.*)?$")?;
    Ok(regex.is_match(url))
}

impl ContactData {
    pub fn validate(&self) -> Result<(), QRError> {
        if self.email.is_empty()
            && self.phone.is_empty()
            && self.first_name.is_empty()
            && self.last_name.is_empty()
            && self.organization.is_empty()
            && self.website.is_empty()
        {
            return Err(QRError::VCardError {
                field: "all".to_string(),
                reason: "Contact data cannot be empty".to_string(),
            });
        }

        if !self.email.is_empty() && !is_valid_email(&self.email)? {
            return Err(QRError::VCardError {
                field: "email".to_string(),
                reason: "Invalid email format".to_string(),
            });
        }

        if !self.phone.is_empty() && !is_valid_phone(&self.phone)? {
            return Err(QRError::VCardError {
                field: "phone".to_string(),
                reason: "Invalid phone number format".to_string(),
            });
        }

        if !self.website.is_empty() && !is_valid_url(&self.website)? {
            return Err(QRError::VCardError {
                field: "website".to_string(),
                reason: "Invalid website URL format".to_string(),
            });
        }

        Ok(())
    }
}

impl WifiData {
    pub fn validate(&self) -> Result<(), QRError> {
        if self.ssid.is_empty() {
            return Err(QRError::InvalidData("SSID cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl LocationData {
    pub fn validate(&self) -> Result<(), QRError> {
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(QRError::InvalidData(
                "Latitude must be between -90 and 90".to_string(),
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(QRError::InvalidData(
                "Longitude must be between -180 and 180".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QRData {
    URL(String),
    Text(String),
    Contact(ContactData),
    Wifi(WifiData),
    Location(LocationData),
}

pub fn format_url(url: &str) -> Cow<'_, str> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Cow::Borrowed("");
    }

    let bytes = trimmed.as_bytes();
    let is_http = bytes.len() >= 7 && bytes[..7].eq_ignore_ascii_case(b"http://");
    let is_https = bytes.len() >= 8 && bytes[..8].eq_ignore_ascii_case(b"https://");

    if is_http || is_https {
        Cow::Borrowed(trimmed)
    } else {
        Cow::Owned(format!("https://{}", trimmed))
    }
}

const fn build_escape_vcard_table() -> [u8; 256] {
    let mut table = [0; 256];
    table[b'\\' as usize] = 1;
    table[b',' as usize] = 1;
    table[b';' as usize] = 1;
    table[b':' as usize] = 1;
    table[b'\n' as usize] = 1;
    table[b'\r' as usize] = 1;
    table
}

const VCARD_ESCAPE_TABLE: [u8; 256] = build_escape_vcard_table();

fn escape_vcard_value_to(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let mut search_slice = bytes;

    while let Some(idx) = search_slice
        .iter()
        .position(|&b| VCARD_ESCAPE_TABLE[b as usize] != 0)
    {
        let i = last_pos + idx;
        out.push_str(&s[last_pos..i]);
        let b = bytes[i];
        let escaped = match b {
            b'\\' => "\\\\",
            b',' => "\\,",
            b';' => "\\;",
            b':' => "\\:",
            b'\n' => "\\n",
            b'\r' => "\\r",
            _ => unreachable!(),
        };
        out.push_str(escaped);
        last_pos = i + 1;
        search_slice = &bytes[last_pos..];
    }
    out.push_str(&s[last_pos..]);
}

pub fn generate_vcard(contact: &ContactData) -> String {
    let mut vcard = String::with_capacity(256);
    vcard.push_str("BEGIN:VCARD\nVERSION:3.0\n");

    if !contact.first_name.is_empty() || !contact.last_name.is_empty() {
        vcard.push_str("FN:");
        escape_vcard_value_to(&contact.first_name, &mut vcard);
        vcard.push(' ');
        escape_vcard_value_to(&contact.last_name, &mut vcard);

        if vcard.ends_with(' ') {
            vcard.pop();
        }
        vcard.push('\n');

        vcard.push_str("N:");
        escape_vcard_value_to(&contact.last_name, &mut vcard);
        vcard.push(';');
        escape_vcard_value_to(&contact.first_name, &mut vcard);
        vcard.push_str(";;;\n");
    }

    if !contact.organization.is_empty() {
        vcard.push_str("ORG:");
        escape_vcard_value_to(&contact.organization, &mut vcard);
        vcard.push('\n');
    }

    if !contact.phone.is_empty() {
        vcard.push_str("TEL:");
        escape_vcard_value_to(&contact.phone, &mut vcard);
        vcard.push('\n');
    }

    if !contact.email.is_empty() {
        vcard.push_str("EMAIL:");
        escape_vcard_value_to(&contact.email, &mut vcard);
        vcard.push('\n');
    }

    let trimmed_website = contact.website.trim();
    if !trimmed_website.is_empty() {
        vcard.push_str("URL:");
        let bytes = trimmed_website.as_bytes();
        let is_http = bytes.len() >= 7 && bytes[..7].eq_ignore_ascii_case(b"http://");
        let is_https = bytes.len() >= 8 && bytes[..8].eq_ignore_ascii_case(b"https://");

        if !is_http && !is_https {
            // "https://" escaped is "https\://"
            vcard.push_str("https\\://");
        }
        escape_vcard_value_to(trimmed_website, &mut vcard);
        vcard.push('\n');
    }

    vcard.push_str("END:VCARD");
    vcard
}

const fn build_escape_wifi_table() -> [u8; 256] {
    let mut table = [0; 256];
    table[b'\\' as usize] = 1;
    table[b';' as usize] = 1;
    table[b',' as usize] = 1;
    table[b':' as usize] = 1;
    table[b'\"' as usize] = 1;
    table
}

const WIFI_ESCAPE_TABLE: [u8; 256] = build_escape_wifi_table();

fn escape_wifi_string_to(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let mut search_slice = bytes;

    while let Some(idx) = search_slice
        .iter()
        .position(|&b| WIFI_ESCAPE_TABLE[b as usize] != 0)
    {
        let i = last_pos + idx;
        out.push_str(&s[last_pos..i]);
        let b = bytes[i];
        let escaped = match b {
            b'\\' => "\\\\",
            b';' => "\\;",
            b',' => "\\,",
            b':' => "\\:",
            b'\"' => "\\\"",
            _ => unreachable!(),
        };
        out.push_str(escaped);
        last_pos = i + 1;
        search_slice = &bytes[last_pos..];
    }
    out.push_str(&s[last_pos..]);
}

pub fn generate_wifi(wifi: &WifiData) -> String {
    let mut out = String::with_capacity(128);
    let encryption = match wifi.encryption {
        WifiEncryption::WPA => "WPA",
        WifiEncryption::WEP => "WEP",
        WifiEncryption::Nopass => "nopass",
    };
    let hidden = if wifi.hidden { "true" } else { "false" };

    out.push_str("WIFI:T:");
    out.push_str(encryption);
    out.push_str(";S:");
    escape_wifi_string_to(&wifi.ssid, &mut out);
    out.push_str(";P:");
    escape_wifi_string_to(&wifi.password, &mut out);
    out.push_str(";H:");
    out.push_str(hidden);
    out.push_str(";;");
    out
}

pub fn generate_geo_uri(location: &LocationData) -> String {
    format!("geo:{},{}", location.latitude, location.longitude)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_url() {
        // Standard cases
        assert_eq!(format_url("example.com"), "https://example.com");
        assert_eq!(format_url("https://example.com"), "https://example.com");
        assert_eq!(format_url("http://example.com"), "http://example.com");

        // Edge cases - Empty and whitespace
        assert_eq!(format_url(""), "");
        assert_eq!(format_url("   "), "");
        assert_eq!(format_url("  example.com  "), "https://example.com");

        // Edge cases - Protocol case-insensitivity and trimming
        assert_eq!(format_url("HTTP://example.com"), "HTTP://example.com");
        assert_eq!(format_url("Https://example.com"), "Https://example.com");
        assert_eq!(format_url("  http://example.com  "), "http://example.com");
    }

    #[test]
    fn test_vcard_newline_escaping() {
        let contact = ContactData {
            first_name: "John\nTEL:555-0199".to_string(),
            last_name: "Doe\r\nORG:Injection".to_string(),
            ..Default::default()
        };

        let vcard = generate_vcard(&contact);
        // "John\nTEL:555-0199" -> "John\\nTEL\\:555-0199"
        // "Doe\r\nORG:Injection" -> "Doe\\r\\nORG\\:Injection"
        assert!(vcard.contains("FN:John\\nTEL\\:555-0199 Doe\\r\\nORG\\:Injection"));
        assert!(!vcard.contains("\nTEL:"));
        assert!(!vcard.contains("\r\nORG:"));
    }

    #[test]
    fn test_vcard_generation() {
        let contact = ContactData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: "+1234567890".to_string(),
            organization: "ACME Corp".to_string(),
            website: "example.com".to_string(),
        };

        let vcard = generate_vcard(&contact);
        assert!(vcard.contains("BEGIN:VCARD"));
        assert!(vcard.contains("FN:John Doe"));
        assert!(vcard.contains("URL:https\\://example.com"));
    }

    #[test]
    fn test_vcard_escaping() {
        let contact = ContactData {
            first_name: "John, Jr.".to_string(),
            last_name: "Doe;Smith".to_string(),
            email: "john:smith@example.com".to_string(),
            phone: "+1234567890".to_string(),
            organization: "ACME\\Corp".to_string(),
            website: "example.com".to_string(),
        };

        let vcard = generate_vcard(&contact);
        assert!(vcard.contains("BEGIN:VCARD"));
        assert!(vcard.contains("FN:John\\, Jr. Doe\\;Smith"));
        assert!(vcard.contains("N:Doe\\;Smith;John\\, Jr.;;;"));
        assert!(vcard.contains("ORG:ACME\\\\Corp"));
        assert!(vcard.contains("EMAIL:john\\:smith@example.com"));
    }

    #[test]
    fn test_vcard_unicode() {
        let contact = ContactData {
            first_name: "Jürgen".to_string(),
            last_name: "Müller, Sr.".to_string(),
            organization: "🏢 Unicode Corp".to_string(),
            ..Default::default()
        };
        let vcard = generate_vcard(&contact);
        // Note: Comma in "Müller, Sr." should be escaped to "Müller\, Sr."
        assert!(vcard.contains("FN:Jürgen Müller\\, Sr."));
        assert!(vcard.contains("N:Müller\\, Sr.;Jürgen;;;"));
        assert!(vcard.contains("ORG:🏢 Unicode Corp"));
    }

    #[test]
    fn test_vcard_partial_data() {
        // Only first name
        let only_first = ContactData {
            first_name: "John".to_string(),
            ..Default::default()
        };
        let vcard_only_first = generate_vcard(&only_first);
        // Trailing space after "John" should be removed by pop()
        assert!(vcard_only_first.contains("FN:John\n"));
        assert!(vcard_only_first.contains("N:;John;;;\n"));

        // Only last name
        let only_last = ContactData {
            last_name: "Doe".to_string(),
            ..Default::default()
        };
        let vcard_only_last = generate_vcard(&only_last);
        // Note: Currently pushes " " + last_name if first_name is empty
        assert!(vcard_only_last.contains("FN: Doe\n"));
        assert!(vcard_only_last.contains("N:Doe;;;;\n"));
    }

    #[test]
    fn test_vcard_optional_fields() {
        // Test with only organization
        let contact1 = ContactData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            organization: "ACME Corp".to_string(),
            ..Default::default()
        };
        let vcard1 = generate_vcard(&contact1);
        assert!(vcard1.contains("ORG:ACME Corp\n"));
        assert!(!vcard1.contains("TEL:"));
        assert!(!vcard1.contains("EMAIL:"));
        assert!(!vcard1.contains("URL:"));

        // Test with only phone and email
        let contact2 = ContactData {
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            phone: "+1234567890".to_string(),
            email: "jane@example.com".to_string(),
            ..Default::default()
        };
        let vcard2 = generate_vcard(&contact2);
        assert!(!vcard2.contains("ORG:"));
        assert!(vcard2.contains("TEL:+1234567890\n"));
        assert!(vcard2.contains("EMAIL:jane@example.com\n"));
        assert!(!vcard2.contains("URL:"));

        // Test with website, which requires URL formatting
        let contact3 = ContactData {
            first_name: "Alice".to_string(),
            last_name: "Wonder".to_string(),
            website: "alice.com".to_string(),
            ..Default::default()
        };
        let vcard3 = generate_vcard(&contact3);
        assert!(!vcard3.contains("ORG:"));
        assert!(!vcard3.contains("TEL:"));
        assert!(!vcard3.contains("EMAIL:"));
        assert!(vcard3.contains("URL:https\\://alice.com\n"));

        // Test with everything absent except names
        let contact4 = ContactData {
            first_name: "Bob".to_string(),
            last_name: "Builder".to_string(),
            ..Default::default()
        };
        let vcard4 = generate_vcard(&contact4);
        assert!(!vcard4.contains("ORG:"));
        assert!(!vcard4.contains("TEL:"));
        assert!(!vcard4.contains("EMAIL:"));
        assert!(!vcard4.contains("URL:"));
    }

    #[test]
    fn test_vcard_empty_names() {
        let empty_names = ContactData {
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let vcard = generate_vcard(&empty_names);
        // VERSION:3.0 contains "N:", so we check for "\nN:" to ensure the N field isn't present
        assert!(!vcard.contains("\nFN:"));
        assert!(!vcard.contains("\nN:"));
        assert!(vcard.contains("EMAIL:test@example.com"));
    }

    #[test]
    fn test_is_valid_phone() {
        // Valid phone numbers
        assert!(is_valid_phone("+1234567890").unwrap());
        assert!(is_valid_phone("1234567890").unwrap());
        assert!(is_valid_phone("+44 20 7946 0958").unwrap());
        assert!(is_valid_phone("0123-456-789").unwrap());
        assert!(is_valid_phone("(012) 345-6789").unwrap());
        assert!(is_valid_phone("123.456.7890").unwrap());
        assert!(is_valid_phone("+1.123.456.7890").unwrap());
        assert!(is_valid_phone("1234567").unwrap()); // Exactly 7 chars (minimum)
        assert!(is_valid_phone("12345678901234567890").unwrap()); // Exactly 20 chars (maximum)

        // Invalid phone numbers
        assert!(!is_valid_phone("123456").unwrap()); // Too short (6 chars)
        assert!(!is_valid_phone("123456789012345678901").unwrap()); // Too long (21 chars)
        assert!(!is_valid_phone("phone123").unwrap()); // Contains letters
        assert!(!is_valid_phone("+12345+678").unwrap()); // Multiple +
        assert!(!is_valid_phone("++123456789").unwrap()); // Multiple leading +
        assert!(!is_valid_phone("123456789+").unwrap()); // Trailing +
        assert!(!is_valid_phone("123 456 789 012 345 678 901").unwrap()); // Too long with spaces
        assert!(!is_valid_phone("").unwrap()); // Empty
        assert!(!is_valid_phone("       ").unwrap()); // Only spaces
        assert!(!is_valid_phone(".......").unwrap()); // Only dots
        assert!(!is_valid_phone("-------").unwrap()); // Only hyphens
        assert!(!is_valid_phone("()()()()").unwrap()); // Only parentheses
        assert!(!is_valid_phone("!@#$%^&*").unwrap()); // Special characters not allowed
    }

    #[test]
    fn test_is_valid_url() {
        // Valid URLs
        assert!(is_valid_url("google.com").expect("Regex should compile"));
        assert!(is_valid_url("www.google.com").expect("Regex should compile"));
        assert!(is_valid_url("https://example.com").expect("Regex should compile"));
        assert!(is_valid_url("http://example.com/path").expect("Regex should compile"));
        assert!(is_valid_url("example.com/path?query=1#fragment").expect("Regex should compile"));
        assert!(is_valid_url("sub.domain.co.uk").expect("Regex should compile"));
        assert!(is_valid_url("a.b").expect("Regex should compile"));

        // Invalid URLs
        assert!(!is_valid_url("localhost").expect("Regex should compile")); // Requires at least one dot
        assert!(!is_valid_url("google.").expect("Regex should compile"));
        assert!(!is_valid_url(".google").expect("Regex should compile"));
        assert!(!is_valid_url("http://").expect("Regex should compile"));
        assert!(!is_valid_url("https://.com").expect("Regex should compile"));

        // ReDoS protection test
        let evil_url = format!("{}!", "a.".repeat(100));
        assert!(!is_valid_url(&evil_url).expect("Regex should compile"));

        let evil_url_2 = format!("{}!", "a".repeat(100));
        assert!(!is_valid_url(&evil_url_2).expect("Regex should compile"));
    }

    #[test]
    fn test_is_valid_email() {
        // Valid emails - Standard
        // Standard valid emails
        assert!(is_valid_email("user@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user.name@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user.name@sub.domain.co.uk").expect("Regex should compile")); // Added from issue description
        assert!(is_valid_email("user+tag@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user-name@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user_name@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user@localhost").expect("Regex should compile"));
        assert!(is_valid_email("user@123.123.123.123").expect("Regex should compile"));
        assert!(is_valid_email("user@sub.example.com").expect("Regex should compile"));
        assert!(is_valid_email("user@example.co.uk").expect("Regex should compile"));
        assert!(is_valid_email("1234567890@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user@example-one.com").expect("Regex should compile"));
        assert!(is_valid_email("_______@example.com").expect("Regex should compile"));
        assert!(is_valid_email("u@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user.name.with.dots@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user!name@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user#name@example.com").expect("Regex should compile"));

        // Extended valid edge cases
        assert!(is_valid_email("valid-local-part@domain.com").expect("Regex should compile"));
        assert!(is_valid_email("valid_local_part@domain.com").expect("Regex should compile"));
        assert!(is_valid_email("12345678901234567890@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user@a.com").expect("Regex should compile")); // short domain
        assert!(is_valid_email("user@domain.solutions").expect("Regex should compile")); // long TLD
        assert!(is_valid_email("user@domain-with-dash.com").expect("Regex should compile")); // domain with dash
        assert!(is_valid_email("a@b.c").expect("Regex should compile")); // extremely short

        // Valid emails - Edge cases (Complex characters in local part)
        assert!(is_valid_email("!#$%&'*+-/=?^_`{|}~@example.com").expect("Regex should compile"));
        assert!(is_valid_email("user%name@example.com").expect("Regex should compile"));

        // Valid emails - Edge cases (Domain segments length)
        // 63 character domain segment
        let long_domain = format!("user@{}.com", "a".repeat(63));
        assert!(is_valid_email(&long_domain).expect("Regex should compile"));

        // Complex multi-subdomains
        assert!(
            is_valid_email("user.name.with.dots@sub.domain.co.uk").expect("Regex should compile")
        );
        assert!(is_valid_email("user@sub.sub.sub.domain.com").expect("Regex should compile"));

        // Invalid emails - Missing parts
        assert!(!is_valid_email("plainaddress").expect("Regex should compile"));
        assert!(!is_valid_email("@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email.example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email@").expect("Regex should compile"));
        assert!(!is_valid_email("user@.com").expect("Regex should compile"));

        // Invalid emails - Structure/Syntax issues
        assert!(!is_valid_email("email@example@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("Joe Smith <email@example.com>").expect("Regex should compile"));
        assert!(!is_valid_email("email@example.com (Joe Smith)").expect("Regex should compile"));
        assert!(!is_valid_email("あいうえお@example.com").expect("Regex should compile")); // Non-ASCII

        // Invalid emails - Consecutive or leading/trailing dots
        assert!(!is_valid_email(".email@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email.@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email..email@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email@example..com").expect("Regex should compile"));
        assert!(!is_valid_email("Abc..123@example.com").expect("Regex should compile"));

        // Invalid emails - Hyphen position in domain
        assert!(!is_valid_email("email@-example.com").expect("Regex should compile"));
        assert!(!is_valid_email("email@example-.com").expect("Regex should compile"));
        assert!(!is_valid_email("email@example.-com").expect("Regex should compile"));
        assert!(!is_valid_email("email@example.com-").expect("Regex should compile"));

        // Invalid emails - Domain segments length
        // 64 character domain segment (exceeds the maximum 63 permitted by the regex length limit {0,61})
        // "a" + 62 times "a" + "a" = 64 characters. Limit is "a" + {0,61} + "a" = 63 max characters.
        let too_long_domain = format!("user@{}.com", "a".repeat(64));
        assert!(!is_valid_email(&too_long_domain).expect("Regex should compile"));

        // Invalid emails - Spaces
        assert!(!is_valid_email(" user@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user @example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@ example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@example.com ").expect("Regex should compile"));
        assert!(!is_valid_email("user name@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@exam ple.com").expect("Regex should compile"));

        // Invalid emails - Quotes (current regex does not permit quotes)
        assert!(!is_valid_email("\"user\"@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("\"user.name\"@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("\" \"@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user..name@example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@.example.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@example.com.").expect("Regex should compile"));
        assert!(!is_valid_email("user@example_domain.com").expect("Regex should compile"));
        assert!(!is_valid_email("user@example com").expect("Regex should compile")); // Space in domain
        assert!(!is_valid_email("user@example.").expect("Regex should compile")); // Trailing dot in domain

        // Extended invalid edge cases
        assert!(!is_valid_email("").expect("Regex should compile")); // empty
        assert!(!is_valid_email(" ").expect("Regex should compile")); // whitespace
        assert!(!is_valid_email("user@do_main.com").expect("Regex should compile")); // underscore in domain
        assert!(!is_valid_email(".user@domain.com").expect("Regex should compile")); // leading dot in local part
        assert!(!is_valid_email("user.@domain.com").expect("Regex should compile")); // trailing dot in local part
        assert!(!is_valid_email("user..name@domain.com").expect("Regex should compile")); // consecutive dots in local part
        assert!(!is_valid_email("user@domain.-com").expect("Regex should compile")); // TLD starts with dash
        assert!(!is_valid_email("user@domain.com-").expect("Regex should compile")); // TLD ends with dash
        assert!(!is_valid_email("user@domain-.com").expect("Regex should compile")); // Domain ends with dash

        // very long local part (assuming 64 chars limit isn't strictly enforced by the regex but let's see,
        // actually the current regex doesn't seem to bound the local part length.
        // It does bound domain labels to 63 chars `{0,61}` inside `(?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?`

        // Valid length label
        let valid_label = "a".repeat(63);
        assert!(is_valid_email(&format!("user@{}.com", valid_label)).expect("Regex should compile"));

        // Invalid length label (> 63 characters)
        let invalid_label = "a".repeat(64);
        assert!(
            !is_valid_email(&format!("user@{}.com", invalid_label)).expect("Regex should compile")
        );

        // Length limit (max 254 chars)
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(!is_valid_email(&long_email).expect("Regex should compile"));

        // ReDoS protection test
        let evil_email = format!("user@{}!", "a.".repeat(100));
        assert!(!is_valid_email(&evil_email).expect("Regex should compile"));
    }

    #[test]
    fn test_contact_data_validation() {
        // Valid contact
        let contact = ContactData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: "+1234567890".to_string(),
            website: "example.com".to_string(),
            ..Default::default()
        };
        assert!(contact.validate().is_ok());

        // Empty contact (invalid)
        let empty_contact = ContactData::default();
        assert!(empty_contact.validate().is_err());

        // Invalid email
        let invalid_email = ContactData {
            email: "invalid-email".to_string(),
            ..contact.clone()
        };
        assert!(invalid_email.validate().is_err());

        // Invalid phone
        let invalid_phone = ContactData {
            phone: "not-a-phone".to_string(),
            ..contact.clone()
        };
        assert!(invalid_phone.validate().is_err());

        // Invalid website
        let invalid_website = ContactData {
            website: "not-a-url".to_string(),
            ..contact.clone()
        };
        assert!(invalid_website.validate().is_err());

        // Website without protocol (should be valid as format_url handles it)
        let no_proto_website = ContactData {
            website: "example.com".to_string(),
            ..contact.clone()
        };
        assert!(no_proto_website.validate().is_ok());

        // Website with protocol
        let proto_website = ContactData {
            website: "https://example.com/test".to_string(),
            ..contact.clone()
        };
        assert!(proto_website.validate().is_ok());

        // Valid international phone
        let intl_phone = ContactData {
            phone: "+44 20 7946 0958".to_string(),
            ..contact.clone()
        };
        assert!(intl_phone.validate().is_ok());
    }

    #[test]
    fn test_wifi_generation() {
        let wifi = WifiData {
            ssid: "MyNetwork".to_string(),
            password: "mypassword".to_string(),
            encryption: WifiEncryption::WPA,
            hidden: false,
        };
        let wifi_string = generate_wifi(&wifi);
        assert_eq!(wifi_string, "WIFI:T:WPA;S:MyNetwork;P:mypassword;H:false;;");

        let wifi_nopass = WifiData {
            ssid: "FreeWifi".to_string(),
            password: "".to_string(),
            encryption: WifiEncryption::Nopass,
            hidden: true,
        };
        let wifi_nopass_string = generate_wifi(&wifi_nopass);
        assert_eq!(wifi_nopass_string, "WIFI:T:nopass;S:FreeWifi;P:;H:true;;");
    }

    #[test]
    fn test_wifi_validation() {
        let valid_wifi = WifiData {
            ssid: "MyNetwork".to_string(),
            password: "mypassword".to_string(),
            encryption: WifiEncryption::WPA,
            hidden: false,
        };
        assert!(valid_wifi.validate().is_ok());

        let invalid_wifi = WifiData {
            ssid: "".to_string(),
            password: "mypassword".to_string(),
            encryption: WifiEncryption::WPA,
            hidden: false,
        };
        let result = invalid_wifi.validate();
        assert!(result.is_err());
        match result {
            Err(QRError::InvalidData(msg)) => assert_eq!(msg, "SSID cannot be empty"),
            _ => panic!("Expected QRError::InvalidData error"),
        }
    }

    #[test]
    fn test_wifi_escaping() {
        let wifi = WifiData {
            ssid: "My;Network".to_string(),
            password: "pass\\word:123".to_string(),
            encryption: WifiEncryption::WPA,
            hidden: false,
        };
        let wifi_string = generate_wifi(&wifi);
        // "My;Network" -> "My\;Network"
        // "pass\word:123" -> "pass\\word\:123"
        assert_eq!(
            wifi_string,
            "WIFI:T:WPA;S:My\\;Network;P:pass\\\\word\\:123;H:false;;"
        );
    }

    #[test]
    fn test_location_generation() {
        let location = LocationData {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        let geo_uri = generate_geo_uri(&location);
        assert_eq!(geo_uri, "geo:40.7128,-74.006");
    }

    #[test]
    fn test_location_validation() {
        let valid_location = LocationData {
            latitude: 45.0,
            longitude: 90.0,
        };
        assert!(valid_location.validate().is_ok());

        let invalid_lat = LocationData {
            latitude: 91.0,
            longitude: 0.0,
        };
        assert!(invalid_lat.validate().is_err());

        let invalid_lon = LocationData {
            latitude: 0.0,
            longitude: -181.0,
        };
        assert!(invalid_lon.validate().is_err());
    }
}
