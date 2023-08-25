#![no_std]
#![no_main]

mod sys;

use sys::*;

/// 4 KB max input size
const MAX_STDIN_SIZE: usize = 1024 * 4;
const MAX_TAPE_SIZE: u16 = 2 ^ 16;

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) {
    // Get whole program from STDin
    // Limited to MAX_STDIN_SIZE
    let input: [u8; MAX_STDIN_SIZE] = [0; MAX_STDIN_SIZE];
    read_from_std_in(input.as_ptr(), MAX_STDIN_SIZE);

    /*write_str_slice("\n\n\nBF:");

    write_to_std_out(input.as_ptr(), MAX_STDIN_SIZE);

    write_str_slice("\n\n\nEXE:");*/

    // Main loop
    let mut current_input_index: usize = 0;
    let mut current_tape_pointer: u16 = 0;
    loop {
        let current_char: u8 = unsafe { *input.get_unchecked(current_input_index) };

        match current_char {
            b'>' => {current_tape_pointer += 1},
            b'<' => {current_tape_pointer -= 1},
            b'+' => todo!(),
            b'-' => todo!(),
            b'.' => todo!(),
            b',' => todo!(),
            b'[' => todo!(),
            b']' => todo!(),
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
