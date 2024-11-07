use core::arch;

const FREQ_MS: u32 = 13_500;

pub fn sleepms(ms: u32) {
    let time = ms * FREQ_MS;
    unsafe {
        for _ in 0..time {arch::asm!("nop");}
    }
}
