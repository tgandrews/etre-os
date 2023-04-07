#![no_std]
#![no_main]

mod panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world! From EtreOS: {}", 1.0 / 2.0);

    panic!("Oh dear something went wrong!");

    loop {}
}
