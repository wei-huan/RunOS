use crate::cpu::{current_task, current_user_token};
use crate::fs::{
    ch_dir, make_pipe, open, Dirent, DiskInodeType, File, FileClass, FileDescripter, IOVec, Kstat,
    OSInode, OpenFlags, MNT_TABLE, S_IFDIR, S_IFREG, S_IRWXG, S_IRWXO, S_IRWXU,
};
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};
use crate::task::TaskControlBlockInner;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::mem::size_of;
use spin::MutexGuard;

const AT_FDCWD: isize = -100;
pub const FD_LIMIT: usize = 128;

pub fn sys_ioctl() -> isize {
    0
}

pub fn sys_fcntl() -> isize {
    0
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file: Arc<dyn File + Send + Sync> = match &file.fclass {
            FileClass::Abstr(f) => f.clone(),
            FileClass::File(f) => {
                /*print!("\n");*/
                f.clone()
            }
            _ => return -1,
        };
        if !file.writable() {
            return -1;
        }
        drop(inner);
        let size = file.write(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        if fd == 2 {
            // str::replace(translated_str(token, buf).as_str(), "\n", "\\n");
        }
        size as isize
    } else {
        -1
    }
}

pub fn sys_writev(fd: usize, iov: *const IOVec, iocnt: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();

    let mut ret = 0isize;

    if fd >= inner.fd_table.len() {
        return -1;
    }

    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match &file.fclass {
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

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    // log::debug!("sys_read");
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file: Arc<dyn File + Send + Sync> = match &file.fclass {
            FileClass::Abstr(f) => f.clone(),
            FileClass::File(f) => f.clone(),
            _ => return -1,
        };
        if !file.readable() {
            return -1;
        }
        // release current task PCB inner manually to avoid multi-borrow
        drop(inner);
        // release current task PCB manually to avoid Arc::strong_count grow
        // drop(task);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_readv(fd: usize, iov: *const IOVec, iocnt: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();

    if fd >= inner.fd_table.len() {
        return -1;
    }

    let mut ret = 0isize;
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match &file.fclass {
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

// TODO:文件所有权
pub fn sys_open_at(dirfd: isize, path: *const u8, flags: u32, _mode: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut inner = task.acquire_inner_lock();

    let oflags = OpenFlags::from_bits(flags).unwrap();
    if dirfd == AT_FDCWD {
        if let Some(inode) = open(
            inner.get_work_path().as_str(),
            path.as_str(),
            oflags,
            DiskInodeType::File,
        ) {
            // log::debug!("open path: {:#?}", path);
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileDescripter::new(
                oflags.contains(OpenFlags::CLOEXEC),
                FileClass::File(inode),
            ));
            fd as isize
        } else {
            -1
        }
    } else {
        let fd_usz = dirfd as usize;
        if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
            return -1;
        }
        if let Some(file) = &inner.fd_table[fd_usz] {
            log::debug!("dirfd: {:#?}, path: {:#?}", dirfd, path);
            match &file.fclass {
                FileClass::File(f) => {
                    // log::debug!("dirfd: {:#?}, path: {:#?}", dirfd, path);
                    // 需要新建文件
                    if oflags.contains(OpenFlags::CREATE) {
                        if let Some(new_file) = f.create(path.as_str(), DiskInodeType::File) {
                            let fd = inner.alloc_fd();
                            inner.fd_table[fd] = Some(FileDescripter::new(
                                oflags.contains(OpenFlags::CLOEXEC),
                                FileClass::File(new_file),
                            ));
                            return fd as isize;
                        } else {
                            return -1;
                        }
                    }
                    // 正常打开文件
                    if let Some(tar_f) = f.find(path.as_str(), oflags) {
                        let fd = inner.alloc_fd();
                        inner.fd_table[fd] = Some(FileDescripter::new(
                            oflags.contains(OpenFlags::CLOEXEC),
                            FileClass::File(tar_f),
                        ));
                        fd as isize
                    } else {
                        return -1;
                    }
                }
                _ => return -1, // 如果是抽象类文件，不能open
            }
        } else {
            return -1;
        }
    }
}

pub fn sys_close(fd: usize) -> isize {
    log::debug!("sys_close");
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_getcwd(buf: *mut u8, len: usize) -> isize {
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
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if old_fd >= inner.fd_table.len() || new_fd > FD_LIMIT {
        return -1;
    }
    if inner.fd_table[old_fd].is_none() {
        return -1;
    }
    // 太傻比了，为了一个 fd 添加这么多，以后要改
    if new_fd >= inner.fd_table.len() {
        for _ in inner.fd_table.len()..(new_fd + 1) {
            inner.fd_table.push(None);
        }
    }
    inner.fd_table[new_fd] = Some(inner.fd_table[old_fd].as_ref().unwrap().clone());
    new_fd as isize
}

pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
    log::trace!("sys_fstat fd: {}", fd);
    let token = current_user_token();
    let task = current_task().unwrap();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let inner = task.acquire_inner_lock();
    // 使用UserBuffer结构，以便于跨页读写
    let mut userbuf = UserBuffer::new(buf_vec);
    let mut kstat = Kstat::empty();
    if fd == AT_FDCWD {
        let work_path = inner.current_path.clone();
        if let Some(file) = open(
            "/",
            work_path.as_str(),
            OpenFlags::RDONLY,
            DiskInodeType::Directory,
        ) {
            file.get_fstat(&mut kstat);
            userbuf.write(kstat.as_bytes());
            return 0;
        } else {
            return -1;
        }
    } else {
        let fd_usz = fd as usize;
        if fd_usz >= inner.fd_table.len() && fd_usz > FD_LIMIT {
            return -1;
        }
        if let Some(file) = &inner.fd_table[fd_usz] {
            match &file.fclass {
                FileClass::File(f) => {
                    f.get_fstat(&mut kstat);
                    userbuf.write(kstat.as_bytes());
                    return 0;
                }
                _ => {
                    userbuf.write(Kstat::new_abstract().as_bytes());
                    return 0; //warning
                }
            }
        } else {
            return -1;
        }
    }
}

fn fstat_inner(f: Arc<OSInode>, userbuf: &mut UserBuffer) -> isize {
    let mut kstat = Kstat::empty();
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

pub fn sys_fstatat(dirfd: isize, path: *mut u8, buf: *mut u8) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);

    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let mut userbuf = UserBuffer::new(buf_vec);

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        task.acquire_inner_lock().current_path.clone()
    } else {
        String::from("/")
    };

    let ret = if let Some(osfile) = open(&cwd, &path, OpenFlags::RDONLY, DiskInodeType::Directory) {
        fstat_inner(osfile, &mut userbuf)
    } else {
        -1
    };
    ret
}

pub fn sys_pipe(pipe: *mut u32, flags: usize) -> isize {
    if flags != 0 {
        println!("[sys_pipe]: flags not support");
    }
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(FileDescripter::new(false, FileClass::Abstr(pipe_read)));
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(FileDescripter::new(false, FileClass::Abstr(pipe_write)));
    *translated_refmut(token, pipe) = read_fd as u32;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd as u32;
    0
}

pub fn sys_mkdir(dirfd: isize, path: *const u8, _mode: u32) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    let path = translated_str(token, path);
    if dirfd == AT_FDCWD {
        let work_path = inner.current_path.clone();
        if let Some(_) = open(
            inner.get_work_path().as_str(),
            path.as_str(),
            OpenFlags::CREATE,
            DiskInodeType::Directory,
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
            match &file.fclass {
                FileClass::File(f) => {
                    if let Some(_) = f.create(path.as_str(), DiskInodeType::Directory) {
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
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let path = translated_str(token, path);
    let mut work_path = inner.current_path.clone();
    let new_ino_id = ch_dir(work_path.as_str(), path.as_str()) as isize;
    //println!("new inode id = {}", new_ino_id);
    if new_ino_id >= 0 {
        //inner.current_inode = new_ino_id as u32;
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
    //println!("=====================================");
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
            OpenFlags::RDONLY,
            DiskInodeType::Directory,
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
            match &file.fclass {
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
    let type_ = {
        if oflags.contains(OpenFlags::DIRECTROY) {
            DiskInodeType::Directory
        } else {
            DiskInodeType::File
        }
    };
    if fd == AT_FDCWD {
        if let Some(inode) = open(inner.get_work_path().as_str(), path.as_str(), oflags, type_) {
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
            match &file.fclass {
                FileClass::File(f) => {
                    if oflags.contains(OpenFlags::CREATE) {
                        if let Some(tar_f) = f.create(path.as_str(), type_) {
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
    let task = current_task().unwrap();
    let token = current_user_token();
    // 这里传入的地址为用户的虚地址，因此要使用用户的虚地址进行映射
    let path = translated_str(token, path);
    let inner = task.acquire_inner_lock();

    if let Some(file) = get_file_discpt(
        fd as isize,
        &path,
        &inner,
        OpenFlags::from_bits(flags).unwrap(),
    ) {
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

pub fn sys_mount(
    p_special: *const u8,
    p_dir: *const u8,
    p_fstype: *const u8,
    flags: usize,
    data: *const u8,
) -> isize {
    // TODO
    let token = current_user_token();
    let special = translated_str(token, p_special);
    let dir = translated_str(token, p_dir);
    let fstype = translated_str(token, p_fstype);
    MNT_TABLE.lock().mount(special, dir, fstype, flags as u32)
}

pub fn sys_umount(p_special: *const u8, flags: usize) -> isize {
    // TODO
    let token = current_user_token();
    let special = translated_str(token, p_special);
    MNT_TABLE.lock().umount(special, flags as u32)
}

pub fn sys_faccessat(fd: usize, path: *const u8, time: usize, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    // 这里传入的地址为用户的虚地址，因此要使用用户的虚地址进行映射
    let path = translated_str(token, path);
    //print!("\n");
    //println!("unlink: path = {}", path);
    let inner = task.acquire_inner_lock();
    //println!("openat: fd = {}", dirfd);
    if let Some(file) = get_file_discpt(
        fd as isize,
        &path,
        &inner,
        OpenFlags::from_bits(flags).unwrap(),
    ) {
        match file {
            FileClass::File(f) => return 0,
            _ => return -1,
        }
    } else {
        return -2;
    }
}

#[repr(C)]
struct Timespec {
    tv_sec: usize,  /* seconds */
    tv_nsec: usize, /* nanoseconds */
}

pub fn sys_utimensat(fd: usize, path: *const u8, time: usize, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut inner = task.acquire_inner_lock();
    if let Some(file) = get_file_discpt(
        fd as isize,
        &path,
        &inner,
        OpenFlags::from_bits(flags).unwrap(),
    ) {
        match file {
            FileClass::File(f) => return 0,
            _ => return -1,
        }
    } else {
        return -2;
    }
}

/* return the num of bytes */
pub fn sys_sendfile(out_fd: isize, in_fd: isize, offset_ptr: *mut usize, count: usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let inner = task.acquire_inner_lock();

    if let Some(file_in) = &inner.fd_table[in_fd as usize] {
        // file_in exists
        match &file_in.fclass {
            FileClass::File(fin) => {
                if let Some(file_out) = &inner.fd_table[out_fd as usize] {
                    //file_out exists
                    match &file_out.fclass {
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
    let task = current_task().unwrap();
    let token = current_user_token();
    let inner = task.acquire_inner_lock();

    if fd > inner.fd_table.len() {
        return -1;
    }

    if let Some(fdes) = &inner.fd_table[fd] {
        let fclass = &fdes.fclass;
        match fclass {
            FileClass::File(inode) => return inode.lseek(offset as isize, whence),
            _ => return -1,
        }
    } else {
        return -1;
    }
}
