// DEVICE TREE mod

extern crate fdt;

use core::ptr;
use fdt::Fdt;
use fdt::node::FdtNode;
use core::sync::atomic::{AtomicPtr, Ordering, AtomicUsize};

pub static FDT: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());
pub static CPU_NUMS: AtomicUsize = AtomicUsize::new(1);
pub static TIMER_FREQ: AtomicUsize = AtomicUsize::new(0);

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

fn fdt_get_timerfreq(hart_id: usize, fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let current_cpu = fdt.cpus().find(|cpu| cpu.ids().first() == hart_id).unwrap();
    let timebase_frequency = current_cpu.timebase_frequency();
    println!("timer freq: {}", timebase_frequency);
    TIMER_FREQ.store(timebase_frequency, Ordering::Relaxed);
}

fn fdt_get_ncpu(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let n_cpus = fdt.cpus().count();
    println!("n_cpus: {}", n_cpus as u64);
    CPU_NUMS.store(n_cpus, Ordering::Release);
}

pub fn init(hart_id: usize, fdt_ptr: *mut u8) {
    FDT.store(fdt_ptr, Ordering::Release);
    fdt_get_timerfreq(hart_id, fdt_ptr);
    fdt_get_ncpu(fdt_ptr);
}
