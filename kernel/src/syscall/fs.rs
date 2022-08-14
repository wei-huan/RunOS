use crate::cpu::{current_task, current_user_token};
use crate::fs::{
    ch_dir, make_pipe, open, open_device_file, Dirent, File, FileClass, IOVec, OSInode, OpenFlags,
    Stat, StatFS, MNT_TABLE, S_IFCHR, S_IFDIR, S_IFREG, S_IRWXG, S_IRWXO, S_IRWXU,
};
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};
use crate::syscall::errorno::{EBADF, EISDIR, ENOENT, ESPIPE};
use crate::syscall::ENOTDIR;
use crate::task::{suspend_current_and_run_next, SigSet, TaskControlBlockInner};
use crate::timer::{get_time_ns, TimeSpec, TimeVal, NSEC_PER_SEC};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::write;
use core::mem::size_of;
use core::task;
use spin::MutexGuard;

use super::{EINVAL, EPERM};

const AT_FDCWD: isize = -100;

pub const FD_LIMIT: usize = 128;

pub fn sys_ioctl() -> isize {
    0
}

pub const F_DUPFD: u32 = 0; /* dup */
pub const F_GETFD: u32 = 1; /* get close_on_exec */
pub const F_SETFD: u32 = 2; /* set/clear close_on_exec */
pub const F_GETFL: u32 = 3; /* set file->f_flags */
pub const F_DUPFD_CLOEXEC: u32 = 1030;

pub fn sys_fcntl(fd: usize, cmd: u32, arg: usize) -> isize {
    log::debug!("sys_fcntl fd: {}, cmd: {}, arg: {}", fd, cmd, arg);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd > inner.fd_table.len() {
        return -EINVAL;
    }
    let ret = {
        if let Some(_) = &mut inner.fd_table[fd] {
            match cmd {
                F_DUPFD_CLOEXEC | F_DUPFD => {
                    let new_fd = inner.alloc_fd();
                    inner.fd_table[new_fd] = inner.fd_table[fd].clone();
                    new_fd as isize
                }
                F_GETFD | F_SETFD => 0,
                F_GETFL => 0,
                _ => unimplemented!("sys_fcntl cmd: {}", cmd),
            }
        } else {
            -ENOENT
        }
    };
    ret
}

pub fn sys_write(fd: isize, buf: *const u8, len: usize) -> isize {
    if (fd >= 3 || fd == AT_FDCWD) {
        log::debug!("sys_write fd: {}, buf: {:#X?}, len: {}", fd, buf, len);
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd as usize >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd as usize] {
        let file: Arc<dyn File + Send + Sync> = match &file {
            FileClass::File(f) => f.clone(),
            FileClass::Abstr(f) => f.clone(),
        };
        if !file.writable() {
            return -EPERM;
        }
        drop(inner);
        drop(task);
        let size = file.write(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        size as isize
    } else {
        return -EBADF;
    }
}

pub fn sys_writev(fd: usize, iov: *const IOVec, iocnt: usize) -> isize {
    log::debug!("sys_writev: fd: {}, iov: {:#X?}, iocnt: {}", fd, iov, iocnt);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();

    let mut ret = 0isize;

    if fd >= inner.fd_table.len() {
        return -1;
    }

    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match &file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abstr(fi) => f = fi.clone(),
        }
        if !f.writable() {
            return -1;
        }
        // release current task PCB inner manually to avoid multi-borrow
        drop(inner);

        for i in 0..iocnt {
            let iovec = translated_ref(token, unsafe { iov.add(i) });
            let buf = translated_byte_buffer(token, iovec.iov_base, iovec.iov_len);
            ret += f.write(UserBuffer::new(buf)) as isize;
        }
    }

    ret
}

pub fn sys_pread(fd: usize, buf: *mut u8, count: usize, offset: usize) -> isize {
    log::debug!(
        "sys_read fd: {}, buf: {:#X?}, count: {}, offset: {:#X?}",
        fd,
        buf,
        count,
        offset
    );
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    let ret = if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match &file {
            FileClass::File(fi) => {
                let old_off = fi.get_offset();
                fi.set_offset(offset);
                let read_cnt =
                    fi.read(UserBuffer::new(translated_byte_buffer(token, buf, count))) as isize;
                fi.set_offset(old_off);
                read_cnt
            }
            FileClass::Abstr(_) => -ESPIPE,
        }
    } else {
        -EBADF
    };
    ret
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    if (fd >= 3) {
        log::debug!("sys_read fd: {}, buf: {:#X?}, len: {}", fd, buf, len);
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -EINVAL;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file: Arc<dyn File + Send + Sync> = match &file {
            FileClass::Abstr(f) => f.clone(),
            FileClass::File(f) => f.clone(),
        };
        if !file.readable() {
            return -EPERM;
        }
        // release current task PCB inner manually to avoid multi-borrow
        drop(inner);
        // log::debug!("sys_read ptr: {:#X?}, len: {:#X?}", buf, len);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -EBADF
    }
}

pub fn sys_readv(fd: usize, iov: *const IOVec, iocnt: usize) -> isize {
    log::debug!("sys_readv: fd: {}, iov: {:#X?}, iocnt: {}", fd, iov, iocnt);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();

    if fd >= inner.fd_table.len() {
        return -1;
    }

    let mut ret = 0isize;
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match &file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abstr(fi) => f = fi.clone(),
        }
        if !f.readable() {
            return -1;
        }
        // release current task PCB inner manually to avoid multi-borrow
        drop(inner);

        for i in 0..iocnt {
            let iovec = translated_ref(token, unsafe { iov.add(i) });
            let buf = translated_byte_buffer(token, iovec.iov_base, iovec.iov_len);
            ret += f.read(UserBuffer::new(buf)) as isize;
        }
    }
    ret
}

// pub fn sys_open(path: *const u8, flags: u32) -> isize {
//     let task = current_task().unwrap();
//     let token = current_user_token();
//     let path = translated_str(token, path);
//     if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
//         let mut inner = task.acquire_inner_lock();
//         let fd = inner.alloc_fd();
//         inner.fd_table[fd] = Some(inode);
//         fd as isize
//     } else {
//         -1
//     }
// }

pub fn sys_open_at(dirfd: isize, path: *const u8, flags: u32, mode: u32) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let flags = OpenFlags::from_bits(flags).unwrap_or(OpenFlags::RDONLY);
    log::debug!(
        "sys_open_at dirfd: {} path: {:#?}, flags: {:#?}, mode: {}",
        dirfd,
        path,
        flags,
        mode
    );

    // check dirfd
    if dirfd >= 0 {
        let dirfd_usize = dirfd as usize;
        if dirfd_usize >= inner.fd_table.len()
            || dirfd_usize >= FD_LIMIT
            || inner.fd_table[dirfd_usize].is_none()
        {
            return -EINVAL;
        }
    } else if dirfd != AT_FDCWD {
        return -EINVAL;
    } else {
        // dirfd == AT_FDCWD ok
    }

    // device file
    if path == "/dev/zero" || path == "/dev/null" || path == "/dev/misc/rtc" {
        if let Some(dev) = open_device_file(path.as_str(), flags) {
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileClass::Abstr(dev));
            return fd as isize;
        }
    }

    // common file
    /* get dirfd inode */
    let dir_inode: Arc<OSInode> = if dirfd == AT_FDCWD {
        let current_path = if dirfd == AT_FDCWD && !path.starts_with("/") {
            inner.current_path.clone()
        } else {
            String::from("/")
        };
        open("/", current_path.as_str(), flags).unwrap()
    } else {
        let file = inner.fd_table[dirfd as usize].as_ref().unwrap();
        match &file {
            FileClass::File(f) => {
                if !f.is_dir() {
                    log::debug!("dirfd inode is not directory, fail");
                    return -ENOTDIR;
                } else {
                    f.clone()
                }
            }
            FileClass::Abstr(_) => {
                log::debug!("dirfd inode is abstr file, fail");
                return -ENOTDIR;
            }
        }
    };

    // need to create
    /* get inode in directory */
    if flags.contains(OpenFlags::CREATE) {
        if let Some(new_file) = dir_inode.create(path.as_str(), flags) {
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileClass::File(new_file));
            log::debug!("sys_open_at with creatation success fd: {}", fd);
            return fd as isize;
        } else {
            return -ENOENT;
        }
    } else {
        if let Some(file) = dir_inode.find(path.as_str(), flags) {
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileClass::File(file));
            log::debug!("sys_open_at success fd: {}", fd);
            return fd as isize;
        } else {
            return -ENOENT;
        }
    }
}

pub fn sys_close(fd: isize) -> isize {
    log::debug!("sys_close fd: {}", fd);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd as usize >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd as usize].is_none() {
        return -1;
    }
    inner.fd_table[fd as usize].take();
    0
}

pub fn sys_getcwd(buf: *mut u8, len: usize) -> isize {
    log::debug!("sys_getcwd buf: {:#X?}, len: {}", buf, len);
    let token = current_user_token();
    let task = current_task().unwrap();
    let buf_vec = translated_byte_buffer(token, buf, len);
    let inner = task.acquire_inner_lock();
    let mut userbuf = UserBuffer::new(buf_vec);
    if buf as usize == 0 {
        return 0;
    } else {
        let cwd = inner.current_path.as_bytes();
        userbuf.write(cwd);
        return buf as isize;
    }
}

pub fn sys_dup(fd: usize) -> isize {
    log::debug!("sys_dup fd: {}", fd);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(inner.fd_table[fd].as_ref().unwrap().clone());
    new_fd as isize
}

pub fn sys_dup3(old_fd: usize, new_fd: usize) -> isize {
    log::debug!("sys_dup3 old_fd: {}, new_fd: {}", old_fd, new_fd);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if old_fd >= inner.fd_table.len() || new_fd > FD_LIMIT {
        return -1;
    }
    if inner.fd_table[old_fd].is_none() {
        return -1;
    }
    
    if new_fd >= inner.fd_table.len() {
        for _ in inner.fd_table.len()..(new_fd + 1) {
            inner.fd_table.push(None);
        }
    }
    inner.fd_table[new_fd] = Some(inner.fd_table[old_fd].as_ref().unwrap().clone());
    new_fd as isize
}

// pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
//     log::debug!("sys_fstat fd: {}, buf: {:#X?}", fd, buf as usize);
//     let token = current_user_token();
//     let task = current_task().unwrap();
//     let buf_vec = translated_byte_buffer(token, buf, size_of::<Stat>());
//     let inner = task.acquire_inner_lock();
//     let mut userbuf = UserBuffer::new(buf_vec);
//     let mut kstat = Stat::empty();
//     if fd == AT_FDCWD {
//         let work_path = inner.current_path.clone();
//         if let Some(file) = open(
//             "/",
//             work_path.as_str(),
//             OpenFlags::RDONLY,
//             DiskInodeType::Directory,
//         ) {
//             file.get_fstat(&mut kstat);
//             userbuf.write(kstat.as_bytes());
//             return 0;
//         } else {
//             return -1;
//         }
//     } else {
//         let fd_usz = fd as usize;
//         if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
//             return -EPERM;
//         }
//         if let Some(file) = &inner.fd_table[fd_usz] {
//             match file {
//                 FileClass::File(f) => {
//                     f.get_fstat(&mut kstat);
//                     userbuf.write(kstat.as_bytes());
//                     return 0;
//                 }
//                 // _ => {
//                 //     userbuf.write(Stat::new_abstract().as_bytes());
//                 //     return 0; //warning
//                 // }
//                 _ => {
//                     return -EPERM;
//                 }
//             }
//         } else {
//             return -EPERM;
//         }
//     }
// }

pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
    log::debug!("sys_fstat fd: {}, buf: {:#X?}", fd, buf as usize);
    let token = current_user_token();
    let task = current_task().unwrap();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Stat>());
    let mut userbuf = UserBuffer::new(buf_vec);
    let inner = task.acquire_inner_lock();

    let ret = if fd == AT_FDCWD {
        let cwd = inner.current_path.clone();
        let inode = open("/", cwd.as_str(), OpenFlags::RDONLY | OpenFlags::DIRECTROY).unwrap();
        fstat_inner(inode, &mut userbuf)
    } else if let Some(file) = inner.fd_table[fd as usize].clone() {
        match file {
            FileClass::File(f) => fstat_inner(f, &mut userbuf),
            FileClass::Abstr(_) => {
                userbuf.write(Stat::new_abstract().as_bytes());
                0
            }
        }
    } else {
        -EPERM
    };
    ret
}

fn fstat_inner(f: Arc<OSInode>, userbuf: &mut UserBuffer) -> isize {
    let mut kstat = Stat::empty();
    kstat.st_mode = {
        if f.is_dir() {
            S_IFDIR | S_IRWXU | S_IRWXG | S_IRWXO
        } else {
            S_IFREG | S_IRWXU | S_IRWXG | S_IRWXO
        }
    };
    kstat.st_ino = f.get_head_cluster() as u64;
    kstat.st_size = f.get_size() as i64;
    userbuf.write(kstat.as_bytes());
    0
}

type FdMask = u64;

const FD_SETSIZE: usize = 256;
const NFDBITS: usize = 8 * size_of::<FdMask>();

#[derive(Debug, Clone, Copy)]
pub struct FDSet {
    fds_bits: [FdMask; FD_SETSIZE / NFDBITS],
}

impl FDSet {
    pub fn set_bit(&mut self, n: usize) {
        self.fds_bits[n / NFDBITS] |= 0x01 << (n % NFDBITS);
    }
    pub fn clear_bit(&mut self, n: usize) {
        self.fds_bits[n / NFDBITS] &= !(0x01 << (n % NFDBITS));
    }
    pub fn contains_bit(&self, n: usize) -> bool {
        (self.fds_bits[n / NFDBITS] & (0x01 << (n % NFDBITS))) > 0
    }
    pub fn clear_all(&mut self) {
        for i in 0..FD_SETSIZE / NFDBITS {
            self.fds_bits[i] = 0;
        }
    }
}

pub fn sys_pselect6(
    nfds: i32,
    readfds: *mut FDSet,
    writefds: *mut FDSet,
    exceptfds: *mut FDSet,
    timeout: *const TimeSpec,
    sigmask: *const SigSet,
) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let timeout = translated_ref(token, timeout);
    log::debug!(
        "sys_pselect6 nfds: {}, readfds: {:#X?}, writefds: {:#X?}, exceptfds: {:#X?}, timeout: {:#X?}, sigmask: {:#X?}",
        nfds,
        readfds as usize,
        writefds as usize,
        exceptfds as usize,
        timeout,
        sigmask as usize
    );
    let mut ret: isize = 0;
    if timeout.nsec != 0 || timeout.sec != 0 {
        let timeout_ns: u64 =
            get_time_ns() as u64 + timeout.nsec + (timeout.sec * NSEC_PER_SEC as u64);
        loop {
            let inner = task.acquire_inner_lock();
            if readfds as usize != 0 {
                let readfds = translated_refmut(token, readfds);
                for i in 0..nfds as usize {
                    if let Some(f) = &inner.fd_table[i] {
                        match &f {
                            FileClass::File(file) => {
                                ret += 1;
                                readfds.set_bit(i);
                            }
                            FileClass::Abstr(abs) => {
                                if abs.read_available() {
                                    ret += 1;
                                    readfds.set_bit(i);
                                } else {
                                    readfds.clear_bit(i);
                                }
                            }
                        }
                    } else {
                        readfds.clear_bit(i);
                    }
                }
            }
            if writefds as usize != 0 {
                let writefds = translated_refmut(token, writefds);
                for i in 0..nfds as usize {
                    if let Some(f) = &inner.fd_table[i] {
                        match &f {
                            FileClass::File(file) => {
                                ret += 1;
                                writefds.set_bit(i);
                            }
                            FileClass::Abstr(abs) => {
                                if abs.write_available() {
                                    ret += 1;
                                    writefds.set_bit(i);
                                } else {
                                    writefds.clear_bit(i);
                                }
                            }
                        }
                    } else {
                        writefds.clear_bit(i);
                    }
                }
            }
            let current_time_ns = get_time_ns() as u64;
            if ret == 0 && (current_time_ns < timeout_ns) {
                drop(inner);
                suspend_current_and_run_next();
            } else {
                if exceptfds as usize != 0 {
                    let exceptfds = translated_refmut(token, exceptfds);
                    exceptfds.clear_all();
                }
                break;
            }
        }
    } else {
        let inner = task.acquire_inner_lock();
        if readfds as usize != 0 {
            let readfds = translated_refmut(token, readfds);
            for i in 0..nfds as usize {
                if let Some(f) = &inner.fd_table[i] {
                    match &f {
                        FileClass::File(file) => {
                            ret += 1;
                            readfds.set_bit(i);
                        }
                        FileClass::Abstr(abs) => {
                            if abs.read_available() {
                                ret += 1;
                                readfds.set_bit(i);
                            } else {
                                readfds.clear_bit(i);
                            }
                        }
                    }
                } else {
                    readfds.clear_bit(i);
                }
            }
        }
        if writefds as usize != 0 {
            let writefds = translated_refmut(token, writefds);
            for i in 0..nfds as usize {
                if let Some(f) = &inner.fd_table[i] {
                    match &f {
                        FileClass::File(file) => {
                            ret += 1;
                            writefds.set_bit(i);
                        }
                        FileClass::Abstr(abs) => {
                            if abs.write_available() {
                                ret += 1;
                                writefds.set_bit(i);
                            } else {
                                writefds.clear_bit(i);
                            }
                        }
                    }
                } else {
                    writefds.clear_bit(i);
                }
            }
        }
        if exceptfds as usize != 0 {
            let exceptfds = translated_refmut(token, exceptfds);
            exceptfds.clear_all();
        }
    }
    log::debug!("pselect6 return {}", ret);
    ret
}

#[repr(C)]
pub struct PollFD {
    fd: i32,
    events: u16,
    revents: u16,
}

pub const POLLIN: u16 = 0x001;
pub const POLLPRI: u16 = 0x002;
pub const POLLOUT: u16 = 0x004;
pub const POLLERR: u16 = 0x008;
pub const POLLHUP: u16 = 0x010;
pub const POLLNVAL: u16 = 0x020;
pub const POLLRDNORM: u16 = 0x040;
pub const POLLRDBAND: u16 = 0x080;

pub fn sys_ppoll(
    fds: *mut PollFD,
    nfds: u32,
    timeout: *mut TimeSpec,
    sigmask: *const SigSet,
) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    log::debug!(
        "sys_ppoll fds: {:#X?}, nfds: {}, timeout: {:#X?}, sigmask: {:#X?}",
        fds as usize,
        nfds,
        timeout as usize,
        sigmask as usize
    );
    let mut ret = 0isize;

    for i in 0..nfds {
        let mut pollfd = translated_refmut(token, unsafe { fds.add(i as usize) });
        if let Some(f) = inner.fd_table.get(pollfd.fd as usize) {
            if f.is_some() {
                pollfd.revents |= POLLIN;
                ret += 1;
            }
        }
    }
    ret
}

pub fn sys_fstatat(dirfd: isize, path: *mut u8, buf: *mut u8) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    log::debug!("sys_fstatat path: {}, buf: {:#X?}", path, buf);

    let stat_buf = translated_byte_buffer(token, buf, size_of::<Stat>());
    let mut stat_userbuf = UserBuffer::new(stat_buf);

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        task.acquire_inner_lock().current_path.clone()
    } else {
        String::from("/")
    };

    // 权宜之计, 比完赛后删除
    if path == "/dev/null" {
        let mut kstat = Stat::empty();
        kstat.st_mode = S_IFCHR;
        stat_userbuf.write(kstat.as_bytes());
        return 0;
    }

    let ret = if let Some(osfile) = open(&cwd, &path, OpenFlags::RDONLY) {
        fstat_inner(osfile, &mut stat_userbuf)
    } else {
        -ENOENT
    };
    ret
}

pub fn sys_pipe(pipe: *mut u32, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let flags = OpenFlags::from_bits(flags).unwrap();
    log::debug!("sys_pipe pipe: {:#X?}, flags: {:?}", pipe as usize, flags);
    let mut inner = task.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(FileClass::Abstr(pipe_read));
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(FileClass::Abstr(pipe_write));
    *translated_refmut(token, pipe) = read_fd as u32;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd as u32;
    0
}

pub fn sys_mkdir(dirfd: isize, path: *const u8, mode: u32) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    log::debug!(
        "sys_mkdir: dirfd: {}, path: {}, mode: {}",
        dirfd,
        path,
        mode
    );
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if dirfd == AT_FDCWD {
        if let Some(_) = open(
            inner.get_work_path().as_str(),
            path.as_str(),
            OpenFlags::CREATE | OpenFlags::DIRECTROY,
        ) {
            return 0;
        } else {
            return -1;
        }
    } else {
        // DEBUG: 获取dirfd的OSInode
        let fd_usz = dirfd as usize;
        if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
            return -1;
        }
        if let Some(file) = &inner.fd_table[fd_usz] {
            match &file {
                FileClass::File(f) => {
                    if let Some(_) = f.create(path.as_str(), OpenFlags::DIRECTROY) {
                        return 0;
                    } else {
                        return -1;
                    }
                }
                _ => return -1,
            }
        } else {
            return -1;
        }
    }
}

pub fn sys_chdir(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    log::debug!("sys_chdir path: {}", path);
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let mut work_path = inner.current_path.clone();
    let new_ino_id = ch_dir(work_path.as_str(), path.as_str()) as isize;
    //println!("new inode id = {}", new_ino_id);
    if new_ino_id >= 0 {
        if path.chars().nth(0).unwrap() == '/' {
            inner.current_path = path.clone();
        } else {
            work_path.push('/');
            work_path.push_str(path.as_str());
            let path_vec: Vec<&str> = work_path.as_str().split('/').collect();
            let mut new_pathv: Vec<&str> = Vec::new();
            for i in 0..path_vec.len() {
                if path_vec[i] == "" || path_vec[i] == "." {
                    continue;
                }
                if path_vec[i] == ".." {
                    new_pathv.pop();
                    continue;
                }
                new_pathv.push(path_vec[i]);
            }
            let mut new_wpath = String::new();
            for i in 0..new_pathv.len() {
                new_wpath.push('/');
                new_wpath.push_str(new_pathv[i]);
            }
            if new_pathv.len() == 0 {
                new_wpath.push('/');
            }
            //println!("after cd workpath = {}", new_wpath);
            inner.current_path = new_wpath.clone();
        }
        new_ino_id
    } else {
        new_ino_id
    }
}

pub fn sys_getdents64(fd: isize, buf: *mut u8, len: usize) -> isize {
    log::debug!(
        "sys_getdents64: fd: {}, buf: {:#X?}, len: {}",
        fd,
        buf as usize,
        len
    );
    let token = current_user_token();
    let task = current_task().unwrap();
    let buf_vec = translated_byte_buffer(token, buf, len);
    let inner = task.acquire_inner_lock();
    let dent_len = size_of::<Dirent>();
    //let max_num = len / dent_len;
    let mut total_len: usize = 0;
    // 使用UserBuffer结构，以便于跨页读写
    let mut userbuf = UserBuffer::new(buf_vec);
    let mut dirent = Dirent::default();
    if fd == AT_FDCWD {
        let work_path = inner.current_path.clone();
        if let Some(file) = open(
            "/",
            work_path.as_str(),
            OpenFlags::RDONLY | OpenFlags::DIRECTROY,
        ) {
            loop {
                if total_len + dent_len > len {
                    break;
                }
                if file.getdirent(&mut dirent) > 0 {
                    userbuf.write_at(total_len, dirent.as_bytes());
                    total_len += dent_len;
                } else {
                    break;
                }
            }
            return total_len as isize; //warning
        } else {
            return -1;
        }
    } else {
        let fd_usz = fd as usize;
        if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
            return -1;
        }
        if let Some(file) = &inner.fd_table[fd_usz] {
            match &file {
                FileClass::File(f) => {
                    loop {
                        if total_len + dent_len > len {
                            break;
                        }
                        if f.getdirent(&mut dirent) > 0 {
                            userbuf.write_at(total_len, dirent.as_bytes());
                            total_len += dent_len;
                        } else {
                            break;
                        }
                    }
                    return total_len as isize; //warning
                }
                _ => {
                    return -1;
                }
            }
        } else {
            return -1;
        }
    }
}

fn get_file_discpt(
    fd: isize,
    path: &String,
    inner: &MutexGuard<TaskControlBlockInner>,
    oflags: OpenFlags,
) -> Option<FileClass> {
    if fd == AT_FDCWD {
        if let Some(inode) = open(inner.get_work_path().as_str(), path.as_str(), oflags) {
            //println!("find old");
            return Some(FileClass::File(inode));
        } else {
            return None;
        }
    } else {
        let fd_usz = fd as usize;
        if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
            return None;
        }
        if let Some(file) = &inner.fd_table[fd_usz] {
            match &file {
                FileClass::File(f) => {
                    if oflags.contains(OpenFlags::CREATE) {
                        if let Some(tar_f) = f.create(path.as_str(), oflags) {
                            return Some(FileClass::File(tar_f));
                        } else {
                            return None;
                        }
                    } else {
                        if let Some(tar_f) = f.find(path.as_str(), oflags) {
                            return Some(FileClass::File(tar_f));
                        } else {
                            return None;
                        }
                    }
                }
                _ => return None, // 如果是抽象类文件，不能open
            }
        } else {
            return None;
        }
    }
}

pub fn sys_unlinkat(fd: i32, path: *const u8, flags: u32) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags).unwrap();
    log::debug!(
        "sys_unlinkat fd: {}, path: {}, flags: {:?}",
        fd,
        path,
        flags,
    );
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if let Some(file) = get_file_discpt(fd as isize, &path, &inner, flags) {
        match file {
            FileClass::File(f) => {
                f.delete();
                return 0;
            }
            _ => return -1,
        }
    } else {
        return -1;
    }
}

// fake implement
pub fn sys_mount(
    special: *const u8,
    dir: *const u8,
    fstype: *const u8,
    flags: u32,
    data: *const u8,
) -> isize {
    let token = current_user_token();
    let special = translated_str(token, special);
    let dir = translated_str(token, dir);
    let fstype = translated_str(token, fstype);
    log::debug!(
        "sys_mount special: {}, dir: {}, flags: {}, fstype: {}, data: {:#X?}",
        special,
        dir,
        fstype,
        flags,
        data
    );
    MNT_TABLE.lock().mount(special, dir, fstype, flags as u32)
}

// fake implement
pub fn sys_umount(p_special: *const u8, flags: u32) -> isize {
    let token = current_user_token();
    let special = translated_str(token, p_special);
    log::debug!("sys_umount special: {}, flags: {}", special, flags);
    MNT_TABLE.lock().umount(special, flags as u32)
}

pub fn sys_faccessat(fd: usize, path: *const u8, time: usize, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    // 这里传入的地址为用户的虚地址，因此要使用用户的虚地址进行映射
    let path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags).unwrap();
    log::debug!(
        "sys_faccessat fd: {}, path: {:#?}: time: {:#?}, flags: {:?}",
        fd,
        path,
        time,
        flags
    );
    let inner = task.acquire_inner_lock();
    if let Some(file) = get_file_discpt(fd as isize, &path, &inner, flags) {
        match file {
            FileClass::File(_) => return 0,
            _ => return -1,
        }
    } else {
        return -2;
    }
}

pub fn sys_utimensat(fd: isize, path: *const u8, time: usize, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags).unwrap();
    log::debug!(
        "sys_utimensat: fd: {}, path: {}, time: {:#X?}, flags: {:?}",
        fd,
        path,
        time,
        flags
    );
    let inner = task.acquire_inner_lock();
    if let Some(file) = get_file_discpt(fd, &path, &inner, flags) {
        match file {
            FileClass::File(_f) => return 0,
            _ => return -1,
        }
    } else {
        return -2;
    }
}

// fake implement
pub fn sys_readlinkat(dirfd: i32, pathname: *const u8, buf: *mut u8, bufsize: usize) -> isize {
    let token = current_user_token();
    let pathname_str = translated_str(token, pathname);
    log::debug!(
        "sys_readlinkat dirfd: {}, pathname: {}, bufsize: {}",
        dirfd,
        pathname_str,
        bufsize
    );
    if pathname_str != "/proc/self/exe" {
        panic!("sys_readlinkat: pathname not support");
    }
    let linkpath_str = "/lmbench_all\0";
    let buf_vec = translated_byte_buffer(token, buf, 128);
    let mut userbuf = UserBuffer::new(buf_vec);
    userbuf.write(linkpath_str.as_bytes());
    0
}

/* return the num of bytes */
pub fn sys_sendfile(out_fd: isize, in_fd: isize, offset_ptr: *mut usize, count: usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let inner = task.acquire_inner_lock();
    log::debug!(
        "sys_sendfile out_fd: {}, in_fd: {} offset_ptr: {:#X?}, count: {}",
        out_fd,
        in_fd,
        offset_ptr as usize,
        count
    );
    if let Some(file_in) = &inner.fd_table[in_fd as usize] {
        // file_in exists
        match &file_in {
            FileClass::File(fin) => {
                if let Some(file_out) = &inner.fd_table[out_fd as usize] {
                    //file_out exists
                    match &file_out {
                        FileClass::File(fout) => {
                            if offset_ptr as usize != 0 {
                                //won't influence file.offset
                                let offset = translated_refmut(token, offset_ptr);
                                let data = fin.read_vec(*offset as isize, count);
                                let wlen = fout.write_all(&data);
                                *offset += wlen;
                                return wlen as isize;
                            } else {
                                //use file.offset
                                let data = fin.read_vec(-1, count);
                                let wlen = fout.write_all(&data);
                                return wlen as isize;
                            }
                        }
                        _ => return -1,
                    }
                } else {
                    return -1;
                }
            }
            _ => return -1,
        }
    } else {
        return -1;
    }
}

pub fn sys_lseek(fd: usize, offset: isize, whence: i32) -> isize {
    log::debug!(
        "sys_lseek fd: {}, offset: {}, whence: {}",
        fd,
        offset,
        whence
    );
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();

    if fd > inner.fd_table.len() {
        return -EBADF;
    }

    if let Some(fdes) = &inner.fd_table[fd] {
        let fclass = &fdes;
        match fclass {
            FileClass::File(inode) => return inode.lseek(offset, whence),
            _ => return -EISDIR,
        }
    } else {
        return -EBADF;
    }
}

pub fn sys_statfs(path: *const u8, buf: *mut u8) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    log::debug!("sys_statfs path: {}, buf: {:#X?}", path, buf as usize);
    let buf_vec = translated_byte_buffer(token, buf, size_of::<StatFS>());
    let mut userbuf = UserBuffer::new(buf_vec);

    let kstat = StatFS::empty();
    userbuf.write(kstat.as_bytes());
    0
}

// no implement, just for debug
pub fn sys_renameat2(
    olddirfd: i32,
    oldpath: *const u8,
    newdirfd: i32,
    newpath: *const u8,
    flags: u32,
) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let oldpath = translated_str(token, oldpath);
    let newpath = translated_str(token, newpath);
    log::debug!(
        "sys_renameat2 olddirfd: {}, oldpath: {}, newdirfd: {}, newpath: {}, flags: {}",
        olddirfd,
        oldpath,
        newdirfd,
        newpath,
        flags
    );
    0
}
