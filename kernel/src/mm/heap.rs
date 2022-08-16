extern crate alloc;

use crate::config::{HEAP_ALLOCATOR_MAX_ORDER, KERNEL_HEAP_SIZE};
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<HEAP_ALLOCATOR_MAX_ORDER> = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Kernel Heap allocation error, layout = {:#?}", layout)
}

static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

extern "C" {
    fn sbss();
    fn ebss();
    fn sdata();
    fn edata();
}

#[allow(unused)]
pub fn whereis_heap() {
    println!("data: 0x{:X} - 0x{:X}", sdata as usize, edata as usize);
    println!("bss: 0x{:X} - 0x{:X}", sbss as usize, ebss as usize);
    unsafe {
        println!(
            "HEAP_SPACE: 0x{:X}",
            (&(KERNEL_HEAP_SPACE[KERNEL_HEAP_SIZE / 2])) as *const u8 as usize
        );
    }
}

#[allow(unused)]
pub fn heap_test() {
    use alloc::vec::Vec;
    let bss_range = sbss as usize..ebss as usize;

    let mut a = Vec::<usize>::new();
    for i in 1..=10 {
        a.push(i);
    }

    assert!(bss_range.contains(&(a.as_ptr() as usize)));
    println!("heap test pass");
}
