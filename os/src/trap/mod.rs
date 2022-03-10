mod trap;

use trap::set_kernel_trap_entry;

pub fn init() {
    set_kernel_trap_entry();
    // println!("trap init finish");
}
