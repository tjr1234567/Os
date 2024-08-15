#![no_std]
#![no_main]
use core::panic::PanicInfo;

//tackle the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

//entry point
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop{}
}