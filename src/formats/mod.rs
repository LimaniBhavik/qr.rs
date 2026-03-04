use crate::error::QRError;
use regex::Regex;
use serde::{Deserialize, Serialize};
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

fn is_valid_email(email: &str) -> bool {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").expect("Invalid email regex pattern")
        Regex::new(r"^[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$")
            .expect("Failed to compile email validation regex")
    }).is_match(email)
}

fn is_valid_phone(phone: &str) -> bool {
    static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();
    PHONE_REGEX
        .get_or_init(|| {
            Regex::new(r"^\+?[0-9\s\-()\.]{7,20}$")
                .expect("Failed to compile phone validation regex")
        })
        .is_match(phone)
}

fn is_valid_url(url: &str) -> bool {
    static URL_REGEX: OnceLock<Regex> = OnceLock::new();
    URL_REGEX
        .get_or_init(|| {
            Regex::new(r"^(https?://)?([\w\d-]+\.)+[\w\d-]+(/.*)?$")
                .expect("Failed to compile URL validation regex")
        })
        .is_match(url)
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

        if !self.email.is_empty() && !is_valid_email(&self.email) {
            return Err(QRError::VCardError {
                field: "email".to_string(),
                reason: "Invalid email format".to_string(),
            });
        }

        if !self.phone.is_empty() && !is_valid_phone(&self.phone) {
            return Err(QRError::VCardError {
                field: "phone".to_string(),
                reason: "Invalid phone number format".to_string(),
            });
        }

        if !self.website.is_empty() && !is_valid_url(&self.website) {
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

pub fn format_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.to_lowercase().starts_with("http://")
        || trimmed.to_lowercase().starts_with("https://")
    {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

fn escape_vcard_value_to(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            ',' => out.push_str("\\,"),
            ';' => out.push_str("\\;"),
            ':' => out.push_str("\\:"),
            _ => out.push(c),
        }
    }
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

    if !contact.website.is_empty() {
        vcard.push_str("URL:");
        escape_vcard_value_to(&format_url(&contact.website), &mut vcard);
        vcard.push('\n');
    }

    vcard.push_str("END:VCARD");
    vcard
}

fn escape_wifi_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace(':', "\\:")
        .replace('"', "\\\"")
}

pub fn generate_wifi(wifi: &WifiData) -> String {
    let encryption = match wifi.encryption {
        WifiEncryption::WPA => "WPA",
        WifiEncryption::WEP => "WEP",
        WifiEncryption::Nopass => "nopass",
    };
    let hidden = if wifi.hidden { "true" } else { "false" };
    format!(
        "WIFI:T:{};S:{};P:{};H:{};;",
        encryption,
        escape_wifi_string(&wifi.ssid),
        escape_wifi_string(&wifi.password),
        hidden
    )
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
        assert_eq!(format_url("example.com"), "https://example.com");
        assert_eq!(format_url("https://example.com"), "https://example.com");
        assert_eq!(format_url("http://example.com"), "http://example.com");
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
    fn test_is_valid_email() {
        // Valid emails - Standard
        // Standard valid emails
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name@example.com"));
        assert!(is_valid_email("user.name@sub.domain.co.uk")); // Added from issue description
        assert!(is_valid_email("user+tag@example.com"));
        assert!(is_valid_email("user.name@sub.domain.co.uk"));
        assert!(is_valid_email("user-name@example.com"));
        assert!(is_valid_email("user_name@example.com"));
        assert!(is_valid_email("user@localhost"));
        assert!(is_valid_email("user@123.123.123.123"));
        assert!(is_valid_email("user@sub.example.com"));
        assert!(is_valid_email("user@example.co.uk"));
        assert!(is_valid_email("1234567890@example.com"));
        assert!(is_valid_email("user@example-one.com"));
        assert!(is_valid_email("_______@example.com"));
        assert!(is_valid_email("u@example.com"));
        assert!(is_valid_email("user.name.with.dots@example.com"));
        assert!(is_valid_email("user-name@example.com"));
        assert!(is_valid_email("user_name@example.com"));
        assert!(is_valid_email("user!name@example.com"));
        assert!(is_valid_email("user#name@example.com"));

        // Extended valid edge cases
        assert!(is_valid_email("user.name@sub.domain.co.uk"));
        assert!(is_valid_email("valid-local-part@domain.com"));
        assert!(is_valid_email("valid_local_part@domain.com"));
        assert!(is_valid_email("12345678901234567890@example.com"));
        assert!(is_valid_email("user@a.com")); // short domain
        assert!(is_valid_email("user@domain.solutions")); // long TLD
        assert!(is_valid_email("user@domain-with-dash.com")); // domain with dash
        assert!(is_valid_email("a@b.c")); // extremely short

        // Valid emails - Edge cases (Complex characters in local part)
        assert!(is_valid_email("!#$%&'*+-/=?^_`{|}~@example.com"));
        assert!(is_valid_email("user-name@example.com"));
        assert!(is_valid_email("user_name@example.com"));
        assert!(is_valid_email("user%name@example.com"));

        // Valid emails - Edge cases (Domain segments length)
        // 63 character domain segment
        let long_domain = format!("user@{}.com", "a".repeat(63));
        assert!(is_valid_email(&long_domain));

        // Complex multi-subdomains
        assert!(is_valid_email("user.name.with.dots@sub.domain.co.uk"));
        assert!(is_valid_email("user@sub.sub.sub.domain.com"));

        // Invalid emails - Missing parts
        assert!(!is_valid_email("plainaddress"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("email.example.com"));
        assert!(!is_valid_email("email@"));
        assert!(!is_valid_email("user@.com"));

        // Invalid emails - Structure/Syntax issues
        assert!(!is_valid_email("email@example@example.com"));
        assert!(!is_valid_email("Joe Smith <email@example.com>"));
        assert!(!is_valid_email("email@example.com (Joe Smith)"));
        assert!(!is_valid_email("あいうえお@example.com")); // Non-ASCII

        // Invalid emails - Consecutive or leading/trailing dots
        assert!(!is_valid_email(".email@example.com"));
        assert!(!is_valid_email("email.@example.com"));
        assert!(!is_valid_email("email..email@example.com"));
        assert!(!is_valid_email("email@example..com"));
        assert!(!is_valid_email("Abc..123@example.com"));

        // Invalid emails - Hyphen position in domain
        assert!(!is_valid_email("email@-example.com"));
        assert!(!is_valid_email("email@example-.com"));
        assert!(!is_valid_email("email@example.-com"));
        assert!(!is_valid_email("email@example.com-"));

        // Invalid emails - Domain segments length
        // 64 character domain segment (exceeds the maximum 63 permitted by the regex length limit {0,61})
        // "a" + 62 times "a" + "a" = 64 characters. Limit is "a" + {0,61} + "a" = 63 max characters.
        let too_long_domain = format!("user@{}.com", "a".repeat(64));
        assert!(!is_valid_email(&too_long_domain));

        // Invalid emails - Spaces
        assert!(!is_valid_email(" user@example.com"));
        assert!(!is_valid_email("user @example.com"));
        assert!(!is_valid_email("user@ example.com"));
        assert!(!is_valid_email("user@example.com "));
        assert!(!is_valid_email("user name@example.com"));
        assert!(!is_valid_email("user@exam ple.com"));

        // Invalid emails - Quotes (current regex does not permit quotes)
        assert!(!is_valid_email("\"user\"@example.com"));
        assert!(!is_valid_email("\"user.name\"@example.com"));
        assert!(!is_valid_email("\" \"@example.com"));
        assert!(!is_valid_email("user..name@example.com"));
        assert!(!is_valid_email("user@.example.com"));
        assert!(!is_valid_email("user@example.com."));
        assert!(!is_valid_email("user@example_domain.com"));
        assert!(!is_valid_email(".email@example.com")); // Leading dot in local part
        assert!(!is_valid_email("email.@example.com")); // Trailing dot in local part
        assert!(!is_valid_email("email..email@example.com")); // Consecutive dots in local part
        assert!(!is_valid_email("あいうえお@example.com"));
        assert!(!is_valid_email("email@example.com (Joe Smith)"));
        assert!(!is_valid_email("email@-example.com"));
        assert!(!is_valid_email("email@example..com")); // Consecutive dots in domain
        assert!(!is_valid_email("Abc..123@example.com"));
        assert!(!is_valid_email("user name@example.com")); // Space in local part
        assert!(!is_valid_email("user@example com")); // Space in domain
        assert!(!is_valid_email("user@")); // Missing domain
        assert!(!is_valid_email("user@.com")); // Missing domain name
        assert!(!is_valid_email("user@example.")); // Trailing dot in domain

        // Extended invalid edge cases
        assert!(!is_valid_email("")); // empty
        assert!(!is_valid_email(" ")); // whitespace
        assert!(!is_valid_email("user@")); // missing domain
        assert!(!is_valid_email("user@.com")); // domain starts with dot
        assert!(!is_valid_email("user@domain.com.")); // domain ends with dot
        assert!(!is_valid_email("user@domain..com")); // consecutive dots in domain
        assert!(!is_valid_email("user@do_main.com")); // underscore in domain
        assert!(!is_valid_email(".user@domain.com")); // leading dot in local part
        assert!(!is_valid_email("user.@domain.com")); // trailing dot in local part
        assert!(!is_valid_email("user..name@domain.com")); // consecutive dots in local part
        assert!(!is_valid_email("user@domain.-com")); // TLD starts with dash
        assert!(!is_valid_email("user@domain.com-")); // TLD ends with dash
        assert!(!is_valid_email("user@domain-.com")); // Domain ends with dash
        assert!(!is_valid_email("user@-domain.com")); // Domain starts with dash

        // very long local part (assuming 64 chars limit isn't strictly enforced by the regex but let's see,
        // actually the current regex doesn't seem to bound the local part length.
        // It does bound domain labels to 63 chars `{0,61}` inside `(?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?`

        // Valid length label
        let valid_label = "a".repeat(63);
        assert!(is_valid_email(&format!("user@{}.com", valid_label)));

        // Invalid length label (> 63 characters)
        let invalid_label = "a".repeat(64);
        assert!(!is_valid_email(&format!("user@{}.com", invalid_label)));
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

    #[test]
    fn test_is_valid_url() {
        // Valid URLs
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("example.com"));
        assert!(is_valid_url("www.example.com"));
        assert!(is_valid_url("sub.example.com"));
        assert!(is_valid_url("example.com/path"));
        assert!(is_valid_url("example.com/path?query=1"));
        assert!(is_valid_url("example.com/path#fragment"));
        assert!(is_valid_url("127.0.0.1"));
        assert!(is_valid_url("my-domain.com"));

        // Invalid URLs
        assert!(!is_valid_url("ftp://example.com")); // only http/https or no protocol
        assert!(!is_valid_url("http:/example.com"));
        assert!(!is_valid_url("https//example.com"));
        assert!(!is_valid_url(".example.com"));
        assert!(!is_valid_url("example."));
        assert!(!is_valid_url("http://"));
        assert!(!is_valid_url("localhost")); // currently regex requires at least one dot
        assert!(!is_valid_url(""));
    }
}
