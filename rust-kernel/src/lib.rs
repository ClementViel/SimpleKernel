#![no_std]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;
use core::panic::PanicInfo;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

const FREQ: u32 = 250_000;
const GPIO_FSEL0: u32 = 0x20200000;
const GPIO_FSEL1: u32 = 0x20200004;
const GPIO_SET0: u32 = 0x2020001C;
const GPIO_SET1: u32 = 0x20200020;
const GPIO_CLR0: u32 = 0x20200028;
const GPIO_CLR1: u32 = 0x2020002C;

fn sleep(time: u32) {
    let mut tick: u32 = 500_000;

    while tick > 0 {
         tick -= 1;
    }
}

#[no_mangle]
fn gpio_select_mode(offset: u32, mode: u32) {
    let gpio_fun_reg = GPIO_FSEL1 as *mut u32;
    unsafe {*(gpio_fun_reg) |= mode << offset }
}

#[no_mangle]
fn gpio_set(gpio: u32) {
    let gpio_to_set = GPIO_SET0 as *mut u32;
    unsafe {*(gpio_to_set) = 1 << gpio }

}

#[no_mangle]
fn gpio_clr(gpio: u32) {
    let gpio_to_clr = GPIO_CLR0 as *mut u32;
    unsafe {*(gpio_to_clr) = 1 << gpio }
   
}

#[no_mangle]
fn blink_led() {
    // setting GPIO16 to 1 which is output
    gpio_select_mode(18, 1);
    loop {
    for i in 0..50000000 {}
    gpio_clr(16);
    for i in 0..50000000 {}
    gpio_set(16);
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
