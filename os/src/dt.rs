// DEVICE TREE mod

use fdt::Fdt;
use core::ptr;
use fdt::node::FdtNode;
use core::sync::atomic::{AtomicPtr, Ordering, AtomicUsize};
use crate::cpus::Cpus;

pub static CPU_NUMS: AtomicUsize = AtomicUsize::new(1);
pub static TIMER_FREQ: AtomicUsize = AtomicUsize::new(0);
pub static FDT: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());

fn print_node(node: FdtNode<'_, '_>, n_spaces: usize) {
    (0..n_spaces).for_each(|_| print!(" "));
    println!("{}/", node.name);
    for child in node.children() {
        print_node(child, n_spaces + 4);
    }
}

#[allow(unused)]
pub fn fdt_print(fdt: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt).unwrap() };
    print_node(fdt.find_node("/").unwrap(), 0);
}

fn fdt_get_timerfreq(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let hart_id = Cpus::cpu_id();
    let current_cpu = fdt.cpus().find(|cpu| cpu.ids().first() == hart_id).unwrap();
    let timebase_frequency = current_cpu.timebase_frequency();
    TIMER_FREQ.store(timebase_frequency, Ordering::Relaxed);
    // println!("timer freq: {}", TIMER_FREQ.load(Ordering::Relaxed));
}

fn fdt_get_ncpu(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let n_cpus = fdt.cpus().count();
    // println!("n_cpus: {}", n_cpus as u64);
    CPU_NUMS.store(n_cpus, Ordering::Release);
}

pub fn init(fdt_ptr: *mut u8) {
    FDT.store(fdt_ptr, Ordering::Release);
    fdt_get_timerfreq(fdt_ptr);
    fdt_get_ncpu(fdt_ptr);
}
