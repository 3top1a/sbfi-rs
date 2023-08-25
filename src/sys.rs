use core::arch::asm;

/// Write to stdout
/// See smallest-hello-rs
pub fn write_to_std_out(string_pointer: *const u8, string_length: usize) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // write syscall number
            in("rdi") 1, // stdout file descriptor, 2 is stderr
            in("rsi") string_pointer,
            in("rdx") string_length,
        );
    }
}

pub fn write_str_slice(string: &'static str) {
    write_to_std_out(string.as_ptr(), string.len())
}

/// Read from stdin
/// ```rust
/// const X: [u8; 20] = [0; 20];
/// read_from_std_in(X.as_ptr(), 20);
/// ```
pub fn read_from_std_in(output_pointer: *const u8, output_length: usize) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 0, // read syscall number
            in("rdi") 0,
            in("rsi") output_pointer,
            in("rdx") output_length,
        );
    }
}

/// Exit nicely
pub fn exit(code: i32) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // exit syscall number
            in("rdi") code,
            options(noreturn),
        );
    }
}
