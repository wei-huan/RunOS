use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::cpu::{current_trap_cx, current_user_token, current_token};
use crate::mm::{kernel_token, kernel_translate};
use crate::scheduler::schedule;
use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sepc, stval, stvec, satp
};
// #[cfg(feature = "rustsbi")]
// use crate::rustsbi::shutdown;
// #[cfg(feature = "opensbi")]
// use crate::opensbi::shutdown;

global_asm!(include_str!("trap.S"));

pub fn set_kernel_trap_entry() {
    extern "C" {
        fn __kernelvec();
    }
    unsafe {
        stvec::write(__kernelvec as usize, TrapMode::Direct);
    }
}

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    // fn ekernel();
    // fn strampoline();
}

#[allow(unused)]
fn print_kernel_layout() {
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        ".bss [{:#x}, {:#x})",
        sbss_with_stack as usize, ebss as usize
    );
}

#[allow(unused)]
fn where_is_stval(stval: usize) {
    log::info!("stval = 0x{:X}", stval);
    if stval >= stext as usize && stval < etext as usize {
        println!("stval is in .text");
    } else if stval >= srodata as usize && stval < erodata as usize {
        println!("stval is in .rodata");
    } else if stval >= sdata as usize && stval < edata as usize {
        println!("stval is in .data");
    } else if stval >= sbss_with_stack as usize && stval < ebss as usize {
        println!("stval is in .bss");
    } else {
        println!("stval either");
    }
}

#[allow(unused)]
fn where_is_sepc(sepc: usize) {
    log::info!("sepc = 0x{:X}", sepc);
    if sepc >= stext as usize && sepc < etext as usize {
        println!("sepc is in .text");
    } else if sepc >= srodata as usize && sepc < erodata as usize {
        println!("sepc is in .rodata");
    } else if sepc >= sdata as usize && sepc < edata as usize {
        println!("sepc is in .data");
    } else if sepc >= sbss_with_stack as usize && sepc < ebss as usize {
        println!("sepc is in .bss");
    } else {
        println!("sepc either");
    }
}

#[no_mangle]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // log::debug!("Supervisor Timer");
            set_next_trigger();
            schedule();
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            log::debug!("boot hart");
        }
        Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadPageFault)
        | Trap::Exception(Exception::InstructionPageFault) => {
            let token = current_token();
            let kernel_token = kernel_token();
            if token != kernel_token {
                println!("not kernel token");
                unsafe {
                    satp::write(kernel_token);
                    asm!("sfence.vma");
                }
            } else {
                let stval = stval::read();
                if let Some(pte) = kernel_translate(stval.into()) {
                    println!("pte: {:#?}", pte);
                    panic!("a trap {:?} from kernel!", scause.cause());
                } else {
                    println!("No pte");
                    panic!("a trap {:?} from kernel!", scause.cause());
                }
            }
            panic!("a trap {:?} from kernel!", scause.cause());
        }
        _ => {
            log::error!("stval = {:#X} sepc = {:#X}", stval::read(), sepc::read());
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
    // log::debug!("kstack ptr: 0x{:X}", current_trap_cx().kernel_sp);
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // log::debug!("UserEnvCall");
            // jump to syscall next instruction anyway, avoid re-trigger
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
            log::debug!(
                "[kernel] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.",
                scause.cause(),
                stval,
                current_trap_cx().sepc,
            );
            // page fault exit code
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            log::debug!("[kernel] IllegalInstruction in application, kernel killed it.");
            // illegal instruction exit code
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // log::debug!("User Timer");
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
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    // log::debug!("trap return");
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
