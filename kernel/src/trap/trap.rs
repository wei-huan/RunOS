use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::cpu::{current_trap_cx, current_user_token};
use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sepc, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn set_kernel_trap_entry() {
    extern "C" {
        fn __kernelvec();
    }
    unsafe {
        stvec::write(__kernelvec as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            log::debug!("Supervisor Timer Interrupt");
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            log::debug!("Supervisor External Interrupt");
        }
        _ => {
            println!("stval = {}, sepc = 0x{:X}", stval::read(), sepc::read());
            panic!("a trap {:?} from kernel!", scause.cause());
        }
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn user_trap_handler() -> ! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // log::debug!("UserEnvCall");
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            // get system call return value
            let result = syscall(
                cx.x[17],
                [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14], cx.x[15]],
            );
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.x[10] = result as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!(
                "[kernel] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.",
                scause.cause(),
                stval,
                current_trap_cx().sepc,
            );
            // page fault exit code
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            // illegal instruction exit code
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
            // log::debug!("User Timer Interrupt");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    // check signals
    // if let Some((errno, msg)) = check_signals_of_current() {
    //     println!("[kernel] {}", msg);
    //     exit_current_and_run_next(errno);
    // }
    user_trap_return();
}

#[no_mangle]
pub fn user_trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __uservec();
        fn __restore();
    }
    let restore_va = __restore as usize - __uservec as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

// #[no_mangle]
// pub fn kernel_trap_goto_schedule() -> ! {
//     unsafe {
//         asm!(
//             "jr {restore_va}",
//             restore_va = in(reg) restore_va,
//             in("a0") trap_cx_ptr,
//             in("a1") user_satp,
//             options(noreturn)
//         );
//     }
// }
