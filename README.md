x86/x86_64 Timer
---

This is a low level CPU cycle count timer. What it returns is the value held within the processor time stamp counter. This counter is incremented every CPU cycle.

#### Considerations

When bench marking with this crate you want to assure your CPU is set to a 
fixed multipler. This mean Intel Turbo Boost must be OFF. You can do this
in via the bios.

This crate requires Rust Nightly (as it uses `llvm_asm!` macro),
you will need to enable `!#[feature(llvm_asm)]`.

#### Functions

```rust
fn ticks() -> u64;
```

This returns the number of CPU cycles that have passed since the last time
the CPU counter has rolled over. OR Power on.

#### In Depth Documentation


This function step by step:

```nasm
#Intel Syntax

ldfence		#cheapest fence on x86
		    #this prevents instruction re-ordering
		    #ensures all loads are complete
		    #on x64 this is done for you by the memory model
		    #so this fence is _free_
		    #this fence is here to prevent speculative
		    #execution of rdtsc.

rdtsc		#puts timestamp counter values into the low
		    #32bits of rdx and rax.


shl rdx, $32	#move the high section, into the high 32bits of
		        #its register

or rax, rdx	#combine bits

retq		#leave function
```

[Reference](http://www.felixcloutier.com/x86/RDTSC.html)


### License

Consider this crate licensed under the MIT.


### x86 support

This crate does implement 32bit x86 support, but it is untested.
