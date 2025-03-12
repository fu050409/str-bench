use criterion::{Criterion, black_box, criterion_group, criterion_main};

use compact_str::CompactString;
use ecow::EcoString;
use imstr::ImString;
use smartstring::alias::String as SmartString;

const SMALL_SIZE: usize = 22;
const LARGE_SIZE: usize = 1024;
const FRAGMENT: &str = "t";

//
// ====================
// 可变字符串测试（Push 操作等）
// ====================
//

// 标准 String 的 push 测试
fn string_push_small(c: &mut Criterion) {
    c.bench_function("String push small", |b| {
        b.iter(|| {
            let mut s = String::new();
            for _ in 0..SMALL_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

fn string_push_large(c: &mut Criterion) {
    c.bench_function("String push large", |b| {
        b.iter(|| {
            let mut s = String::new();
            for _ in 0..LARGE_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

// CompactString 的 push 测试
fn compact_string_push_small(c: &mut Criterion) {
    c.bench_function("CompactString push small", |b| {
        b.iter(|| {
            let mut s = CompactString::default();
            for _ in 0..SMALL_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

fn compact_string_push_large(c: &mut Criterion) {
    c.bench_function("CompactString push large", |b| {
        b.iter(|| {
            let mut s = CompactString::default();
            for _ in 0..LARGE_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

// SmartString 的 push 测试
fn smart_string_push_small(c: &mut Criterion) {
    c.bench_function("SmartString push small", |b| {
        b.iter(|| {
            let mut s = SmartString::new();
            for _ in 0..SMALL_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

fn smart_string_push_large(c: &mut Criterion) {
    c.bench_function("SmartString push large", |b| {
        b.iter(|| {
            let mut s = SmartString::new();
            for _ in 0..LARGE_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

// EcoString 的 push 测试
fn eco_string_push_small(c: &mut Criterion) {
    c.bench_function("EcoString push small", |b| {
        b.iter(|| {
            let mut s = EcoString::new();
            for _ in 0..SMALL_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

fn eco_string_push_large(c: &mut Criterion) {
    c.bench_function("EcoString push large", |b| {
        b.iter(|| {
            let mut s = EcoString::new();
            for _ in 0..LARGE_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

// ImString 的 push 测试
fn imstr_push_small(c: &mut Criterion) {
    c.bench_function("ImString push small", |b| {
        b.iter(|| {
            let mut s = ImString::new();
            for _ in 0..SMALL_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

fn imstr_push_large(c: &mut Criterion) {
    c.bench_function("ImString push large", |b| {
        b.iter(|| {
            let mut s = ImString::new();
            for _ in 0..LARGE_SIZE {
                s.push_str(black_box(FRAGMENT));
            }
            black_box(s)
        })
    });
}

criterion_group!(
    benches,
    string_push_small,
    string_push_large,
    compact_string_push_small,
    compact_string_push_large,
    smart_string_push_small,
    smart_string_push_large,
    eco_string_push_small,
    eco_string_push_large,
    imstr_push_small,
    imstr_push_large,
);
criterion_main!(benches);
