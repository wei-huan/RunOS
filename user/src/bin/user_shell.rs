// #![no_std]
// #![no_main]
// #![allow(clippy::println_empty_string)]

// extern crate alloc;

// #[macro_use]
// extern crate user;

// use alloc::vec::Vec;
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
//     // static_tests.push("setjmp\0");
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
//     // static_tests.push("fflush_exit\0"); syscall 67
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
//     // dynamic_tests.push("setjmp\0");
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
//     dynamic_tests.push("wcsstr\0");
//     dynamic_tests.push("wcstol\0");
//     // dynamic_tests.push("daemon_failure\0");
//     dynamic_tests.push("dn_expand_empty\0");
//     dynamic_tests.push("dn_expand_ptr_0\0");
//     // dynamic_tests.push("fflush_exit\0");     // Unsupported syscall_id: 67
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

// #[no_mangle]
// pub fn main() -> i32 {
//     println!("Rust user shell");
//     final_round_one_test();
//     0
// }

#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
const LINE_START: &str = ">> ";

use alloc::string::String;
use alloc::vec::Vec;
use user::console::getchar;
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

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!("{}", LINE_START);
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    let splited: Vec<_> = line.as_str().split('|').collect();
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
                        } else if !process_args.output.is_empty() || !process_args.input.is_empty()
                        {
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
                                    let output_fd = open(
                                        output.as_str(),
                                        OpenFlags::CREATE | OpenFlags::WRONLY,
                                    );
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
                            //println!("Shell: Process {} exited with code {}", pid, exit_code);
                        }
                    }
                    line.clear();
                }
                print!("{}", LINE_START);
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
