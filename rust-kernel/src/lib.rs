#![no_std]
#![feature(core_intrinsics, lang_items, panic_internals)]
#![crate_type="staticlib"]

use core::intrinsics::abort;
use core::panic::PanicInfo;

mod gpio;
mod utils;
mod uart;

#[no_mangle]
//THis function to indicate that we are executing the simple kernel
fn blink_led_ok() {
    // setting GPIO16 to 1 which is output
    gpio::gpio_select_mode(18, 0);
    for _ in 0..5 {
        utils::sleepms(500);
        gpio::gpio_clr(16);
        utils::sleepms(500);
        gpio::gpio_set(16);
    }
}

#[no_mangle]
pub extern fn kernel_main() {
   uart::init_uart();
    blink_led_ok();
    loop {
        utils::sleepms(3);
      //  uart::send_uart(0x51);
    }
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() { loop {} }
