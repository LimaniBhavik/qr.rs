use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::formats::{generate_vcard, ContactData};

fn bench_vcard_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vcard_generation");

    let contact_with_prefix = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        phone: "1234567890".to_string(),
        email: "john.doe@example.com".to_string(),
        organization: "Acme Corp".to_string(),
        website: "https://example.com".to_string(),
    };

    let contact_without_prefix = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        phone: "1234567890".to_string(),
        email: "john.doe@example.com".to_string(),
        organization: "Acme Corp".to_string(),
        website: "example.com".to_string(),
    };

    group.bench_function("with_prefix", |b| {
        b.iter(|| generate_vcard(black_box(&contact_with_prefix)))
    });

    group.bench_function("without_prefix", |b| {
        b.iter(|| generate_vcard(black_box(&contact_without_prefix)))
    });

    group.finish();
}

criterion_group!(benches, bench_vcard_generation);
criterion_main!(benches);
