#![no_std]
#![no_main]

mod sys;

use sys::*;
use volatile::Volatile;
use core::hint::black_box;

/// 4 KB max input size
const MAX_STDIN_SIZE: usize = 1024 * 4;

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) {
    // Get whole program from STDin
    // Limited to MAX_STDIN_SIZE
    let mut input: [u8; MAX_STDIN_SIZE] = [0; MAX_STDIN_SIZE];
    let mut input_index: usize = 0;

    write_str_slice("BF IN:\n");

    let new_character_buffer: [u8; 1] = black_box([0; 1]);
    
    loop {
        read_from_std_in(new_character_buffer.as_ptr(), 1);

        let new_character: u8 = unsafe { *new_character_buffer.get_unchecked(0) };

        match new_character {
            0 => break, // No change?
            4 => break, // EOF
            _ => {}
        }

        input[input_index] = new_character;

        input_index += 1;
    }

    write_to_std_out(input.as_ptr(), MAX_STDIN_SIZE);

    exit(0);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
