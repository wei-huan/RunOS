use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::cpu::{current_task, current_trap_cx, current_user_token, hart_id};
use crate::syscall::syscall;
use crate::task::{
    check_signals_error_of_current, current_add_signal, exit_current_and_run_next, handle_signals,
    suspend_current_and_run_next, SignalFlags, TIME_TO_SCHEDULE,
};
use crate::timer::set_next_trigger;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    // satp,
    scause::{self, Exception, Interrupt, Trap},
    sepc,
    stval,
    stvec,
};

global_asm!(include_str!("trap.S"));

#[inline(always)]
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
    // unsafe {
    //     backtrace();
    // }
    let scause = scause::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            log::trace!("Supervisor Timer");
            set_next_trigger();
            unsafe {
                TIME_TO_SCHEDULE[hart_id()] = true;
            }
            // go_to_schedule();
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            log::trace!("boot hart");
        }
        Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadPageFault)
        | Trap::Exception(Exception::InstructionPageFault) => {
            log::error!("stval = {:#X} sepc = {:#X}", stval::read(), sepc::read());
            // let token = current_token();
            // let kernel_token = kernel_token();
            // if token != kernel_token {
            //     println!("not kernel token");
            //     unsafe {
            //         satp::write(kernel_token);
            //         asm!("sfence.vma");
            //     }
            // } else {
            //     let stval = stval::read();
            //     if let Some(pte) = kernel_translate(stval.into()) {
            //         println!("ppn: {:#?}", pte.ppn());
            //     } else {
            //         println!("No pte");
            //         // sfence(Some(stval.into()), None);
            //     }
            // }
            panic!("a trap {:?} from kernel", scause.cause());
        }
        Trap::Exception(Exception::Breakpoint) => {
            log::debug!("Breakpoint");
        }
        _ => {
            panic!(
                "a trap {:?} from kernel with stval = {:#X} sepc = {:#X}!",
                scause.cause(),
                stval::read(),
                sepc::read()
            );
        }
    }
}

#[inline(always)]
fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn user_trap_handler() -> ! {
    set_kernel_trap_entry();
    // unsafe {
    //     backtrace();
    // }
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            log::trace!("UserEnvCall");
            let mut cx = current_trap_cx();
            // jump to syscall next instruction anyway, avoid re-trigger
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
            let heap_base = current_task().unwrap().acquire_inner_lock().heap_start;
            let heap_top = current_task().unwrap().acquire_inner_lock().heap_pointer;
            println!("heap_base = {:#x?}, heap_top = {:#x?}", heap_base, heap_top);
            // page fault exit code
            exit_current_and_run_next(-2);
            current_add_signal(SignalFlags::SIGSEGV);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            log::debug!("[kernel] IllegalInstruction in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.",
            stval,
            current_trap_cx().sepc);
            // illegal instruction exit code
            exit_current_and_run_next(-3);
            current_add_signal(SignalFlags::SIGILL);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // log::debug!("User Timer");
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    // handle signals (handle the sent signal)
    handle_signals();

    // check error signals (if error then exit)
    if let Some((errno, msg)) = check_signals_error_of_current() {
        log::error!("[kernel] {}", msg);
        exit_current_and_run_next(errno);
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __uservec();
        fn __restore();
    }
    let restore_va = __restore as usize - __uservec as usize + TRAMPOLINE;
    // log::debug!("trap return");
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
