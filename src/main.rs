#![no_std] //disables the std library
#![no_main] //disable all rust-level entry points

// This function is called on panic
use core::panic::PanicInfo;

//This function handles panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] //don't change the name of this function
pub extern "C" fn _start() -> ! {
    //This is the entry point function
    loop {}
}
