use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
struct ContactState {
    first_name: AttrValue,
    last_name: AttrValue,
    phone: AttrValue,
    email: AttrValue,
    organization: AttrValue,
    website: AttrValue,
}

fn bench_state_cloning(c: &mut Criterion) {
    let mut group = c.benchmark_group("yew_state_cloning");

    // Simulate what the old code does
    let attr = AttrValue::from("a moderately long string for testing purposes");
    let contact = ContactState {
        first_name: attr.clone(),
        last_name: attr.clone(),
        phone: attr.clone(),
        email: attr.clone(),
        organization: attr.clone(),
        website: attr.clone(),
    };

    // Simulate what a UseStateHandle would contain (Rc)
    let rc_attr = Rc::new(attr.clone());
    let rc_contact = Rc::new(contact.clone());

    group.bench_function("clone_inner_values", |b| {
        b.iter(|| {
            // This is roughly what let val = (*state).clone() does
            let c1 = black_box(&contact).clone();
            let a1 = black_box(&attr).clone();
            let a2 = black_box(&attr).clone();
            let a3 = black_box(&attr).clone();
            let a4 = black_box(&attr).clone();
            let a5 = black_box(&attr).clone();
            (c1, a1, a2, a3, a4, a5)
        })
    });

    group.bench_function("clone_rc_handles", |b| {
        b.iter(|| {
            // This is what url_input.clone() does (it clones the UseStateHandle which is essentially an Rc clone)
            let c1 = black_box(&rc_contact).clone();
            let a1 = black_box(&rc_attr).clone();
            let a2 = black_box(&rc_attr).clone();
            let a3 = black_box(&rc_attr).clone();
            let a4 = black_box(&rc_attr).clone();
            let a5 = black_box(&rc_attr).clone();
            (c1, a1, a2, a3, a4, a5)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_state_cloning);
criterion_main!(benches);
