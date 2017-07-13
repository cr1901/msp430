//! Program counter

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u16 {
    let r;
    unsafe {
        asm!("mov R0,$0"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    r
}

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(bits: u16) {
    asm!("mov $0,R0"
         :
         : "r"(bits)
         :
         : "volatile");
}
