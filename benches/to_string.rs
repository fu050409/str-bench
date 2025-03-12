use criterion::{Criterion, black_box, criterion_group, criterion_main};

use arcstr::ArcStr;
use compact_str::CompactString;
use ecow::EcoString;
use imstr::ImString;
use smartstring::alias::String as SmartString;
use std::borrow::Cow;

const SMALL_SIZE: usize = 22;
const LARGE_SIZE: usize = 1024;
const FRAGMENT: &str = "t";

fn compact_string_to_string_small(c: &mut Criterion) {
    let s = CompactString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("CompactString to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn compact_string_to_string_large(c: &mut Criterion) {
    let s = CompactString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("CompactString to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn smart_string_to_string_small(c: &mut Criterion) {
    let s = SmartString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("SmartString to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn smart_string_to_string_large(c: &mut Criterion) {
    let s = SmartString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("SmartString to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn eco_string_to_string_small(c: &mut Criterion) {
    let s = EcoString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("EcoString to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn eco_string_to_string_large(c: &mut Criterion) {
    let s = EcoString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("EcoString to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn arcstr_to_string_small(c: &mut Criterion) {
    let s = ArcStr::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ArcStr to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn arcstr_to_string_large(c: &mut Criterion) {
    let s = ArcStr::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ArcStr to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn imstr_to_string_small(c: &mut Criterion) {
    let s = ImString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ImString to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn imstr_to_string_large(c: &mut Criterion) {
    let s = ImString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ImString to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn cow_to_string_small(c: &mut Criterion) {
    let s: Cow<str> = Cow::Owned(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("Cow to_string small", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

fn cow_to_string_large(c: &mut Criterion) {
    let s: Cow<str> = Cow::Owned(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("Cow to_string large", |b| {
        b.iter(|| {
            let cloned = s.to_string();
            black_box(cloned)
        })
    });
}

criterion_group!(
    benches,
    compact_string_to_string_small,
    compact_string_to_string_large,
    smart_string_to_string_small,
    smart_string_to_string_large,
    eco_string_to_string_small,
    eco_string_to_string_large,
    arcstr_to_string_small,
    arcstr_to_string_large,
    imstr_to_string_small,
    imstr_to_string_large,
    cow_to_string_small,
    cow_to_string_large,
);
criterion_main!(benches);
