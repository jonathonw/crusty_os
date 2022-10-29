#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Here's the number {}, and the number {}", 42, 2.0/3.0);
    panic!("Some panic message");

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga_buffer::ColorCode;
    use crate::vga_buffer::Color;
    vga_buffer::WRITER.lock().change_color(ColorCode::new(Color::LightRed, Color::Black));
    println!("{}", info);
    loop {}
}