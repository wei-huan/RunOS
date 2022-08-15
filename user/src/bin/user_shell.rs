// #![no_std]
// #![no_main]

// extern crate alloc;

// #[macro_use]
// extern crate user;

// use alloc::{
//     string::{String, ToString},
//     vec::Vec,
// };
// use user::{exec, fork, waitpid};

// fn final_round_one_test() {
//     let app_name = "runtest.exe\0";

//     let mut static_tests = Vec::new();

//     // /****************************  static  *****************************/
//     static_tests.push("argv\0");
//     static_tests.push("basename\0");
//     static_tests.push("clocale_mbfuncs\0");
//     static_tests.push("clock_gettime\0");
//     static_tests.push("crypt\0");
//     static_tests.push("dirname\0");
//     static_tests.push("env\0");
//     static_tests.push("fdopen\0");
//     static_tests.push("fnmatch\0");
//     static_tests.push("fscanf\0");
//     static_tests.push("fwscanf\0");
//     static_tests.push("iconv_open\0");
//     static_tests.push("inet_pton\0");
//     static_tests.push("mbc\0");
//     static_tests.push("memstream\0");
//     // static_tests.push("pthread_cancel_points\0");
//     // static_tests.push("pthread_cancel\0");
//     // static_tests.push("pthread_cond\0");
//     // static_tests.push("pthread_tsd\0");
//     static_tests.push("qsort\0");
//     static_tests.push("random\0");
//     static_tests.push("search_hsearch\0");
//     static_tests.push("search_insque\0");
//     static_tests.push("search_lsearch\0");
//     static_tests.push("search_tsearch\0");
//     static_tests.push("setjmp\0");
//     static_tests.push("snprintf\0");
//     static_tests.push("sscanf\0");
//     static_tests.push("sscanf_long\0");
//     static_tests.push("stat\0");
//     static_tests.push("strftime\0");
//     static_tests.push("string\0");
//     static_tests.push("string_memcpy\0");
//     static_tests.push("string_memmem\0");
//     static_tests.push("string_memset\0");
//     static_tests.push("string_strchr\0");
//     static_tests.push("string_strcspn\0");
//     static_tests.push("string_strstr\0");
//     static_tests.push("strptime\0");
//     static_tests.push("strtod\0");
//     static_tests.push("strtod_simple\0");
//     static_tests.push("strtof\0");
//     static_tests.push("strtol\0");
//     static_tests.push("strtold\0");
//     static_tests.push("swprintf\0");
//     static_tests.push("tgmath\0");
//     static_tests.push("time\0");
//     static_tests.push("tls_align\0");
//     static_tests.push("udiv\0");
//     static_tests.push("ungetc\0");
//     // static_tests.push("utime\0");
//     static_tests.push("wcsstr\0");
//     static_tests.push("wcstol\0");
//     static_tests.push("pleval\0");
//     // static_tests.push("daemon_failure\0");
//     static_tests.push("dn_expand_empty\0");
//     static_tests.push("dn_expand_ptr_0\0");
//     static_tests.push("fflush_exit\0");
//     static_tests.push("fgets_eof\0");
//     static_tests.push("fgetwc_buffering\0");
//     static_tests.push("fpclassify_invalid_ld80\0");
//     static_tests.push("ftello_unflushed_append\0");
//     static_tests.push("getpwnam_r_crash\0");
//     static_tests.push("getpwnam_r_errno\0");
//     static_tests.push("iconv_roundtrips\0");
//     static_tests.push("inet_ntop_v4mapped\0");
//     static_tests.push("inet_pton_empty_last_field\0");
//     static_tests.push("iswspace_null\0");
//     static_tests.push("lrand48_signextend\0");
//     static_tests.push("lseek_large\0");
//     static_tests.push("malloc_0\0");
//     static_tests.push("mbsrtowcs_overflow\0");
//     static_tests.push("memmem_oob_read\0");
//     static_tests.push("memmem_oob\0");
//     static_tests.push("mkdtemp_failure\0");
//     static_tests.push("mkstemp_failure\0");
//     static_tests.push("printf_1e9_oob\0");
//     static_tests.push("printf_fmt_g_round\0");
//     static_tests.push("printf_fmt_g_zeros\0");
//     static_tests.push("printf_fmt_n\0");
//     static_tests.push("putenv_doublefree\0");
//     static_tests.push("regex_backref_0\0");
//     static_tests.push("regex_bracket_icase\0");
//     static_tests.push("regex_ere_backref\0");
//     static_tests.push("regex_escaped_high_byte\0");
//     static_tests.push("regex_negated_range\0");
//     static_tests.push("regexec_nosub\0");
//     static_tests.push("rewind_clear_error\0");
//     // static_tests.push("rlimit_open_files\0");
//     static_tests.push("scanf_bytes_consumed\0");
//     static_tests.push("scanf_match_literal_eof\0");
//     static_tests.push("scanf_nullbyte_char\0");
//     static_tests.push("setvbuf_unget\0");
//     static_tests.push("sigprocmask_internal\0");
//     static_tests.push("sscanf_eof\0");
//     static_tests.push("statvfs\0");
//     static_tests.push("strverscmp\0");
//     // static_tests.push("syscall_sign_extend\0");
//     static_tests.push("uselocale_0\0");
//     static_tests.push("wcsncpy_read_overflow\0");
//     static_tests.push("wcsstr_false_negative\0");

//     let mut static_args: Vec<*const u8> = Vec::new();
//     static_args.push(app_name.as_ptr());
//     static_args.push("-w\0".as_ptr());
//     static_args.push("entry-static.exe\0".as_ptr());

//     for static_test in static_tests {
//         static_args.push(static_test.as_ptr());
//         static_args.push(core::ptr::null::<u8>());
//         let pid = fork();
//         if pid == 0 {
//             exec(app_name, static_args.as_slice());
//         } else {
//             let mut exit_code = 0;
//             waitpid(pid as usize, &mut exit_code);
//         }
//         static_args.pop();
//         static_args.pop();
//     }

//     /****************************  dynamic  *****************************/
//     let mut dynamic_tests = Vec::new();
//     dynamic_tests.push("argv\0");
//     dynamic_tests.push("basename\0");
//     dynamic_tests.push("clocale_mbfuncs\0");
//     dynamic_tests.push("clock_gettime\0");
//     dynamic_tests.push("crypt\0");
//     dynamic_tests.push("dirname\0");
//     dynamic_tests.push("dlopen\0");
//     dynamic_tests.push("env\0");
//     dynamic_tests.push("fdopen\0");
//     dynamic_tests.push("fnmatch\0");
//     dynamic_tests.push("fscanf\0");
//     dynamic_tests.push("fwscanf\0");
//     dynamic_tests.push("iconv_open\0");
//     dynamic_tests.push("inet_pton\0");
//     dynamic_tests.push("mbc\0");
//     dynamic_tests.push("memstream\0");
//     // dynamic_tests.push("pthread_cancel_points\0");
//     // dynamic_tests.push("pthread_cancel\0");
//     // dynamic_tests.push("pthread_cond\0");
//     // dynamic_tests.push("pthread_tsd\0");
//     dynamic_tests.push("qsort\0");
//     dynamic_tests.push("random\0");
//     dynamic_tests.push("search_hsearch\0");
//     dynamic_tests.push("search_insque\0");
//     dynamic_tests.push("search_lsearch\0");
//     dynamic_tests.push("search_tsearch\0");
//     // dynamic_tests.push("sem_init\0");
//     dynamic_tests.push("setjmp\0");
//     dynamic_tests.push("snprintf\0");
//     // dynamic_tests.push("socket\0");
//     dynamic_tests.push("sscanf\0");
//     dynamic_tests.push("sscanf_long\0");
//     // dynamic_tests.push("stat\0");        // src/functional/stat.c:22: st.st_ctime<=t failed: 386072 > 32
//     dynamic_tests.push("strftime\0");
//     dynamic_tests.push("string\0");
//     dynamic_tests.push("string_memcpy\0");
//     dynamic_tests.push("string_memmem\0");
//     dynamic_tests.push("string_memset\0");
//     dynamic_tests.push("string_strchr\0");
//     dynamic_tests.push("string_strcspn\0");
//     dynamic_tests.push("string_strstr\0");
//     dynamic_tests.push("strptime\0");
//     dynamic_tests.push("strtod\0");
//     dynamic_tests.push("strtod_simple\0");
//     dynamic_tests.push("strtof\0");
//     dynamic_tests.push("strtol\0");
//     dynamic_tests.push("strtold\0");
//     dynamic_tests.push("swprintf\0");
//     dynamic_tests.push("tgmath\0");
//     dynamic_tests.push("time\0");
//     // dynamic_tests.push("tls_init\0");
//     // dynamic_tests.push("tls_local_exec\0");
//     dynamic_tests.push("udiv\0");
//     dynamic_tests.push("ungetc\0");
//     // dynamic_tests.push("utime\0");
//     dynamic_tests.push("wcsstr\0");
//     dynamic_tests.push("wcstol\0");
//     // dynamic_tests.push("daemon_failure\0");
//     dynamic_tests.push("dn_expand_empty\0");
//     dynamic_tests.push("dn_expand_ptr_0\0");
//     dynamic_tests.push("fflush_exit\0");
//     dynamic_tests.push("fgets_eof\0");
//     dynamic_tests.push("fgetwc_buffering\0");
//     dynamic_tests.push("fpclassify_invalid_ld80\0");
//     dynamic_tests.push("ftello_unflushed_append\0");
//     dynamic_tests.push("getpwnam_r_crash\0");
//     dynamic_tests.push("getpwnam_r_errno\0");
//     dynamic_tests.push("iconv_roundtrips\0");
//     dynamic_tests.push("inet_ntop_v4mapped\0");
//     dynamic_tests.push("inet_pton_empty_last_field\0");
//     dynamic_tests.push("iswspace_null\0");
//     dynamic_tests.push("lrand48_signextend\0");
//     dynamic_tests.push("lseek_large\0");
//     dynamic_tests.push("malloc_0\0");
//     dynamic_tests.push("mbsrtowcs_overflow\0");
//     dynamic_tests.push("memmem_oob_read\0");
//     dynamic_tests.push("memmem_oob\0");
//     dynamic_tests.push("mkdtemp_failure\0");
//     dynamic_tests.push("mkstemp_failure\0");
//     dynamic_tests.push("printf_1e9_oob\0");
//     dynamic_tests.push("printf_fmt_g_round\0");
//     dynamic_tests.push("printf_fmt_g_zeros\0");
//     dynamic_tests.push("printf_fmt_n\0");
//     // dynamic_tests.push("pthread_robust_detach\0");
//     // dynamic_tests.push("pthread_cond_smasher\0");
//     // dynamic_tests.push("pthread_condattr_setclock\0");
//     // dynamic_tests.push("pthread_exit_cancel\0");
//     // dynamic_tests.push("pthread_once_deadlock\0");
//     // dynamic_tests.push("pthread_rwlock_ebusy\0");
//     dynamic_tests.push("putenv_doublefree\0");
//     dynamic_tests.push("regex_backref_0\0");
//     dynamic_tests.push("regex_bracket_icase\0");
//     dynamic_tests.push("regex_ere_backref\0");
//     dynamic_tests.push("regex_escaped_high_byte\0");
//     dynamic_tests.push("regex_negated_range\0");
//     dynamic_tests.push("regexec_nosub\0");
//     dynamic_tests.push("rewind_clear_error\0");
//     // dynamic_tests.push("rlimit_open_files\0");
//     dynamic_tests.push("scanf_bytes_consumed\0");
//     dynamic_tests.push("scanf_match_literal_eof\0");
//     dynamic_tests.push("scanf_nullbyte_char\0");
//     dynamic_tests.push("setvbuf_unget\0");
//     dynamic_tests.push("sigprocmask_internal\0");
//     dynamic_tests.push("sscanf_eof\0");
//     dynamic_tests.push("statvfs\0");
//     dynamic_tests.push("strverscmp\0");
//     dynamic_tests.push("statvfs\0");
//     // dynamic_tests.push("syscall_sign_extend\0");
//     // dynamic_tests.push("tls_get_new_dtv\0");
//     dynamic_tests.push("uselocale_0\0");
//     dynamic_tests.push("wcsncpy_read_overflow\0");
//     dynamic_tests.push("wcsstr_false_negative\0");

//     let mut dynamic_args: Vec<*const u8> = Vec::new();
//     dynamic_args.push(app_name.as_ptr());
//     dynamic_args.push("-w\0".as_ptr());
//     dynamic_args.push("entry-dynamic.exe\0".as_ptr());

//     for dynamic_test in dynamic_tests {
//         dynamic_args.push(dynamic_test.as_ptr());
//         dynamic_args.push(core::ptr::null::<u8>());
//         let pid = fork();
//         if pid == 0 {
//             exec(app_name, dynamic_args.as_slice());
//         } else {
//             let mut exit_code = 0;
//             waitpid(pid as usize, &mut exit_code);
//         }
//         dynamic_args.pop();
//         dynamic_args.pop();
//     }
// }

// fn busybox_test() {
//     // no busybox bash
//     let mut busybox_tests_with_args = Vec::new();
//     busybox_tests_with_args.push("echo \"#### independent command test\"");
//     busybox_tests_with_args.push("ash -c exit");
//     busybox_tests_with_args.push("sh -c exit");
//     busybox_tests_with_args.push("basename /aaa/bbb");
//     busybox_tests_with_args.push("cal");
//     busybox_tests_with_args.push("clear");
//     busybox_tests_with_args.push("date");
//     busybox_tests_with_args.push("df");
//     busybox_tests_with_args.push("dirname /aaa/bbb");
//     busybox_tests_with_args.push("dmesg");
//     busybox_tests_with_args.push("du");
//     busybox_tests_with_args.push("expr 1 + 1");
//     busybox_tests_with_args.push("false");
//     busybox_tests_with_args.push("true");
//     busybox_tests_with_args.push("which ls");
//     busybox_tests_with_args.push("uname");
//     busybox_tests_with_args.push("uptime");
//     busybox_tests_with_args.push("printf \"abc\n\"");
//     busybox_tests_with_args.push("ps");
//     busybox_tests_with_args.push("pwd");
//     busybox_tests_with_args.push("free");
//     busybox_tests_with_args.push("hwclock");
//     busybox_tests_with_args.push("kill 10");
//     busybox_tests_with_args.push("ls");
//     busybox_tests_with_args.push("sleep 1");
//     busybox_tests_with_args.push("echo \"#### file opration test\"");
//     busybox_tests_with_args.push("touch test.txt");
//     busybox_tests_with_args.push("echo \"hello world\" > test.txt");
//     busybox_tests_with_args.push("cat test.txt");
//     busybox_tests_with_args.push("cut -c 3 test.txt");
//     busybox_tests_with_args.push("od test.txt");
//     busybox_tests_with_args.push("head test.txt");
//     busybox_tests_with_args.push("tail test.txt");
//     busybox_tests_with_args.push("hexdump -C test.txt");
//     busybox_tests_with_args.push("md5sum test.txt");
//     busybox_tests_with_args.push("echo \"ccccccc\" >> test.txt");
//     busybox_tests_with_args.push("echo \"bbbbbbb\" >> test.txt");
//     busybox_tests_with_args.push("echo \"aaaaaaa\" >> test.txt");
//     busybox_tests_with_args.push("echo \"2222222\" >> test.txt");
//     busybox_tests_with_args.push("echo \"1111111\" >> test.txt");
//     busybox_tests_with_args.push("echo \"bbbbbbb\" >> test.txt");
//     busybox_tests_with_args.push("sort test.txt | ./busybox uniq");
//     busybox_tests_with_args.push("stat test.txt");
//     busybox_tests_with_args.push("strings test.txt");
//     busybox_tests_with_args.push("wc test.txt");
//     busybox_tests_with_args.push("[ -f test.txt ]");
//     busybox_tests_with_args.push("more test.txt");
//     busybox_tests_with_args.push("rm test.txt");
//     busybox_tests_with_args.push("mkdir test_dir");
//     busybox_tests_with_args.push("mv test_dir test");
//     busybox_tests_with_args.push("rmdir test");
//     busybox_tests_with_args.push("grep hello busybox_cmd.txt");
//     busybox_tests_with_args.push("cp busybox_cmd.txt busybox_cmd.bak");
//     busybox_tests_with_args.push("rm busybox_cmd.bak");
//     busybox_tests_with_args.push("find -name \"busybox_cmd.txt\"");

//     for busybox_test_with_args in busybox_tests_with_args {
//         let args: Vec<_> = busybox_test_with_args.split(' ').collect();
//         // println!("args: {:#?}", args);
//         let args_copy: Vec<_> = args
//             .iter()
//             .filter(|&arg| !arg.is_empty())
//             .map(|&arg| {
//                 let mut string = String::new();
//                 string.push_str(arg);
//                 string.push('\0');
//                 string
//             })
//             .collect();
//         // println!("args_copy: {:#?}", args_copy);
//         let mut busybox_args: Vec<*const u8> = Vec::new();
//         busybox_args.push("./busybox\0".as_ptr());
//         for arg in args_copy.iter() {
//             busybox_args.push(arg.as_ptr());
//         }
//         busybox_args.push(core::ptr::null::<u8>());
//         let pid = fork();
//         if pid == 0 {
//             // println!("busybox_args: {:#?}", busybox_args);
//             exec("./busybox\0", busybox_args.as_slice());
//         } else {
//             let mut exit_code = 0;
//             waitpid(pid as usize, &mut exit_code);
//             if exit_code == 0 {
//                 println!("testcase busybox {} success", busybox_test_with_args);
//             } else {
//                 println!("testcase busybox {} fail", busybox_test_with_args);
//             }
//         }
//     }

//     // with busybox bash
//     // let mut busybox_args = Vec::new();
//     // busybox_args.push("busybox_testcode.sh\0".as_ptr());
//     // busybox_args.push(core::ptr::null::<u8>());
//     // let pid = fork();
//     // if pid == 0 {
//     //     exec("busybox_testcode.sh\0", busybox_args.as_slice());
//     // } else {
//     //     let mut exit_code = 0;
//     //     waitpid(pid as usize, &mut exit_code);
//     // }
// }

// fn lua_test() {
//     // no busybox bash
//     let mut lua_tests = Vec::new();
//     lua_tests.push("date.lua\0");
//     lua_tests.push("file_io.lua\0");
//     lua_tests.push("max_min.lua\0");
//     lua_tests.push("random.lua\0");
//     lua_tests.push("remove.lua\0");
//     lua_tests.push("round_num.lua\0");
//     lua_tests.push("sin30.lua\0");
//     lua_tests.push("sort.lua\0");
//     lua_tests.push("strings.lua\0");

//     let mut lua_args: Vec<*const u8> = Vec::new();
//     lua_args.push("./lua\0".as_ptr());
//     for lua_test in lua_tests {
//         lua_args.push(lua_test.as_ptr());
//         lua_args.push(core::ptr::null::<u8>());
//         let pid = fork();
//         if pid == 0 {
//             exec("./lua\0", lua_args.as_slice());
//         } else {
//             let mut exit_code = 0;
//             waitpid(pid as usize, &mut exit_code);
//             if exit_code == 0 {
//                 println!("testcase lua {} success", lua_test);
//             } else {
//                 println!("testcase lua {} fail", lua_test);
//             }
//         }
//         lua_args.pop();
//         lua_args.pop();
//     }

//     // with busybox bash
//     // let mut lua_tests = Vec::new();
//     // lua_tests.push("date.lua\0");
//     // lua_tests.push("file_io.lua\0");
//     // lua_tests.push("max_min.lua\0");
//     // lua_tests.push("random.lua\0");
//     // lua_tests.push("remove.lua\0");
//     // lua_tests.push("round_num.lua\0");
//     // lua_tests.push("sin30.lua\0");
//     // lua_tests.push("sort.lua\0");
//     // lua_tests.push("strings.lua\0");

//     // let mut lua_args: Vec<*const u8> = Vec::new();
//     // lua_args.push("./test.sh\0".as_ptr());
//     // for lua_test in lua_tests {
//     //     lua_args.push(lua_test.as_ptr());
//     //     lua_args.push(core::ptr::null::<u8>());
//     //     let pid = fork();
//     //     if pid == 0 {
//     //         exec("./test.sh\0", lua_args.as_slice());
//     //     } else {
//     //         let mut exit_code = 0;
//     //         waitpid(pid as usize, &mut exit_code);
//     //     }
//     //     lua_args.pop();
//     //     lua_args.pop();
//     // }
// }

// fn lmbench_test() {
//     // no busybox bash
//     let mut lmbench_tests_with_args = Vec::new();
//     lmbench_tests_with_args.push("lat_syscall -P 1 null\0");
//     lmbench_tests_with_args.push("read\0");
//     lmbench_tests_with_args.push("write\0");
//     lmbench_tests_with_args.push("random.lua\0");
//     lmbench_tests_with_args.push("remove.lua\0");
//     lmbench_tests_with_args.push("round_num.lua\0");
//     lmbench_tests_with_args.push("sin30.lua\0");
//     lmbench_tests_with_args.push("sort.lua\0");
//     lmbench_tests_with_args.push("strings.lua\0");

//     let mut lmbench_args: Vec<*const u8> = Vec::new();
//     lmbench_args.push("lmbench_all\0".as_ptr());
//     lmbench_args.push("lat_syscall\0".as_ptr());
//     lmbench_args.push("-P\0".as_ptr());
//     lmbench_args.push("1\0".as_ptr());
//     for lmbench_test in lmbench_tests_with_args {
//         lmbench_args.push(lmbench_test.as_ptr());
//         lmbench_args.push(core::ptr::null::<u8>());
//         let pid = fork();
//         if pid == 0 {
//             exec("lmbench_all\0", lmbench_args.as_slice());
//         } else {
//             let mut exit_code = 0;
//             waitpid(pid as usize, &mut exit_code);
//         }
//         lmbench_args.pop();
//         lmbench_args.pop();
//     }

//     // with busybox bash
//     let mut lmbench_args: Vec<*const u8> = Vec::new();
//     lmbench_args.push("lmbench_testcode.sh\0".as_ptr());
//     lmbench_args.push(core::ptr::null::<u8>());
//     let pid = fork();
//     if pid == 0 {
//         exec("lmbench_testcode.sh\0", lmbench_args.as_slice());
//     } else {
//         let mut exit_code = 0;
//         waitpid(pid as usize, &mut exit_code);
//     }
// }

// fn final_round_two_test() {
//     busybox_test();
//     lua_test();
//     // lmbench_test();
// }

// #[no_mangle]
// pub fn main() -> i32 {
//     println!("Rust user shell");
//     final_round_two_test();
//     0
// }

// #![no_std]
// #![no_main]
// #![allow(clippy::println_empty_string)]

// extern crate alloc;

// #[macro_use]
// extern crate user;

// const LF: u8 = 0x0au8;
// const CR: u8 = 0x0du8;
// const DL: u8 = 0x7fu8;
// const BS: u8 = 0x08u8;
// const LINE_START: &str = ">> ";

// use alloc::string::String;
// use alloc::vec::Vec;
// use user::console::getchar;
// use user::{close, dup, exec, fork, open, pipe, waitpid, OpenFlags};

// #[derive(Debug)]
// struct ProcessArguments {
//     input: String,
//     output: String,
//     args_copy: Vec<String>,
//     args_addr: Vec<*const u8>,
// }

// impl ProcessArguments {
//     pub fn new(command: &str) -> Self {
//         let args: Vec<_> = command.split(' ').collect();
//         let mut args_copy: Vec<String> = args
//             .iter()
//             .filter(|&arg| !arg.is_empty())
//             .map(|&arg| {
//                 let mut string = String::new();
//                 string.push_str(arg);
//                 string.push('\0');
//                 string
//             })
//             .collect();

//         // redirect input
//         let mut input = String::new();
//         if let Some((idx, _)) = args_copy
//             .iter()
//             .enumerate()
//             .find(|(_, arg)| arg.as_str() == "<\0")
//         {
//             input = args_copy[idx + 1].clone();
//             args_copy.drain(idx..=idx + 1);
//         }

//         // redirect output
//         let mut output = String::new();
//         if let Some((idx, _)) = args_copy
//             .iter()
//             .enumerate()
//             .find(|(_, arg)| arg.as_str() == ">\0")
//         {
//             output = args_copy[idx + 1].clone();
//             args_copy.drain(idx..=idx + 1);
//         }

//         let mut args_addr: Vec<*const u8> = args_copy.iter().map(|arg| arg.as_ptr()).collect();
//         args_addr.push(core::ptr::null::<u8>());

//         Self {
//             input,
//             output,
//             args_copy,
//             args_addr,
//         }
//     }
// }

// #[no_mangle]
// pub fn main() -> i32 {
//     println!("Rust user shell");
//     let mut line: String = String::new();
//     print!("{}", LINE_START);
//     loop {
//         let c = getchar();
//         match c {
//             LF | CR => {
//                 println!("");
//                 if !line.is_empty() {
//                     let splited: Vec<_> = line.as_str().split('|').collect();
//                     let process_arguments_list: Vec<_> = splited
//                         .iter()
//                         .map(|&cmd| ProcessArguments::new(cmd))
//                         .collect();
//                     let mut valid = true;
//                     for (i, process_args) in process_arguments_list.iter().enumerate() {
//                         if i == 0 {
//                             if !process_args.output.is_empty() {
//                                 valid = false;
//                             }
//                         } else if i == process_arguments_list.len() - 1 {
//                             if !process_args.input.is_empty() {
//                                 valid = false;
//                             }
//                         } else if !process_args.output.is_empty() || !process_args.input.is_empty()
//                         {
//                             valid = false;
//                         }
//                     }
//                     if process_arguments_list.len() == 1 {
//                         valid = true;
//                     }
//                     if !valid {
//                         println!("Invalid command: Inputs/Outputs cannot be correctly binded!");
//                     } else {
//                         // create pipes
//                         let mut pipes_fd: Vec<[usize; 2]> = Vec::new();
//                         if !process_arguments_list.is_empty() {
//                             for _ in 0..process_arguments_list.len() - 1 {
//                                 let mut pipe_fd = [0usize; 2];
//                                 pipe(&mut pipe_fd);
//                                 pipes_fd.push(pipe_fd);
//                             }
//                         }
//                         let mut children: Vec<_> = Vec::new();
//                         for (i, process_argument) in process_arguments_list.iter().enumerate() {
//                             let pid = fork();
//                             if pid == 0 {
//                                 let input = &process_argument.input;
//                                 let output = &process_argument.output;
//                                 let args_copy = &process_argument.args_copy;
//                                 let args_addr = &process_argument.args_addr;
//                                 // redirect input
//                                 if !input.is_empty() {
//                                     let input_fd = open(input.as_str(), OpenFlags::RDONLY);
//                                     if input_fd == -1 {
//                                         println!("Error when opening file {}", input);
//                                         return -4;
//                                     }
//                                     let input_fd = input_fd as usize;
//                                     close(0);
//                                     assert_eq!(dup(input_fd), 0);
//                                     close(input_fd);
//                                 }
//                                 // redirect output
//                                 if !output.is_empty() {
//                                     let output_fd = open(
//                                         output.as_str(),
//                                         OpenFlags::CREATE | OpenFlags::WRONLY,
//                                     );
//                                     if output_fd == -1 {
//                                         println!("Error when opening file {}", output);
//                                         return -4;
//                                     }
//                                     let output_fd = output_fd as usize;
//                                     close(1);
//                                     assert_eq!(dup(output_fd), 1);
//                                     close(output_fd);
//                                 }
//                                 // receive input from the previous process
//                                 if i > 0 {
//                                     close(0);
//                                     let read_end = pipes_fd.get(i - 1).unwrap()[0];
//                                     assert_eq!(dup(read_end), 0);
//                                 }
//                                 // send output to the next process
//                                 if i < process_arguments_list.len() - 1 {
//                                     close(1);
//                                     let write_end = pipes_fd.get(i).unwrap()[1];
//                                     assert_eq!(dup(write_end), 1);
//                                 }
//                                 // close all pipe ends inherited from the parent process
//                                 for pipe_fd in pipes_fd.iter() {
//                                     close(pipe_fd[0]);
//                                     close(pipe_fd[1]);
//                                 }
//                                 // execute new application
//                                 if exec(args_copy[0].as_str(), args_addr.as_slice()) == -1 {
//                                     println!("Error when executing!");
//                                     return -4;
//                                 }
//                                 unreachable!();
//                             } else {
//                                 children.push(pid);
//                             }
//                         }
//                         for pipe_fd in pipes_fd.iter() {
//                             close(pipe_fd[0]);
//                             close(pipe_fd[1]);
//                         }
//                         let mut exit_code: i32 = 0;
//                         for pid in children.into_iter() {
//                             let exit_pid = waitpid(pid as usize, &mut exit_code);
//                             assert_eq!(pid, exit_pid);
//                             //println!("Shell: Process {} exited with code {}", pid, exit_code);
//                         }
//                     }
//                     line.clear();
//                 }
//                 print!("{}", LINE_START);
//             }
//             BS | DL => {
//                 if !line.is_empty() {
//                     print!("{}", BS as char);
//                     print!(" ");
//                     print!("{}", BS as char);
//                     line.pop();
//                 }
//             }
//             _ => {
//                 print!("{}", c as char);
//                 line.push(c as char);
//             }
//         }
//     }
// }

#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use alloc::vec::Vec;
use user::{close, dup, exec, fork, open, pipe, waitpid, OpenFlags};

#[derive(Debug)]
struct ProcessArguments {
    input: String,
    output: String,
    args_copy: Vec<String>,
    args_addr: Vec<*const u8>,
}

impl ProcessArguments {
    pub fn new(command: &str) -> Self {
        let args: Vec<_> = command.split(' ').collect();
        let mut args_copy: Vec<String> = args
            .iter()
            .filter(|&arg| !arg.is_empty())
            .map(|&arg| {
                let mut string = String::new();
                string.push_str(arg);
                string.push('\0');
                string
            })
            .collect();

        // redirect input
        let mut input = String::new();
        if let Some((idx, _)) = args_copy
            .iter()
            .enumerate()
            .find(|(_, arg)| arg.as_str() == "<\0")
        {
            input = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        }

        // redirect output
        let mut output = String::new();
        if let Some((idx, _)) = args_copy
            .iter()
            .enumerate()
            .find(|(_, arg)| arg.as_str() == ">\0")
        {
            output = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        }

        let mut args_addr: Vec<*const u8> = args_copy.iter().map(|arg| arg.as_ptr()).collect();
        args_addr.push(core::ptr::null::<u8>());

        Self {
            input,
            output,
            args_copy,
            args_addr,
        }
    }
}

static BUSYBOX_LUA_TESTS: [&str; 63] = [
    "lua date.lua",
    "lua file_io.lua",
    "lua max_min.lua",
    "lua random.lua",
    "lua round_num.lua",
    "lua sin30.lua",
    "lua sort.lua",
    "lua strings.lua",
    "lua remove.lua",
    "busybox echo \"#### independent command test\"",
    "busybox ash -c exit",
    "busybox sh -c exit",
    "busybox basename /aaa/bbb",
    "busybox cal",
    "busybox clear",
    "busybox date",
    "busybox df",
    "busybox dirname /aaa/bbb",
    "busybox dmesg",
    "busybox du",
    "busybox expr 1 + 1",
    "busybox false",
    "busybox true",
    "busybox which ls",
    "busybox uname",
    "busybox uptime",
    "busybox printf \"abc\n\"",
    "busybox ps",
    "busybox pwd",
    "busybox free",
    "busybox hwclock",
    "busybox kill 10",
    "busybox ls",
    "busybox sleep 1",
    "busybox echo \"#### file opration test\"",
    "busybox mkdir test_dir",
    "busybox mv test_dir test",
    "busybox rmdir test",
    "busybox grep hello busybox_cmd.txt",
    "busybox cp busybox_cmd.txt busybox_cmd.bak",
    "busybox rm busybox_cmd.bak",
    "busybox find -name \"busybox_cmd.txt\"",
    "busybox touch test.txt",
    "busybox echo \"hello world\" > test.txt",
    "busybox cat test.txt",
    "busybox cut -c 3 test.txt",
    "busybox od test.txt",
    "busybox head test.txt",
    "busybox tail test.txt",
    "busybox hexdump -C test.txt",
    "busybox md5sum test.txt",
    "busybox echo \"ccccccc\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox echo \"aaaaaaa\" >> test.txt",
    "busybox echo \"2222222\" >> test.txt",
    "busybox echo \"1111111\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox stat test.txt",
    "busybox strings test.txt",
    "busybox wc test.txt",
    "busybox [ -f test.txt ]",
    "busybox more test.txt",
    "busybox rm test.txt",
    // "busybox sort test.txt | ./busybox uniq",
];

pub fn busybox_lua_tests() -> isize {
    for line in BUSYBOX_LUA_TESTS {
        let splited: Vec<_> = line.split('|').collect();
        let process_arguments_list: Vec<_> = splited
            .iter()
            .map(|&cmd| ProcessArguments::new(cmd))
            .collect();
        let mut valid = true;
        for (i, process_args) in process_arguments_list.iter().enumerate() {
            if i == 0 {
                if !process_args.output.is_empty() {
                    valid = false;
                }
            } else if i == process_arguments_list.len() - 1 {
                if !process_args.input.is_empty() {
                    valid = false;
                }
            } else if !process_args.output.is_empty() || !process_args.input.is_empty() {
                valid = false;
            }
        }
        if process_arguments_list.len() == 1 {
            valid = true;
        }
        if !valid {
            println!("Invalid command: Inputs/Outputs cannot be correctly binded!");
        } else {
            // create pipes
            let mut pipes_fd: Vec<[usize; 2]> = Vec::new();
            if !process_arguments_list.is_empty() {
                for _ in 0..process_arguments_list.len() - 1 {
                    let mut pipe_fd = [0usize; 2];
                    pipe(&mut pipe_fd);
                    pipes_fd.push(pipe_fd);
                }
            }
            let mut children: Vec<_> = Vec::new();
            for (i, process_argument) in process_arguments_list.iter().enumerate() {
                let pid = fork();
                if pid == 0 {
                    let input = &process_argument.input;
                    let output = &process_argument.output;
                    let args_copy = &process_argument.args_copy;
                    let args_addr = &process_argument.args_addr;
                    // redirect input
                    if !input.is_empty() {
                        let input_fd = open(input.as_str(), OpenFlags::RDONLY);
                        if input_fd == -1 {
                            println!("Error when opening file {}", input);
                            return -4;
                        }
                        let input_fd = input_fd as usize;
                        close(0);
                        assert_eq!(dup(input_fd), 0);
                        close(input_fd);
                    }
                    // redirect output
                    if !output.is_empty() {
                        let output_fd =
                            open(output.as_str(), OpenFlags::CREATE | OpenFlags::WRONLY);
                        if output_fd == -1 {
                            println!("Error when opening file {}", output);
                            return -4;
                        }
                        let output_fd = output_fd as usize;
                        close(1);
                        assert_eq!(dup(output_fd), 1);
                        close(output_fd);
                    }
                    // receive input from the previous process
                    if i > 0 {
                        close(0);
                        let read_end = pipes_fd.get(i - 1).unwrap()[0];
                        assert_eq!(dup(read_end), 0);
                    }
                    // send output to the next process
                    if i < process_arguments_list.len() - 1 {
                        close(1);
                        let write_end = pipes_fd.get(i).unwrap()[1];
                        assert_eq!(dup(write_end), 1);
                    }
                    // close all pipe ends inherited from the parent process
                    for pipe_fd in pipes_fd.iter() {
                        close(pipe_fd[0]);
                        close(pipe_fd[1]);
                    }
                    // execute new application
                    if exec(args_copy[0].as_str(), args_addr.as_slice()) == -1 {
                        println!("Error when executing!");
                        return -4;
                    }
                    unreachable!();
                } else {
                    children.push(pid);
                }
            }
            for pipe_fd in pipes_fd.iter() {
                close(pipe_fd[0]);
                close(pipe_fd[1]);
            }
            let mut exit_code: i32 = 0;
            for pid in children.into_iter() {
                let exit_pid = waitpid(pid as usize, &mut exit_code);
                assert_eq!(pid, exit_pid);
                // println!("pid: {}", pid);
                if pid == 2 {
                    println!("testcase {} success", line);
                }
            }
        }
    }
    0
}

static LMBENCH_TESTS: [&str; 1] = [
    // "busybox echo latency measurements",
    // "lmbench_all lat_syscall -P 1 null",
    // "lmbench_all lat_syscall -P 1 read",
    // "lmbench_all lat_syscall -P 1 write",
    // "busybox mkdir /var/tmp",
    // "busybox touch /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 stat /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 fstat /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 open /var/tmp/lmbench",
    // "lmbench_all lat_select -n 100 -P 1 file", // Could not create temp file lat_selectdnN7do
    // "lmbench_all lat_sig -P 1 install",
    // "lmbench_all lat_sig -P 1 catch",    // Exception(StorePageFault) process bomb
    // "lmbench_all lat_sig -P 1 prot lat_sig", // Exception(StorePageFault) process bomb
    // "lmbench_all lat_pipe -P 1", // Shit No pages
    // "lmbench_all lat_proc -P 1 fork",
    // "lmbench_all lat_proc -P 1 exec",
    // "busybox cp hello /tmp",
    // "lmbench_all lat_proc -P 1 shell",
    // "lmbench_all lmdd label=\"File /var/tmp/XXX write bandwidth:\" of=/var/tmp/XXX move=645m fsync=1 print=3",
    // "lmbench_all lat_pagefault -P 1 /var/tmp/XXX",
    // "lmbench_all lat_mmap -P 1 512k /var/tmp/XXX",
    // "busybox echo file system latency",
    // "lmbench_all lat_fs /var/tmp",
    // "busybox echo Bandwidth measurements",
    // "lmbench_all bw_pipe -P 1",
    // "lmbench_all bw_file_rd -P 1 512k io_only /var/tmp/XXX",
    // "lmbench_all bw_file_rd -P 1 512k open2close /var/tmp/XXX",
    // "lmbench_all bw_mmap_rd -P 1 512k mmap_only /var/tmp/XXX",
    // "lmbench_all bw_mmap_rd -P 1 512k open2close /var/tmp/XXX",
    // "busybox echo context switch overhead",
    "lmbench_all lat_ctx -P 1 -s 32 2 4 8 16 24 32 64 96",
];

pub fn lmbench_tests() -> isize {
    for line in LMBENCH_TESTS {
        let splited: Vec<_> = line.split('|').collect();
        let process_arguments_list: Vec<_> = splited
            .iter()
            .map(|&cmd| ProcessArguments::new(cmd))
            .collect();
        let mut valid = true;
        for (i, process_args) in process_arguments_list.iter().enumerate() {
            if i == 0 {
                if !process_args.output.is_empty() {
                    valid = false;
                }
            } else if i == process_arguments_list.len() - 1 {
                if !process_args.input.is_empty() {
                    valid = false;
                }
            } else if !process_args.output.is_empty() || !process_args.input.is_empty() {
                valid = false;
            }
        }
        if process_arguments_list.len() == 1 {
            valid = true;
        }
        if !valid {
            println!("Invalid command: Inputs/Outputs cannot be correctly binded!");
        } else {
            // create pipes
            let mut pipes_fd: Vec<[usize; 2]> = Vec::new();
            if !process_arguments_list.is_empty() {
                for _ in 0..process_arguments_list.len() - 1 {
                    let mut pipe_fd = [0usize; 2];
                    pipe(&mut pipe_fd);
                    pipes_fd.push(pipe_fd);
                }
            }
            let mut children: Vec<_> = Vec::new();
            for (i, process_argument) in process_arguments_list.iter().enumerate() {
                let pid = fork();
                if pid == 0 {
                    let input = &process_argument.input;
                    let output = &process_argument.output;
                    let args_copy = &process_argument.args_copy;
                    let args_addr = &process_argument.args_addr;
                    // redirect input
                    if !input.is_empty() {
                        let input_fd = open(input.as_str(), OpenFlags::RDONLY);
                        if input_fd == -1 {
                            println!("Error when opening file {}", input);
                            return -4;
                        }
                        let input_fd = input_fd as usize;
                        close(0);
                        assert_eq!(dup(input_fd), 0);
                        close(input_fd);
                    }
                    // redirect output
                    if !output.is_empty() {
                        let output_fd =
                            open(output.as_str(), OpenFlags::CREATE | OpenFlags::WRONLY);
                        if output_fd == -1 {
                            println!("Error when opening file {}", output);
                            return -4;
                        }
                        let output_fd = output_fd as usize;
                        close(1);
                        assert_eq!(dup(output_fd), 1);
                        close(output_fd);
                    }
                    // receive input from the previous process
                    if i > 0 {
                        close(0);
                        let read_end = pipes_fd.get(i - 1).unwrap()[0];
                        assert_eq!(dup(read_end), 0);
                    }
                    // send output to the next process
                    if i < process_arguments_list.len() - 1 {
                        close(1);
                        let write_end = pipes_fd.get(i).unwrap()[1];
                        assert_eq!(dup(write_end), 1);
                    }
                    // close all pipe ends inherited from the parent process
                    for pipe_fd in pipes_fd.iter() {
                        close(pipe_fd[0]);
                        close(pipe_fd[1]);
                    }
                    // execute new application
                    if exec(args_copy[0].as_str(), args_addr.as_slice()) == -1 {
                        println!("Error when executing!");
                        return -4;
                    }
                    unreachable!();
                } else {
                    children.push(pid);
                }
            }
            for pipe_fd in pipes_fd.iter() {
                close(pipe_fd[0]);
                close(pipe_fd[1]);
            }
            let mut exit_code: i32 = 0;
            for pid in children.into_iter() {
                let exit_pid = waitpid(pid as usize, &mut exit_code);
                assert_eq!(pid, exit_pid);
            }
        }
    }
    0
}

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    // busybox_lua_tests();
    lmbench_tests();
    0
}
