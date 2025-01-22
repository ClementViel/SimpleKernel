use core::arch;

const FREQ_MS: u32 = 13_500;

pub fn sleepms(ms: u32) {
    let time = ms * FREQ_MS;
    unsafe {
        for _ in 0..time {arch::asm!("nop");}
    }
}

pub fn wait_cycles(cycles: u32)
{
    let mut num = cycles;
    unsafe {
        arch::asm!("0: subs {0}, #1; bne 0b;\n", inout(reg) num);
    }
}

pub fn register_write(reg: usize, offset: usize, mask: usize, val: usize) 
{
    unsafe { 
    let register = reg as *mut usize;
        *(register) &= !mask;
        *(register) |= val << offset; 
    }
}

pub fn register_read(reg: usize, offset: usize, mask: usize) {
    unsafe { 
        let val = (reg) as *mut usize;
        *(val) &= reg >> offset;
    }
}
