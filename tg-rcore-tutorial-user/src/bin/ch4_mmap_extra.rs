#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mmap, munmap, trace_read, trace_write};

const PAGE_SIZE: usize = 4096;
const PROT_READ: usize = 1;
const PROT_WRITE: usize = 2;

// Extra regression test for ch4 mmap/munmap:
// - overlapping mmap should fail
// - readonly mmap should reject trace_write
// - munmap should make the range inaccessible to trace
#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    let start: usize = 0x11000000;
    let len: usize = PAGE_SIZE;

    assert_eq!(0, mmap(start, len, PROT_READ | PROT_WRITE));
    assert_eq!(-1, mmap(start, len, PROT_READ | PROT_WRITE));

    assert_eq!(0, trace_write(start as *const u8, 0x5a));
    assert_eq!(Some(0x5a), trace_read(start as *const u8));

    assert_eq!(0, munmap(start, len));
    assert_eq!(None, trace_read(start as *const u8));
    assert_eq!(-1, trace_write(start as *const u8, 0));

    let readonly_start: usize = 0x12000000;
    assert_eq!(0, mmap(readonly_start, len, PROT_READ));
    assert!(trace_read(readonly_start as *const u8).is_some());
    assert_eq!(-1, trace_write(readonly_start as *const u8, 0x33));
    assert_eq!(0, munmap(readonly_start, len));

    println!("Test ch4_mmap_extra OK!");
    0
}
