use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::formats::{generate_wifi, WifiData, WifiEncryption};

fn bench_wifi_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("wifi_generation");

    let wifi_simple = WifiData {
        ssid: "MyNetwork".to_string(),
        password: "mypassword".to_string(),
        encryption: WifiEncryption::WPA,
        hidden: false,
    };

    let wifi_complex = WifiData {
        ssid: "My;Network\\".to_string(),
        password: "pass\\word:123\",;".to_string(),
        encryption: WifiEncryption::WPA,
        hidden: true,
    };

    group.bench_function("simple", |b| {
        b.iter(|| generate_wifi(black_box(&wifi_simple)))
    });

    group.bench_function("complex_escaping", |b| {
        b.iter(|| generate_wifi(black_box(&wifi_complex)))
    });

    group.finish();
}

criterion_group!(benches, bench_wifi_generation);
criterion_main!(benches);
