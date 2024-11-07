#![no_std]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;
use core::panic::PanicInfo;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use core::arch;

mod gpio;
mod utils;

#[no_mangle]
fn blink_led() {
    // setting GPIO16 to 1 which is output
    gpio::gpio_select_mode(18, 1);
    unsafe { 
        loop {
            utils::sleepms(1000);
            gpio::gpio_clr(16);
            utils::sleepms(1000);
            gpio::gpio_set(16);
        }
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    blink_led();
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
