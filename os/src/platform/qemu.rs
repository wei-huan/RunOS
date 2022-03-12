// use riscv::register::satp;

pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];

pub type BlockDeviceImpl = crate::drivers::block::VirtIOBlock;

pub enum ExitStatus {
    Pass,
    Reset,
    Fail(u16),
}

pub fn exit(exit_status: ExitStatus) -> ! {
    // let virt_test: *mut u32 = match satp::read().mode {
    //     satp::SatpMode::Bare => 0x10_0000 as *mut u32,
    //     _ => (PHYSICAL_OFFSET.load(Ordering::Acquire) + 0x10_0000) as *mut u32,
    // };

    // unsafe {
    //     core::ptr::write_volatile(virt_test, exit_status.to_u32());
    // }

    unreachable!()
}

