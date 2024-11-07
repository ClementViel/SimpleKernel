#![no_std]
const GPIO_FSEL0: u32 = 0x20200000;
const GPIO_FSEL1: u32 = 0x20200004;
const GPIO_SET0: u32 = 0x2020001C;
const GPIO_SET1: u32 = 0x20200020;
const GPIO_CLR0: u32 = 0x20200028;
const GPIO_CLR1: u32 = 0x2020002C;

#[no_mangle]
pub fn gpio_select_mode(offset: u32, mode: u32) {
    let gpio_fun_reg = GPIO_FSEL1 as *mut u32;
    unsafe {*(gpio_fun_reg) |= mode << offset }
}

#[no_mangle]
pub fn gpio_set(gpio: u32) {
    let gpio_to_set = GPIO_SET0 as *mut u32;
    unsafe {*(gpio_to_set) = 1 << gpio }

}

#[no_mangle]
pub fn gpio_clr(gpio: u32) {
    let gpio_to_clr = GPIO_CLR0 as *mut u32;
    unsafe {*(gpio_to_clr) = 1 << gpio }
}
