use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn escape_vcard_value_to_current(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        let b = unsafe { *bytes.get_unchecked(i) };
        if matches!(b, b'\\' | b',' | b';' | b':' | b'\n' | b'\r') {
            out.push_str(unsafe {
                std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..i))
            });
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
        }
        i += 1;
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..len)) });
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

fn escape_vcard_value_to_optimized(s: &str, out: &mut String) {
    let mut last_pos = 0;
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        let b = unsafe { *bytes.get_unchecked(i) };
        if VCARD_ESCAPE_TABLE[b as usize] != 0 {
            out.push_str(unsafe {
                std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..i))
            });
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
        }
        i += 1;
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(bytes.get_unchecked(last_pos..len)) });
}

fn criterion_benchmark(c: &mut Criterion) {
    let test_strings = vec![
        "NormalStringWithoutEscapes",
        "String;With:Some\\Escapes,And\nNewlines",
        "Multiple;;;;::::\\\\\\\\,,,,,,,,\r\r\n\n",
        "LongStringThatIsMostlyNormalButHasSomeEscapesAtTheEndLikeThisOne;",
    ];

    let mut group = c.benchmark_group("escape_vcard_string");

    for s in &test_strings {
        group.bench_function(format!("current_{}", s), |b| {
            b.iter(|| {
                let mut out = String::with_capacity(128);
                escape_vcard_value_to_current(black_box(s), &mut out);
                black_box(out);
            })
        });

        group.bench_function(format!("optimized_{}", s), |b| {
            b.iter(|| {
                let mut out = String::with_capacity(128);
                escape_vcard_value_to_optimized(black_box(s), &mut out);
                black_box(out);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
