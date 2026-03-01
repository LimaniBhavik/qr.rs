import re

with open('src/formats/mod.rs', 'r') as f:
    content = f.read()

test_is_valid_email_regex = re.compile(r'#\[test\]\s+fn test_is_valid_email\(\) \{.*?\n    \}', re.DOTALL)

new_test_fn = """#[test]
    fn test_is_valid_email() {
        // Standard valid emails
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name@example.com"));
        assert!(is_valid_email("user+tag@example.com"));
        assert!(is_valid_email("user@sub.example.com"));
        assert!(is_valid_email("user@example.co.uk"));
        assert!(is_valid_email("1234567890@example.com"));
        assert!(is_valid_email("user@example-one.com"));
        assert!(is_valid_email("_______@example.com"));
        assert!(is_valid_email("u@example.com"));

        // Extended valid edge cases
        assert!(is_valid_email("user.name@sub.domain.co.uk"));
        assert!(is_valid_email("valid-local-part@domain.com"));
        assert!(is_valid_email("valid_local_part@domain.com"));
        assert!(is_valid_email("12345678901234567890@example.com"));
        assert!(is_valid_email("user@a.com")); // short domain
        assert!(is_valid_email("user@domain.solutions")); // long TLD
        assert!(is_valid_email("user@domain-with-dash.com")); // domain with dash
        assert!(is_valid_email("a@b.c")); // extremely short

        // Invalid emails
        assert!(!is_valid_email("plainaddress"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("Joe Smith <email@example.com>"));
        assert!(!is_valid_email("email.example.com"));
        assert!(!is_valid_email("email@example@example.com"));
        assert!(!is_valid_email(".email@example.com"));
        assert!(!is_valid_email("email.@example.com"));
        assert!(!is_valid_email("email..email@example.com"));
        assert!(!is_valid_email("あいうえお@example.com"));
        assert!(!is_valid_email("email@example.com (Joe Smith)"));
        assert!(!is_valid_email("email@-example.com"));
        assert!(!is_valid_email("email@example..com"));
        assert!(!is_valid_email("Abc..123@example.com"));

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
    }"""

new_content = test_is_valid_email_regex.sub(new_test_fn, content)

with open('src/formats/mod.rs', 'w') as f:
    f.write(new_content)
