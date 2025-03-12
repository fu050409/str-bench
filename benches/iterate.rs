use criterion::{Criterion, black_box, criterion_group, criterion_main};

use arcstr::ArcStr;
use compact_str::CompactString;
use ecow::EcoString;
use imstr::ImString;
use smartstring::alias::String as SmartString;
use std::borrow::Cow;

const LARGE_SIZE: usize = 1024;
const FRAGMENT: &str = "test";

//
// ====================
// 字符串 Iteration 测试（遍历字符）
// ====================
//

fn string_iterate(c: &mut Criterion) {
    let s = FRAGMENT.repeat(LARGE_SIZE);
    c.bench_function("String iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn compact_string_iterate(c: &mut Criterion) {
    let s = CompactString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("CompactString iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn smart_string_iterate(c: &mut Criterion) {
    let s = SmartString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("SmartString iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn eco_string_iterate(c: &mut Criterion) {
    let s = EcoString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("EcoString iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn arcstr_iterate(c: &mut Criterion) {
    let s = ArcStr::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ArcStr iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn imstr_iterate(c: &mut Criterion) {
    let s = ImString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ImString iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

fn cow_iterate(c: &mut Criterion) {
    let s: Cow<str> = Cow::Owned(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("Cow iterate", |b| {
        b.iter(|| {
            for ch in s.chars() {
                black_box(ch);
            }
        })
    });
}

criterion_group!(
    benches,
    string_iterate,
    compact_string_iterate,
    smart_string_iterate,
    eco_string_iterate,
    arcstr_iterate,
    imstr_iterate,
    cow_iterate,
);
criterion_main!(benches);
