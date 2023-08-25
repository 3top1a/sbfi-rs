#![no_std]
#![no_main]

mod sys;

use sc::syscall;
use sys::*;

/// 4 KB max input size
const MAX_STDIN_SIZE: usize = 1024 * 4;
/// 2^16 Cells
const MAX_TAPE_SIZE: usize = 2usize.pow(16);

extern "C" {
    fn write(fildes: i32, buf: *const core::ffi::c_void, nbyte: usize);
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) {
    // Get whole program from STDin
    // Limited to MAX_STDIN_SIZE
    let input: [u8; MAX_STDIN_SIZE] = [0; MAX_STDIN_SIZE];
    read_from_std_in_until_EOF(input.as_ptr(), MAX_STDIN_SIZE);


    // Main loop
    let mut current_input_index: usize = 0;
    let mut current_tape_pointer: usize = 0;
    let mut tape: [u8; MAX_TAPE_SIZE] = [0; MAX_TAPE_SIZE];
    loop {
        let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
        let current_cell_value: u8 = unsafe { *tape.get_unchecked(current_tape_pointer) };

        /*unsafe {
            let sc: [u8; 1] = [current_char];
            syscall!(WRITE, 1, sc.as_ptr(), 1);
        }*/

        match current_char {
            b'>' => {
                current_tape_pointer += 1;
            }
            b'<' => {
                current_tape_pointer -= 1;
            }
            b'+' => unsafe {
                *(tape.as_mut_ptr().wrapping_add(current_tape_pointer)) =
                current_cell_value.wrapping_add(1);
            },
            b'-' => unsafe {
                *(tape.as_mut_ptr().wrapping_add(current_tape_pointer)) =
                current_cell_value.wrapping_sub(1);
            },
            b'.' => unsafe {
                let tmp_buffer: [u8; 1] = [current_cell_value];
                syscall!(WRITE, 1, tmp_buffer.as_ptr(), 1);
            },
            b',' => error("TODO implement,"),
            b'[' => {
                let mut bracket_depth: u8 = 0;
                while current_cell_value == 0 {
                    current_input_index += 1;

                    let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
                    match current_char {
                        b']' => {
                            match bracket_depth {
                                0 => break,
                                _ => {bracket_depth -= 1}
                            }
                        }
                        b'[' => {
                            bracket_depth += 1
                        }
                        _ => {}
                    }
                }
                current_tape_pointer += 1; //fixme
            },
            b']' => {
                let mut bracket_depth: u8 = 0;
                while current_cell_value != 0 {
                    current_input_index -= 1;

                    let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
                    match current_char {
                        b'[' => {
                            match bracket_depth {
                                0 => break,
                                _ => {bracket_depth -= 1}
                            }
                        }
                        b']' => {
                            bracket_depth += 1
                        }
                        _ => {}
                    }
                }
                current_tape_pointer -= 1; //fixme
            },
            0 => break,
            _ => {}
        }

        current_input_index += 1;
    }

    exit(0);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
