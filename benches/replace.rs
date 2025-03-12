use criterion::{Criterion, black_box, criterion_group, criterion_main};

use arcstr::ArcStr;
use compact_str::CompactString;
use ecow::EcoString;
use imstr::ImString;
use smartstring::alias::String as SmartString;
use std::borrow::Cow;

const SMALL_SIZE: usize = 5;
const LARGE_SIZE: usize = 1024;
const FRAGMENT: &str = "test";

//
// ====================
// 字符串替换测试（使用 replace 方法，将 "test" 替换为 "TEST"）
// ====================
//

fn string_replace_small(c: &mut Criterion) {
    let base = FRAGMENT.repeat(SMALL_SIZE);
    c.bench_function("String replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn string_replace_large(c: &mut Criterion) {
    let base = FRAGMENT.repeat(LARGE_SIZE);
    c.bench_function("String replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn compact_string_replace_small(c: &mut Criterion) {
    let base = CompactString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("CompactString replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn compact_string_replace_large(c: &mut Criterion) {
    let base = CompactString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("CompactString replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn smart_string_replace_small(c: &mut Criterion) {
    let base = SmartString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("SmartString replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn smart_string_replace_large(c: &mut Criterion) {
    let base = SmartString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("SmartString replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn eco_string_replace_small(c: &mut Criterion) {
    let base = EcoString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("EcoString replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn eco_string_replace_large(c: &mut Criterion) {
    let base = EcoString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("EcoString replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn arcstr_replace_small(c: &mut Criterion) {
    let base = ArcStr::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ArcStr replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn arcstr_replace_large(c: &mut Criterion) {
    let base = ArcStr::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ArcStr replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn imstr_replace_small(c: &mut Criterion) {
    let base = ImString::from(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("ImString replace small", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn imstr_replace_large(c: &mut Criterion) {
    let base = ImString::from(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("ImString replace large", |b| {
        b.iter(|| {
            let result = base.replace("test", "TEST");
            black_box(result)
        })
    });
}

fn cow_replace_small(c: &mut Criterion) {
    let base: Cow<str> = Cow::Owned(FRAGMENT.repeat(SMALL_SIZE));
    c.bench_function("Cow replace small", |b| {
        b.iter(|| {
            let result = base.clone().into_owned().replace("test", "TEST");
            black_box(result)
        })
    });
}

fn cow_replace_large(c: &mut Criterion) {
    let base: Cow<str> = Cow::Owned(FRAGMENT.repeat(LARGE_SIZE));
    c.bench_function("Cow replace large", |b| {
        b.iter(|| {
            let result = base.clone().into_owned().replace("test", "TEST");
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    string_replace_small,
    string_replace_large,
    compact_string_replace_small,
    compact_string_replace_large,
    smart_string_replace_small,
    smart_string_replace_large,
    eco_string_replace_small,
    eco_string_replace_large,
    arcstr_replace_small,
    arcstr_replace_large,
    imstr_replace_small,
    imstr_replace_large,
    cow_replace_small,
    cow_replace_large,
);
criterion_main!(benches);
