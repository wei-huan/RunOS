use crate::cpu::current_user_token;
use crate::mm::{translated_byte_buffer, UserBuffer};
use core::mem::size_of;

struct SysInfo {
    uptime: i64,     /* Seconds since boot */
    loads: [u64; 3], /* 1, 5, and 15 minute load averages */
    totalram: u64,   /* Total usable main memory size */
    freeram: u64,    /* Available memory size */
    sharedram: u64,  /* Amount of shared memory */
    bufferram: u64,  /* Memory used by buffers */
    totalswap: u64,  /* Total swap space size */
    freeswap: u64,   /* Swap space still available */
    procs: u8,       /* Number of current processes */
    totalhigh: u64,  /* Total high memory size */
    freehigh: u64,   /* Available high memory size */
    mem_unit: u32,   /* Memory unit size in bytes */
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            uptime: 100,
            loads: [30; 3],
            totalram: 0x800000,
            freeram: 0x200000,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: 3,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 1, /* Memory unit size in bytes */
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }
}

pub fn sys_sysinfo(buf: *mut u8) -> isize {
    log::debug!("sys_sysinfo");
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<SysInfo>());
    let sysinfo = SysInfo::new();
    let mut userbuf = UserBuffer::new(buf_vec);
    userbuf.write(sysinfo.as_bytes());
    return 0;
}
