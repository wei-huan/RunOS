use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::timer::set_next_trigger;
use log::*;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec, sepc
};

pub fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(kernel_trap_handler as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    // match scause.cause() {
    //     Trap::Interrupt(Interrupt::SupervisorTimer) => {
    //         set_next_trigger();
    //         info!("Yeah");
    //     }
    //     _ => {
            println!("stval = {:#?}, sepc = 0x{:X}", stval::read(), sepc::read());
            panic!("a trap {:?} from kernel!", scause.cause());
    //     }
    // }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn app_trap_handler() -> ! {
    user_trap_return();
}

#[no_mangle]
pub fn user_trap_return() -> ! {
    unreachable!();
}
