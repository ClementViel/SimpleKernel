#![no_std]

const GPIO_FSEL0: usize = 0x20200000;
const GPIO_FSEL1: usize = 0x20200004;
const GPIO_SET0: usize = 0x2020001C;
const GPIO_SET1: usize = 0x20200020;
const GPIO_CLR0: usize = 0x20200028;
const GPIO_CLR1: usize = 0x2020002C;
const GPIO_PUD: usize = 0x20200094;
const GPIO_PUD_CLK0: usize = 0x20200094;
const GPIO_PUD_CLK1: usize = 0x20200094;
const GPIO_PUD_EN_PU: u8 = 0x01;
const GPIO_PUD_EN_PD: u8 = 0x10;
const GPIO_PUD_DIS: u8 = 0x00;

use crate::utils;


fn gpio_write(address: *mut u32,mask: u32, val: u32, offset: u32) {
    unsafe {
        (*address) &= mask;
        (*address) |= val << offset;
    }
}

#[no_mangle]
pub fn gpio_select_mode(offset: usize, mode: usize) {
    let gpio_fun_reg = if offset < 10  {
        GPIO_FSEL0 as usize
    } else {
        GPIO_FSEL1 as usize
    };
   let mask: usize = 0xFFFF8FFF;
   utils::register_write(gpio_fun_reg, offset, mask, mode); 
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

#[no_mangle]
pub fn gpio_pud_write_state(pad: u8, state: u8)
{
let clk_register =  if pad == 0 { GPIO_PUD_CLK0 as *mut u32 } else { GPIO_PUD_CLK1 as *mut u32};
let pud_register = GPIO_PUD as *mut u32;

unsafe {
    *(pud_register) |= state as u32;
    utils::wait_cycles(150);
    *(clk_register) = (1 << 15) | (1 << 14); 
    utils::wait_cycles(150);
    *(pud_register) = 0x00;
    *(clk_register) = 0x00000000; 
}

    
}
