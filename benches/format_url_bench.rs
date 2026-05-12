use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Original implementation
pub fn format_url_original(url: &str) -> String {
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

use std::borrow::Cow;

// Optimized implementation
pub fn format_url_optimized(url: &str) -> Cow<'_, str> {
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

fn bench_format_url(c: &mut Criterion) {
    let test_urls = vec![
        "example.com",
        "http://example.com",
        "https://example.com",
        "HTTP://example.com",
        "HTTPS://example.com",
        "   https://spaced-example.com   ",
        "short",
        "",
        "http:/missing-slash.com",
        "hTtPs://mixed-case.com",
    ];

    let mut group = c.benchmark_group("format_url");

    for url in test_urls {
        group.bench_function(format!("original_{}", url), |b| {
            b.iter(|| {
                black_box(format_url_original(black_box(url)));
            })
        });

        group.bench_function(format!("optimized_{}", url), |b| {
            b.iter(|| {
                black_box(format_url_optimized(black_box(url)));
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_format_url);
criterion_main!(benches);
