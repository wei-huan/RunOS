use crate::cpu::current_user_token;
use crate::mm::{translated_byte_buffer, UserBuffer};
use core::mem::size_of;

struct UTSName {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

impl UTSName {
    pub fn new() -> Self {
        Self {
            sysname: UTSName::str2u8("Linux"),
            nodename: UTSName::str2u8("ubuntu"),
            release: UTSName::str2u8("5.10.0-7-riscv64"),
            version: UTSName::str2u8("#1 SMP Golden Wheel 0.1.0 (2022-04-24)"),
            machine: UTSName::str2u8("RISC-V64"),
            domainname: UTSName::str2u8("Haozhe Tang"),
        }
    }
    fn str2u8(str: &str) -> [u8; 65] {
        let mut arr: [u8; 65] = [0; 65];
        let str_bytes = str.as_bytes();
        let len = str.len();
        for i in 0..len {
            arr[i] = str_bytes[i];
        }
        arr
    }
    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }
}

pub fn sys_uname(buf: *mut u8) -> isize {
    // log::debug!("sys_uname");
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<UTSName>());
    let uname = UTSName::new();
    let mut userbuf = UserBuffer::new(buf_vec);
    userbuf.write(uname.as_bytes());
    0
}
