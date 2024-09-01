#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

//tackle the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

//static HELLO: &[u8] = b"TJR is one of the best guies in UESTC";
//entry point
#[no_mangle]
pub extern "C" fn _start() -> !{
    vga_buffer::print_something();
    loop{}
}