#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{count_syscall, trace, trace_read, trace_write};

const SYS_TRACE: usize = 410;

// Extra regression test for ch3 trace:
// - invalid trace_request should fail
// - SYS_TRACE count should include trace queries themselves
// - trace_write should write exactly one byte
#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    assert_eq!(-1, trace(99, 0, 0));

    let before = count_syscall(SYS_TRACE);
    let after = count_syscall(SYS_TRACE);
    assert!(after > before);

    let var = 0u8;
    assert_eq!(Some(0), trace_read(&var as *const u8));

    assert_eq!(0, trace_write(&var as *const u8, 0xab));
    assert_eq!(Some(0xab), trace_read(&var as *const u8));

    assert_eq!(0, trace_write(&var as *const u8, 0xff));
    assert_eq!(Some(0xff), trace_read(&var as *const u8));

    println!("Test ch3_trace_extra OK!");
    0
}
