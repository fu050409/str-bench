#![allow(unused)]
use compact_str::CompactString;

const FRAGMENT: &str = "t";
const SMALL_SIZE: usize = 24;
const LARGE_SIZE: usize = 1024;

fn main() {
    let mut s = CompactString::default();
    for _ in 0..SMALL_SIZE {
        s.push_str(FRAGMENT);
    }
    println!("Is heap allocated: {}", s.is_heap_allocated());
    println!("{}", s)
}
