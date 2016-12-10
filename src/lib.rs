#![no_std]
#![feature(asm)]

#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ticks() -> u64 {
    let mask = 0xFFFFFFFFu64;
    let high: u64;
    let low: u64;
    unsafe {
        asm!("lfence;rdtsc"
            : "={edx}"(high), "={eax}"(low)
            :
            : "rdx", "rax"
            : "volatile"
        );
    }
    ((mask&high)<<32) | (mask&low)
}
#[test]
fn test_delta() {
    let x = ticks();
    let y = ticks();
    assert!( (y-x) < 1000000);
}



#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86")]
pub fn ticks() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        asm!("lfence;rdtsc"
            : "={edx}"(high), "={eax}"(low)
            :
            : "edx", "eax"
            : "volatile"
        );
    }
    let high_val = (high as u64) << 32;
    let low_val = low as u64;
    high_val|low_val
}
