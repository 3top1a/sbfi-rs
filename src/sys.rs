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

pub fn error(string: &'static str) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // write syscall number
            in("rdi") 2, // stdout file descriptor, 2 is stderr
            in("rsi") string.as_ptr(),
            in("rdx") string.len(),
        );
    }
    exit(1);
}

pub fn write_str_slice(string: &str) {
    write_to_std_out(string.as_ptr(), string.len())
}

use core::fmt;
pub struct Writer;
// https://degaart.github.io/20230123.html
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        write_str_slice(s);
        Ok(())
    }
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

pub fn read_from_std_in_until_EOF(output_pointer: *const u8, output_length: usize) {
    let mut x: u64 = 1;

    /// TODO Reads only last line

    while x != 0 {
        /*{
            let tmp = [x as u8; 1];
            write_to_std_out(tmp.as_ptr(), 1);
        }*/

        unsafe {
            asm!(
                "syscall",
                in("rax") 0, // read syscall number
                in("rdi") 0,
                in("rsi") output_pointer,
                in("rdx") output_length,
            );

            asm!("mov {}, rax", out(reg) x);
        }
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
