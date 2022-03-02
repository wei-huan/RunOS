// DEVICE TREE mod

extern crate fdt;

use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use fdt::node::FdtNode;
use fdt::Fdt;

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
// 打印设备树
pub fn fdt_print() {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(FDT.load(Ordering::Acquire)).unwrap() };
    print_node(fdt.find_node("/").unwrap(), 0);
}

// 从设备树获取 cpu 时钟频率
fn fdt_get_timerfreq(hart_id: usize) {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(FDT.load(Ordering::Acquire)).unwrap() };
    let current_cpu = fdt.cpus().find(|cpu| cpu.ids().first() == hart_id).unwrap();
    let timebase_frequency = current_cpu.timebase_frequency();
    // println!("timer freq: {}", timebase_frequency);
    TIMER_FREQ.store(timebase_frequency, Ordering::Relaxed);
}

// 从设备树获取 cpu 核数量
fn fdt_get_ncpu() {
    let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(FDT.load(Ordering::Acquire)).unwrap() };
    let n_cpus = fdt.cpus().count();
    // println!("n_cpus: {}", n_cpus as u64);
    CPU_NUMS.store(n_cpus, Ordering::Release);
}

// 从设备树获取硬件内存
fn fdt_get_mm() {
    // let fdt: Fdt<'static> = unsafe { Fdt::from_ptr(FDT.load(Ordering::Acquire)).unwrap() };
    // let (mem_size, mem_start) = {
    //     let memory = fdt
    //         .memory()
    //         .regions()
    //         .find(|region| {
    //             let start = region.starting_address as usize;
    //             let end = region.starting_address as usize + region.size.unwrap();
    //             let kstart_phys = unsafe {
    //                 let start = kernel_patching::kernel_start();
    //                 kernel_section_v2p(VirtualAddress::from_ptr(start)).as_usize()
    //             };
    //             start <= kstart_phys && kstart_phys <= end
    //         })
    //         .unwrap();

    //     (memory.size.unwrap() / 1024 / 1024, memory.starting_address)
    // };
}

// 根据文件设备树初始化OS
pub fn init(hart_id: usize, fdt_ptr: *mut u8) {
    FDT.store(fdt_ptr, Ordering::Release);
    fdt_get_ncpu();
    fdt_get_mm();
    fdt_get_timerfreq(hart_id);
}
