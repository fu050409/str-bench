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

//
// ====================
// 字符串 Clone 测试
// ====================
//

fn string_clone_small(c: &mut Criterion) {
    let s = FRAGMENT.repeat(SMALL_SIZE);
    c.bench_function("String clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn string_clone_large(c: &mut Criterion) {
    let s = FRAGMENT.repeat(LARGE_SIZE);
    c.bench_function("String clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn compact_string_clone_small(c: &mut Criterion) {
    let s = CompactString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("CompactString clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn compact_string_clone_large(c: &mut Criterion) {
    let s = CompactString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("CompactString clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn smart_string_clone_small(c: &mut Criterion) {
    let s = SmartString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("SmartString clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn smart_string_clone_large(c: &mut Criterion) {
    let s = SmartString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("SmartString clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn eco_string_clone_small(c: &mut Criterion) {
    let s = EcoString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("EcoString clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn eco_string_clone_large(c: &mut Criterion) {
    let s = EcoString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("EcoString clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn arcstr_clone_small(c: &mut Criterion) {
    let s = ArcStr::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ArcStr clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn arcstr_clone_large(c: &mut Criterion) {
    let s = ArcStr::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ArcStr clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn imstr_clone_small(c: &mut Criterion) {
    let s = ImString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ImString clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn imstr_clone_large(c: &mut Criterion) {
    let s = ImString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ImString clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn cow_clone_small(c: &mut Criterion) {
    let s: Cow<str> = Cow::Owned(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("Cow clone small", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

fn cow_clone_large(c: &mut Criterion) {
    let s: Cow<str> = Cow::Owned(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("Cow clone large", |b| {
        b.iter(|| {
            let cloned = s.clone();
            black_box(cloned)
        })
    });
}

criterion_group!(
    benches,
    string_clone_small,
    string_clone_large,
    compact_string_clone_small,
    compact_string_clone_large,
    smart_string_clone_small,
    smart_string_clone_large,
    eco_string_clone_small,
    eco_string_clone_large,
    arcstr_clone_small,
    arcstr_clone_large,
    imstr_clone_small,
    imstr_clone_large,
    cow_clone_small,
    cow_clone_large,
);
criterion_main!(benches);
