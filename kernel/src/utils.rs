use crate::config::BOOT_STACK_SIZE;

pub fn get_boot_stack(hart_id: usize) -> (usize, usize) {
    extern "C" {
        fn boot_stack();
    }
    let low = boot_stack as usize + BOOT_STACK_SIZE * hart_id;
    let high = low + BOOT_STACK_SIZE;
    (low, high)
}