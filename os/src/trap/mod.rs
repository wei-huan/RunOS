mod trap;

use trap::entry_init;

pub fn init() {
    entry_init();
    // println!("trap init finish");
}
