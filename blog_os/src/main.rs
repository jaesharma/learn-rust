#![no_std] //don't link the rust standard library
#![no_main] //disable all rust level entry points

use core::panic::PanicInfo;
mod vga_buffer;

//this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Wake up neo!";

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //this function is an entry point, since the linker looks for a function
    //name "_start" by default

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
