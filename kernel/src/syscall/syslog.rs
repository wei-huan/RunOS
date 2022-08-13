use crate::cpu::{current_task, current_user_token};
use crate::mm::translated_str;

/// 控制 log 模式, 不完善
pub fn sys_syslog(_type: isize, buf_pointer: *const u8, len: isize) -> isize {
    log::debug!("sys_syslog");
    let task = current_task().unwrap();
    let token = current_user_token();
    // 这里传入的地址为用户的虚地址，因此要使用用户的虚地址进行映射
    if _type == 2 | 3 {
        let path = translated_str(token, buf_pointer);
        println!("{}", path);
    }
    0
}
