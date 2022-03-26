// DEVICE TREE mod

use crate::cpu::cpu_id;
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use fdt::node::FdtNode;
use fdt::Fdt;

pub static CPU_NUMS: AtomicUsize = AtomicUsize::new(2);
pub static TIMER_FREQ: AtomicUsize = AtomicUsize::new(403000000 / 62);
pub static FDT: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());
// pub static MODEL: AtomicPtr<&str> = AtomicPtr::new(ptr::null_mut());

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
    let hart_id = cpu_id();
    let current_cpu = fdt.cpus().find(|cpu| cpu.ids().first() == hart_id).unwrap();
    let timebase_frequency = current_cpu.timebase_frequency();
    TIMER_FREQ.store(timebase_frequency, Ordering::Relaxed);
    // println!("timer freq: {}", TIMER_FREQ.load(Ordering::Relaxed));
}

fn fdt_get_ncpu(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let n_cpus = fdt.cpus().count();
    CPU_NUMS.store(n_cpus, Ordering::Release);
    // println!("n_cpus: {}", n_cpus as u64);
}

#[allow(unused)]
pub fn fdt_get_model(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let model = fdt
        .root()
        .property("model")
        .and_then(|p| p.as_str())
        .unwrap();
    // println!("device_model: {}", model);
    // MODEL.store(model as *const _ as *mut &'static str, Ordering::Release);
}

// qemu rustsbi
// #[cfg(all(feature = "qemu", feature = "rustsbi"))]
pub fn init(dts_ptr: *const u8) {
    TIMER_FREQ.store(100000000, Ordering::Relaxed);
    CPU_NUMS.store(2, Ordering::Relaxed);
    FDT.store(dts_ptr as *mut u8, Ordering::Release);
}

// qemu opensbi
#[cfg(all(feature = "qemu", feature = "opensbi"))]
pub fn init(dts_ptr: *const u8) {
    FDT.store(dts_ptr as *mut u8, Ordering::Release);
    fdt_get_timerfreq(dts_ptr);
    fdt_get_ncpu(dts_ptr);
    // fdt_get_model(fdt_ptr);
}
