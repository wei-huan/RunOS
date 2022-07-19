#![allow(unused)]
//use bitflags::*;
use alloc::vec::Vec;
use core::mem::size_of;

pub const DT_UNKNOWN: u8 = 0;
pub const DT_DIR: u8 = 4;
pub const DT_REG: u8 = 4; //常规文件

pub const NAME_LIMIT: usize = 128; // 太大了会有跨页问题

pub const S_IFMT: u32 = 0o170000; //bit mask for the file type bit field
pub const S_IFSOCK: u32 = 0o140000; //socket
pub const S_IFLNK: u32 = 0o120000; //symbolic link
pub const S_IFREG: u32 = 0o100000; //regular file
pub const S_IFBLK: u32 = 0o060000; //block device
pub const S_IFDIR: u32 = 0o040000; //directory
pub const S_IFCHR: u32 = 0o020000; //character device
pub const S_IFIFO: u32 = 0o010000; //FIFO

pub const S_ISUID: u32 = 0o4000; //set-user-ID bit (see execve(2))
pub const S_ISGID: u32 = 0o2000; //set-group-ID bit (see below)
pub const S_ISVTX: u32 = 0o1000; //sticky bit (see below)

pub const S_IRWXU: u32 = 0o0700; //owner has read, write, and execute permission
pub const S_IRUSR: u32 = 0o0400; //owner has read permission
pub const S_IWUSR: u32 = 0o0200; //owner has write permission
pub const S_IXUSR: u32 = 0o0100; //owner has execute permission

pub const S_IRWXG: u32 = 0o0070; //group has read, write, and execute permission
pub const S_IRGRP: u32 = 0o0040; //group has read permission
pub const S_IWGRP: u32 = 0o0020; //group has write permission
pub const S_IXGRP: u32 = 0o0010; //group has execute permission

pub const S_IRWXO: u32 = 0o0007; //others (not in group) have read, write,and execute permission
pub const S_IROTH: u32 = 0o0004; //others have read permission
pub const S_IWOTH: u32 = 0o0002; //others have write permission
pub const S_IXOTH: u32 = 0o0001; //others have execute permission

#[repr(C)]
#[derive(Debug)]
pub struct Dirent {
    pub d_ino: usize,
    pub d_off: isize,  // 到下一个dirent的偏移？？？/ 在目录中的偏移
    pub d_reclen: u16, // 当前dirent长度？？？/ 文件名的长度
    pub d_type: u8,
    pub d_name: [u8; NAME_LIMIT],
}

impl Default for Dirent {
    fn default() -> Dirent {
        Dirent {
            d_ino: 0,
            d_off: 0,
            d_reclen: size_of::<Self>() as u16,
            d_type: 0,
            d_name: [0; NAME_LIMIT],
        }
    }
}

impl Dirent {
    pub fn new(name: &str, inode: usize, offset: isize, _reclen: u16, d_type: u8) -> Self {
        let mut dirent = Self {
            d_ino: inode,
            d_off: offset,
            d_type,
            ..Self::default()
        };
        dirent.fill_name(name);
        dirent
    }

    pub fn fill_info(&mut self, name: &str, inode: usize, offset: isize, _reclen: u16, d_type: u8) {
        *self = Self {
            d_ino: inode,
            d_off: offset,
            d_reclen: self.d_reclen,
            d_type,
            d_name: self.d_name,
        };
        self.fill_name(name);
    }

    fn fill_name(&mut self, name: &str) {
        let len = name.len().min(NAME_LIMIT);
        let name_bytes = name.as_bytes();
        for i in 0..len {
            self.d_name[i] = name_bytes[i];
        }
        self.d_name[len] = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as usize as *mut u8, size) }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Stat {
    st_dev: u64,      /* ID of device containing file */
    pub st_ino: u64,  /* Inode number */
    pub st_mode: u32, /* File type and mode */
    st_nlink: u32,    /* Number of hard links */
    st_uid: u32,
    st_gid: u32,
    //__pad   :u64,
    st_blksize: u32,
    //__pad2       :i32,
    st_blocks: u64,
    pub st_size: i64,
    //st_rdev :u64,   /* Device ID (if special file) */
    st_atime_sec: i64,
    st_atime_nsec: i64,
    st_mtime_sec: i64,
    st_mtime_nsec: i64,
    st_ctime_sec: i64,
    st_ctime_nsec: i64,
}

impl Stat {
    pub fn empty() -> Self {
        Self {
            st_mode: 0100777,
            st_blksize: 512,
            st_nlink: 1,
            ..Self::default()
        }
    }

    pub fn new_abstract() -> Self {
        Self {
            st_dev: 5,
            st_ino: 1,
            st_mode: 0100777,
            st_nlink: 1,
            st_blksize: 512,
            ..Self::default()
        }
    }

    // 目前仅填充用户测试需要的成员
    pub fn fill_info(
        &mut self,
        st_dev: u64,
        st_ino: u64,
        st_mode: u32,
        st_nlink: u32,
        st_size: i64,
        st_atime_sec: i64,
        st_mtime_sec: i64,
        st_ctime_sec: i64,
    ) {
        let st_blocks = (st_size as u64 + self.st_blksize as u64 - 1) / self.st_blksize as u64;
        *self = Self {
            st_dev,
            st_ino,
            st_mode,
            st_nlink,
            st_blksize: 512,
            st_mtime_sec,
            st_ctime_sec,
            st_size,
            st_blocks,
            st_atime_sec,
            ..Self::default()
        };
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as usize as *mut u8, size) }
    }
}

#[repr(C)]
pub struct IOVec {
    pub iov_base: *mut u8,
    pub iov_len: usize,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct StatFS {
    f_bsize: u64,  /* Optimal transfer block size */
    f_frsize: u64, /* Optimal transfer block size */
    f_blocks: u64, /* Total data blocks in filesystem */
    f_bfree: u64,  /* Free blocks in filesystem */
    f_bavail: u64, /* Free blocks available to unprivileged user */
    f_files: u64,  /* Total inodes in filesystem */
    f_ffree: u64,  /* Free inodes in filesystem */
    f_favail: u64,
    f_fsid: u64, /* Filesystem ID */
    f_flag: u64, /* Mount flags of filesystem (since Linux 2.6.36) */
    f_namemax: u64, /* Maximum length of filenames */
}

impl StatFS {
    pub fn empty() -> Self {
        Self {
            f_bsize: 512,
            f_frsize: 512,
            f_blocks: 0x200000,
            f_bfree: 0x100000,
            f_bavail: 0x100000,
            f_files: 128,
            f_ffree: 64,
            f_favail: 64,
            f_fsid: 128,
            f_flag: 128,
            f_namemax: 128,
            ..Self::default()
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }
}
