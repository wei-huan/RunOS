use crate::config::BOOT_STACK_SIZE;

pub fn time_parts(micros: usize) -> (usize, usize, usize) {
    let seconds = micros / (1000 * 1000);
    let micros_left = micros % (1000 * 1000);
    let millis = micros_left / 1000;
    let micros = micros_left % 1000;
    (seconds, millis, micros)
}

pub fn micros(ticks: usize, hz: usize) -> usize {
    // ticks / hz -> second
    // ticks / (hz / 1000) -> millisecond
    // ticks / (hz / 1000 / 1000) -> microsecond
    ticks / (hz / 1000 / 1000)
}

pub fn get_boot_stack(hart_id: usize) -> (usize, usize) {
    extern "C" {
        fn boot_stack();
    }
    let low = boot_stack as usize + BOOT_STACK_SIZE * hart_id;
    let high = low + BOOT_STACK_SIZE;
    (low, high)
}