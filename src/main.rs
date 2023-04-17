#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod gdt;
mod interrupts;
mod panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...");
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("Hello from EtreOS");
    loop {
        x86_64::instructions::hlt();
    }
}
