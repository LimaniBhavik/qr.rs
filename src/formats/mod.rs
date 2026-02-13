use serde::{Deserialize, Serialize};
use crate::error::QRError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContactData {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub organization: String,
    pub website: String,
}

impl ContactData {
    pub fn validate(&self) -> Result<(), QRError> {
        if self.email.is_empty() && self.phone.is_empty() && self.first_name.is_empty() && self.last_name.is_empty() && self.organization.is_empty() && self.website.is_empty() {
             return Err(QRError::VCardError {
                field: "all".to_string(),
                reason: "Contact data cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QRData {
    URL(String),
    Text(String),
    Contact(ContactData),
}

pub fn format_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.to_lowercase().starts_with("http://") || trimmed.to_lowercase().starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

pub fn generate_vcard(contact: &ContactData) -> String {
    let mut vcard = vec!["BEGIN:VCARD".to_string(), "VERSION:3.0".to_string()];

    if !contact.first_name.is_empty() || !contact.last_name.is_empty() {
        vcard.push(format!("FN:{} {}", contact.first_name, contact.last_name).trim().to_string());
        vcard.push(format!("N:{};{};;;", contact.last_name, contact.first_name));
    }

    if !contact.organization.is_empty() {
        vcard.push(format!("ORG:{}", contact.organization));
    }

    if !contact.phone.is_empty() {
        vcard.push(format!("TEL:{}", contact.phone));
    }

    if !contact.email.is_empty() {
        vcard.push(format!("EMAIL:{}", contact.email));
    }

    if !contact.website.is_empty() {
        vcard.push(format!("URL:{}", format_url(&contact.website)));
    }

    vcard.push("END:VCARD".to_string());
    vcard.join("\n")
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
        assert!(vcard.contains("URL:https://example.com"));
    }
}
