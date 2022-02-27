use core::arch::asm;
use core::hint::unreachable_unchecked;

#[no_mangle]
pub unsafe extern "C" fn timervec() -> ! {
    // start.rs has set up the memory that mscratch points to:
    // scratch[0,8,16] : register save area.
    // scratch[24] : address of CLINT's MTIMECMP register.
    // scratch[32] : desired interval between interrupts.

    // Now, mscrach has a pointer to an additional scratch space.
    // to aboid overwriting the contents of the integer registers,
    // the prologue of an interrupts handler usually begins by swapping
    // an integer register(say a0) with mscratch CSR.
    // The interrupt handler stores the integer registers
    // used for processing in this scratch space.
    // a0 saved in mscrach, a1 ~ a3 saved in scratch space.
    //loop {}
    asm!(".align 4"); // if miss this alignment, a load access fault will occur.
    asm!(
        "csrrw a0, mscratch, a0",
        "sd a1, 0(a0)",
        "sd a2, 8(a0)",
        "sd a3, 16(a0)",
    );

    // schedule the next timer interrupt
    // by adding interval to mtimecmp.
    asm!(
        "ld a1, 24(a0)", // CLINT_MTIMECMP(hartid) contents
        "ld a2, 32(a0)", // interval
        "ld a3, 0(a1)",
        "add a3, a3, a2",
        "sd a3, 0(a1)",
    );

    // raise a supervisor software interrupt.
    asm!("li a1, 2", "csrw sip, a1",);

    // restore and return
    asm!(
        "ld a3, 16(a0)",
        "ld a2, 8(a0)",
        "ld a1, 0(a0)",
        "csrrw a0, mscratch, a0",
        "mret",
    );

    unreachable_unchecked();
}
