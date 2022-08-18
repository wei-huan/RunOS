use crate::cpu::current_user_token;
use crate::mm::translated_str;

/// 控制 log 模式, 不完善
pub fn sys_syslog(type_: isize, buf: *const u8, len: isize) -> isize {
    log::debug!("sys_syslog");
    let token = current_user_token();
    // 这里传入的地址为用户的虚地址，因此要使用用户的虚地址进行映射
    if type_ == 2 | 3 {
        let path = translated_str(token, buf);
        println!("{}", path);
    }
    0
}
