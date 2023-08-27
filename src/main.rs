#![no_std]
#![no_main]

mod sys;

use sys::*;

/// 4 KB max input size
const MAX_STDIN_SIZE: usize = 1024 * 4;
/// 2^16 Cells
const MAX_TAPE_SIZE: usize = 2usize.pow(16);

#[cfg(debug_assertions)]
use core::fmt::Write;
#[cfg(debug_assertions)]
macro_rules! dbg_in {
    ($($arg:tt)*) => {{
        Writer{}.write_fmt(format_args!($($arg)*)).unwrap();
    }}
}
#[cfg(debug_assertions)]
macro_rules! dbg {
    ($($arg:tt)*) => {{
        dbg_in!("{}\n", format_args!($($arg)*));
    }}
}
#[cfg(not(debug_assertions))]
macro_rules! dbg {
    ($($arg:tt)*) => {{}};
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) {
    // Get whole program from STDin
    // Limited to MAX_STDIN_SIZE
    let input: [u8; MAX_STDIN_SIZE] = [0; MAX_STDIN_SIZE];
    read_from_std_in_until_eof(input.as_ptr(), MAX_STDIN_SIZE);

    // Main loop
    let mut current_input_index: usize = 0;
    let mut current_tape_pointer: u16 = 0;
    let mut tape: [u8; MAX_TAPE_SIZE] = [0; MAX_TAPE_SIZE];

    let program_length = input.iter().filter(|x| **x != 0).count();
    loop {
        let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
        let current_cell_value: u8 = unsafe { *tape.get_unchecked(current_tape_pointer as usize) };

        match current_char {
            b'>' => {
                dbg!("{}", "> at ");
                dbg!("input: {}", current_input_index);
                dbg!("tape: {}", current_tape_pointer);
                current_tape_pointer += 1;
            }
            b'<' => {
                dbg!("{}", "< at");
                dbg!("input: {}", current_input_index);
                dbg!("tape: {}", current_tape_pointer);
                current_tape_pointer -= 1;
            }
            b'+' => unsafe {
                dbg!("{}", "+ at ");
                dbg!("input: {}", current_input_index);
                dbg!("cell: {}", current_cell_value);
                *(tape
                    .as_mut_ptr()
                    .wrapping_add(current_tape_pointer as usize)) = current_cell_value + 1;
            },
            b'-' => unsafe {
                dbg!("{}", "- at ");
                dbg!("input: {}", current_input_index);
                dbg!("cell: {}", current_cell_value);
                *(tape
                    .as_mut_ptr()
                    .wrapping_add(current_tape_pointer as usize)) = current_cell_value - 1;
            },
            b'.' => {
                write_to_std_out(tape.as_ptr().wrapping_add(current_tape_pointer.into()), 1)
            },
            b',' => read_from_std_in(tape.as_ptr().wrapping_add(current_tape_pointer.into()), 1),
            b'[' => {
                let mut bracket_depth: u8 = 0;

                if current_cell_value == 0 {
                    loop {
                        dbg!("{}", "[ inner loop");
                        dbg!("input: {}", current_input_index);
                        dbg!("cell: {}", current_cell_value);
                        dbg!("bracket: {}", bracket_depth);
                        dbg!("{}", "\n");

                        current_input_index += 1;

                        if current_input_index > program_length {
                            error("E");
                        }

                        let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
                        match current_char {
                            b']' => match bracket_depth {
                                0 => break,
                                _ => bracket_depth -= 1,
                            },
                            b'[' => bracket_depth += 1,
                            _ => {}
                        }
                    }
                }
                //current_tape_pointer += 1;
            }
            b']' => {
                let mut bracket_depth: u8 = 0;

                dbg!("{}", "]");

                if current_cell_value != 0 {
                    loop {
                        current_input_index -= 1;

                        if current_input_index > program_length {
                            error("E");
                        }

                        dbg!("{}", "] inner loop");
                        dbg!("input: {}", current_input_index);
                        dbg!("cell: {}", current_cell_value);
                        dbg!("bracket: {}", bracket_depth);
                        dbg!("{}", "\n");

                        let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };
                        match current_char {
                            b'[' => match bracket_depth {
                                0 => break,
                                _ => bracket_depth -= 1,
                            },
                            b']' => bracket_depth += 1,
                            _ => {}
                        }
                    }
                }
                //current_input_index -= 1;
            }
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
