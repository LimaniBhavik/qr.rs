use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct ContactData {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub organization: String,
    pub website: String,
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

fn escape_vcard_string_old(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(',', "\\,")
        .replace(';', "\\;")
        .replace(':', "\\:")
}

pub fn generate_vcard_old(contact: &ContactData) -> String {
    let mut vcard = vec!["BEGIN:VCARD".to_string(), "VERSION:3.0".to_string()];

    if !contact.first_name.is_empty() || !contact.last_name.is_empty() {
        let first_name = escape_vcard_string_old(&contact.first_name);
        let last_name = escape_vcard_string_old(&contact.last_name);
        vcard.push(
            format!("FN:{} {}", first_name, last_name)
                .trim()
                .to_string(),
        );
        vcard.push(format!("N:{};{};;;", last_name, first_name));
    }

    if !contact.organization.is_empty() {
        vcard.push(format!(
            "ORG:{}",
            escape_vcard_string_old(&contact.organization)
        ));
    }

    if !contact.phone.is_empty() {
        vcard.push(format!("TEL:{}", escape_vcard_string_old(&contact.phone)));
    }

    if !contact.email.is_empty() {
        vcard.push(format!("EMAIL:{}", escape_vcard_string_old(&contact.email)));
    }

    if !contact.website.is_empty() {
        vcard.push(format!(
            "URL:{}",
            escape_vcard_string_old(&format_url(&contact.website))
        ));
    }

    vcard.push("END:VCARD".to_string());
    vcard.join("\n")
}

fn escape_vcard_string_to(s: &str, out: &mut String) {
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

pub fn generate_vcard_new(contact: &ContactData) -> String {
    let mut vcard = String::with_capacity(256);
    vcard.push_str("BEGIN:VCARD\nVERSION:3.0\n");

    if !contact.first_name.is_empty() || !contact.last_name.is_empty() {
        vcard.push_str("FN:");
        escape_vcard_string_to(&contact.first_name, &mut vcard);
        vcard.push(' ');
        escape_vcard_string_to(&contact.last_name, &mut vcard);

        if vcard.ends_with(' ') {
            vcard.pop();
        }
        vcard.push('\n');

        vcard.push_str("N:");
        escape_vcard_string_to(&contact.last_name, &mut vcard);
        vcard.push(';');
        escape_vcard_string_to(&contact.first_name, &mut vcard);
        vcard.push_str(";;;\n");
    }

    if !contact.organization.is_empty() {
        vcard.push_str("ORG:");
        escape_vcard_string_to(&contact.organization, &mut vcard);
        vcard.push('\n');
    }

    if !contact.phone.is_empty() {
        vcard.push_str("TEL:");
        escape_vcard_string_to(&contact.phone, &mut vcard);
        vcard.push('\n');
    }

    if !contact.email.is_empty() {
        vcard.push_str("EMAIL:");
        escape_vcard_string_to(&contact.email, &mut vcard);
        vcard.push('\n');
    }

    if !contact.website.is_empty() {
        vcard.push_str("URL:");
        escape_vcard_string_to(&format_url(&contact.website), &mut vcard);
        vcard.push('\n');
    }

    vcard.push_str("END:VCARD");
    vcard
}

fn main() {
    let contacts = vec![
        ContactData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: "+1234567890".to_string(),
            organization: "ACME Corp".to_string(),
            website: "example.com".to_string(),
        },
        ContactData {
            first_name: "John".to_string(),
            last_name: "".to_string(),
            ..Default::default()
        },
        ContactData {
            first_name: "".to_string(),
            last_name: "Doe".to_string(),
            ..Default::default()
        },
        ContactData {
            first_name: "John; Jr.".to_string(),
            last_name: "Doe, III".to_string(),
            organization: "B\\S".to_string(),
            ..Default::default()
        },
    ];

    // Correctness check
    for contact in &contacts {
        let old_vcard = generate_vcard_old(contact);
        let new_vcard = generate_vcard_new(contact);
        if old_vcard != new_vcard {
            println!("ERROR: Output mismatch for contact: {:?}", contact);
            println!("Old:\n{:?}", old_vcard);
            println!("New:\n{:?}", new_vcard);
            assert_eq!(old_vcard, new_vcard);
        }
    }
    println!("Output matches for all test cases!");

    // Warm up
    for contact in &contacts {
        for _ in 0..1000 {
            let _ = generate_vcard_old(contact);
            let _ = generate_vcard_new(contact);
        }
    }

    let iterations = 100_000;

    let start = Instant::now();
    for _ in 0..iterations {
        for contact in &contacts {
            let _ = generate_vcard_old(contact);
        }
    }
    let duration_old = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        for contact in &contacts {
            let _ = generate_vcard_new(contact);
        }
    }
    let duration_new = start.elapsed();

    println!(
        "Old version (total for {} iterations * {} contacts): {:?}",
        iterations,
        contacts.len(),
        duration_old
    );
    println!(
        "New version (total for {} iterations * {} contacts): {:?}",
        iterations,
        contacts.len(),
        duration_new
    );
    println!(
        "Improvement: {:.2}%",
        (duration_old.as_secs_f64() - duration_new.as_secs_f64()) / duration_old.as_secs_f64()
            * 100.0
    );
}
