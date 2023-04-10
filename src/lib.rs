#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "LLVM_ASM", feature(llvm_asm))]
#![cfg_attr(feature = "OLD_ASM", feature(asm))]


#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ticks() -> u64 {
    let mask = 0xFFFFFFFFu64;
    let high: u64;
    let low: u64;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("lfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("lfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("lfence;rdtsc",
            out("edx") high,
            out("eax") low)
        }
    }
    ((high) << 32) | (mask & low)
}
#[test]
fn test_delta() {
    let x = ticks();
    let y = ticks();
    assert!((y - x) < 1000000);
}

/// The difference between `ticks` and `ticks_amd` is that
/// on AMD processors `lfence` instruction doesn't
/// always prevent instruction reordering. Instead it is
/// recommended to use the `mfence` instruction.
///
/// [See this
/// citation](https://github.com/golang/go/blob/bf9ad7080d0a22acf502a60d8bc6ebbc4f5340ef/src/runtime/asm_amd64.s#L112)
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ticks_amd() -> u64 {
    let mask = 0xFFFFFFFFu64;
    let high: u64;
    let low: u64;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("mfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("mfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("mfence;rdtsc",
            out("edx") high,
            out("eax") low)
        }
    }
    ((high) << 32) | (mask & low)
}

/// The standard `ticks` and `ticks_amd` use the `rdtsc` instruction which as it
/// maybe reordered in respect to other instructions the functions will contain
/// memory fences to attempt to insure relative order. But in some highly
/// sensative benchmarks this may introduce memory overhead, as you enforce
/// cache consistency.
///
/// Please see these docs on the [`rdtscp`](http://www.felixcloutier.com/x86/RDTSCP.html)
/// instruction.
///
/// Be aware that this instruction maybe masked by the OS, or Hypervisor.
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ticks_modern() -> u64 {
    let mask = 0xFFFFFFFFu64;
    let high: u64;
    let low: u64;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("rdtscp"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("rdtscp"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("rdtscp",
            out("edx") high,
            out("eax") low)
        }
    }
    ((high) << 32) | (mask & low)
}

#[inline(never)]
#[cfg(target_arch = "x86")]
pub fn ticks() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("lfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "edx", "eax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("rdtscp"
                : "={edx}"(high), "={eax}"(low)
                :
                : "rdx", "rax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("rdtscp",
            out("edx") high,
            out("eax") low)
        }
    }
    let high_val = (high as u64) << 32;
    let low_val = low as u64;
    high_val | low_val
}

/// The difference between `ticks` and `ticks_amd` is that
/// on AMD processors `lfence` instruction doesn't
/// always prevent instruction reordering. Instead it is
/// recommended to use the `mfence` instruction.
///
/// [See this
/// citation](https://github.com/golang/go/blob/bf9ad7080d0a22acf502a60d8bc6ebbc4f5340ef/src/runtime/asm_amd64.s#L112)
#[inline(never)]
#[cfg(target_arch = "x86")]
pub fn ticks_amd() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("mfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "edx", "eax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("mfence;rdtsc"
                : "={edx}"(high), "={eax}"(low)
                :
                : "edx", "eax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("mfence;rdtsc",
            out("edx") high,
            out("eax") low)
        }
    }
    let high_val = (high as u64) << 32;
    let low_val = low as u64;
    high_val | low_val
}

/// The standard `ticks` and `ticks_amd` use the `rdtsc` instruction which as it
/// maybe reordered in respect to other instructions the functions will contain
/// memory fences to attempt to insure relative order. But in some highly
/// sensitive benchmarks this may introduce memory overhead, as you enforce
/// cache consistency.
///
/// Please see these docs on the [`rdtscp`](http://www.felixcloutier.com/x86/RDTSCP.html)
/// instruction.
///
/// Be aware that this instruction maybe masked by the OS, or Hypervisor.
#[inline(never)]
#[cfg(target_arch = "x86")]
pub fn ticks_modern() -> u64 {
    let mask = 0xFFFFFFFFu64;
    let high: u32;
    let low: u32;
    unsafe {
        #[cfg(feature = "LLVM_ASM")]
        {
            llvm_asm!("rdtscp"
                : "={edx}"(high), "={eax}"(low)
                :
                : "edx", "eax"
                : "volatile"
            );
        }
        #[cfg(feature = "OLD_ASM")]
        {
            asm!("rdtscp"
                : "={edx}"(high), "={eax}"(low)
                :
                : "edx", "eax"
                : "volatile"
            );
        }
        #[cfg(all(not(feature = "LLVM_ASM"), not(feature = "OLD_ASM")))]
        {
            core::arch::asm!("rdtscp",
            out("edx") high,
            out("eax") low)
        }
    }
    let high_val = (high as u64) << 32;
    let low_val = low as u64;
    high_val | low_val
}
