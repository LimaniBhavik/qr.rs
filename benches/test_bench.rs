use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn escape_wifi_while_matches(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        let b = unsafe { *bytes.get_unchecked(i) };
        if matches!(b, b'\\' | b';' | b',' | b':' | b'\"') {
            out.push_str(unsafe {
                std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..i))
            });
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
        }
        i += 1;
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..len)) });
}

fn escape_wifi_while_table(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        let b = unsafe { *bytes.get_unchecked(i) };
        if WIFI_ESCAPE_TABLE[b as usize] != 0 {
            out.push_str(unsafe {
                std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..i))
            });
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
        }
        i += 1;
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..len)) });
}

fn escape_wifi_iter_table(s: &str, out: &mut String) {
    let bytes = s.as_bytes();
    let mut last_pos = 0;

    for (i, &b) in bytes.iter().enumerate() {
        if WIFI_ESCAPE_TABLE[b as usize] != 0 {
            let escaped = match b {
                b'\\' => "\\\\",
                b';' => "\\;",
                b',' => "\\,",
                b':' => "\\:",
                b'\"' => "\\\"",
                _ => unreachable!(),
            };
            out.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[last_pos..i]) });
            out.push_str(escaped);
            last_pos = i + 1;
        }
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[last_pos..]) });
}

fn criterion_benchmark(c: &mut Criterion) {
    let test_strings = vec![
        "NormalStringWithoutEscapes",
        "String;With:Some\\Escapes,And\"Quotes\"",
        "Multiple;;;;::::\\\\\\\\,,,,,,,,",
        "LongStringThatIsMostlyNormalButHasSomeEscapesAtTheEndLikeThisOne;",
    ];

    let mut group = c.benchmark_group("escape_methods");

    for s in &test_strings {
        group.bench_function(format!("while_matches_{}", s), |b| {
            b.iter(|| {
                let mut out = String::with_capacity(128);
                escape_wifi_while_matches(black_box(s), &mut out);
                black_box(out);
            })
        });

        group.bench_function(format!("while_table_{}", s), |b| {
            b.iter(|| {
                let mut out = String::with_capacity(128);
                escape_wifi_while_table(black_box(s), &mut out);
                black_box(out);
            })
        });

        group.bench_function(format!("iter_table_{}", s), |b| {
            b.iter(|| {
                let mut out = String::with_capacity(128);
                escape_wifi_iter_table(black_box(s), &mut out);
                black_box(out);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
