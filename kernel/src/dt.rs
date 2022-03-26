// DEVICE TREE mod

use crate::cpu::cpu_id;
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use fdt::node::FdtNode;
use fdt::Fdt;

pub static CPU_NUMS: AtomicUsize = AtomicUsize::new(1);
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

#[cfg(feature = "qemu")]
fn fdt_get_timerfreq(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let hart_id = cpu_id();
    let current_cpu = fdt.cpus().find(|cpu| cpu.ids().first() == hart_id).unwrap();
    let timebase_frequency = current_cpu.timebase_frequency();
    TIMER_FREQ.store(403000000 / 62, Ordering::Relaxed);
    // println!("timer freq: {}", TIMER_FREQ.load(Ordering::Relaxed));
}

#[cfg(feature = "qemu")]
fn fdt_get_ncpu(fdt_ptr: *mut u8) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(fdt_ptr).unwrap() };
    let n_cpus = fdt.cpus().count();
    CPU_NUMS.store(2, Ordering::Release);
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

// qemu
#[cfg(feature = "qemu")]
pub fn init(fdt_ptr: *mut u8) {
    FDT.store(fdt_ptr, Ordering::Release);
    fdt_get_timerfreq(fdt_ptr);
    fdt_get_ncpu(fdt_ptr);
    // fdt_get_model(fdt_ptr);
}

// k210
#[cfg(not(any(feature = "qemu")))]
pub fn init() {
    // TIMER_FREQ.store(403000000 / 62, Ordering::Relaxed);
    // CPU_NUMS.store(2, Ordering::Release);
    println!("here fdt init");
}
