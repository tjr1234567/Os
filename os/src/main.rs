#![no_std]
#![no_main]
use core::panic::PanicInfo;

fn main() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[no_mangle]
