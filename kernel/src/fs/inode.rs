use super::{finfo, Dirent, File, Stat, DT_DIR, DT_REG, DT_UNKNOWN};
use crate::mm::UserBuffer;
use crate::owo_colors::OwoColorize;
use crate::syscall::EINVAL;
use crate::{drivers::BLOCK_DEVICE, println};
use _core::usize;
use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
use lazy_static::*;
use runfs::{FileAttributes, RunFileSystem, VFile};
use spin::rwlock::RwLock;
use spin::Mutex;

pub const SEEK_SET: i32 = 0; /* set to offset bytes.  */
pub const SEEK_CUR: i32 = 1; /* set to its current location plus offset bytes.  */
pub const SEEK_END: i32 = 2; /* set to the size of the file plus offset bytes.  */
/*  Adjust the file offset to the next location in the file
greater than or equal to offset containing data.  If
offset points to data, then the file offset is set to
offset */
#[allow(unused)]
pub const SEEK_DATA: i32 = 3;
/*  Adjust the file offset to the next hole in the file
greater than or equal to offset.  If offset points into
the middle of a hole, then the file offset is set to
offset.  If there is no hole past offset, then the file
offset is adjusted to the end of the file (i.e., there is
an implicit hole at the end of any file). */
#[allow(unused)]
pub const SEEK_HOLE: i32 = 4;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DiskInodeType {
    File,
    Directory,
}

// 此inode实际被当作文件
pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: Mutex<OSInodeInner>,
}

pub struct OSInodeInner {
    offset: usize,     // 当前读写的位置
    inode: Arc<VFile>, // inode引用
}

impl OSInode {
    pub fn new(readable: bool, writable: bool, inode: Arc<VFile>) -> Self {
        Self {
            readable,
            writable,
            //fd_cloexec:false,
            inner: Mutex::new(OSInodeInner { offset: 0, inode }),
        }
    }

    pub fn is_dir(&self) -> bool {
        let inner = self.inner.lock();
        inner.inode.is_dir()
    }

    /* this func will not influence the file offset
     * @parm: if offset == -1, file offset will be used
     */
    pub fn read_vec(&self, offset: isize, len: usize) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut len = len;
        let ori_off = inner.offset;
        if offset >= 0 {
            inner.offset = offset as usize;
        }
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let rlen = inner.inode.read_at(inner.offset, &mut buffer);
            if rlen == 0 {
                break;
            }
            inner.offset += rlen;
            v.extend_from_slice(&buffer[..rlen.min(len)]);
            if len > rlen {
                len -= rlen;
            } else {
                break;
            }
        }
        if offset >= 0 {
            inner.offset = ori_off;
        }
        v
    }

    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }

    pub fn write_all(&self, str_vec: &Vec<u8>) -> usize {
        let mut inner = self.inner.lock();
        let mut remain = str_vec.len();
        let mut base = 0;
        loop {
            let len = remain.min(512);
            inner
                .inode
                .write_at(inner.offset, &str_vec.as_slice()[base..base + len]);
            inner.offset += len;
            base += len;
            remain -= len;
            if remain == 0 {
                break;
            }
        }
        return base;
    }

    pub fn find(&self, path: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
        let inner = self.inner.lock();
        let vfile = inner.inode.find_vfile_bypath(path);
        if vfile.is_none() {
            return None;
        } else {
            let (readable, writable) = flags.read_write();
            return Some(Arc::new(OSInode::new(readable, writable, vfile.unwrap())));
        }
    }

    pub fn getdirent(&self, dirent: &mut Dirent) -> isize {
        let mut inner = self.inner.lock();
        let offset = inner.offset;
        if let Some((name, off, first_clu, attr)) = inner.inode.dirent_info(offset as usize) {
            let d_type: u8;
            if attr.contains(FileAttributes::DIRECTORY) {
                d_type = DT_DIR;
            } else if attr.contains(FileAttributes::ARCHIVE) {
                d_type = DT_REG;
            } else {
                d_type = DT_UNKNOWN;
            }
            dirent.fill_info(
                name.as_str(),
                first_clu as usize,
                (off - offset) as isize,
                name.len() as u16,
                d_type,
            );
            inner.offset = off as usize;
            let len = (name.len() + 8 * 4) as isize;
            len
        } else {
            -1
        }
    }

    pub fn get_fstat(&self, kstat: &mut Stat) {
        let inner = self.inner.lock();
        let vfile = inner.inode.clone();
        let (size, atime, mtime, ctime, ino) = vfile.stat();
        let st_mod: u32 = {
            if vfile.is_dir() {
                finfo::S_IFDIR | finfo::S_IRWXU | finfo::S_IRWXG | finfo::S_IRWXO
            } else {
                finfo::S_IFREG | finfo::S_IRWXU | finfo::S_IRWXG | finfo::S_IRWXO
            }
        };
        kstat.fill_info(0, ino, st_mod, 1, size, atime, mtime, ctime);
    }

    pub fn get_size(&self) -> usize {
        let inner = self.inner.lock();
        let (size, _, _, _, _) = inner.inode.stat();
        return size as usize;
    }

    pub fn create(&self, path: &str, type_: DiskInodeType) -> Option<Arc<OSInode>> {
        let inner = self.inner.lock();
        let cur_inode = inner.inode.clone();
        if !cur_inode.is_dir() {
            log::debug!("[create]:{} is not a directory!", path);
            return None;
        }
        let mut pathv: Vec<&str> = path.split('/').collect();
        log::debug!("pathv: {:#?}", pathv);
        let (readable, writable) = (true, true);
        if let Some(inode) = cur_inode.find_vfile_bypath(path) {
            // already exists, clear
            inode.delete();
        }
        {
            // create file
            let name = pathv.pop().unwrap();
            if let Some(temp_inode) = cur_inode.find_vfile_bypath(path) {
                let attribute = {
                    match type_ {
                        DiskInodeType::Directory => FileAttributes::DIRECTORY,
                        DiskInodeType::File => FileAttributes::FILE,
                    }
                };
                temp_inode
                    .create(name, attribute)
                    .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
            } else {
                let attribute = {
                    match type_ {
                        DiskInodeType::Directory => FileAttributes::DIRECTORY,
                        DiskInodeType::File => FileAttributes::FILE,
                    }
                };
                cur_inode
                    .create(name, attribute)
                    .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
            }
        }
    }

    // pub fn clear(&self) {
    //     let inner = self.inner.lock();
    //     inner.inode.clear();
    // }

    pub fn delete(&self) -> usize {
        let inner = self.inner.lock();
        inner.inode.delete()
    }

    pub fn set_head_cluster(&self, cluster: u32) {
        let inner = self.inner.lock();
        let vfile = &inner.inode;
        vfile.set_first_cluster(cluster);
    }

    pub fn get_head_cluster(&self) -> u32 {
        let inner = self.inner.lock();
        let vfile = &inner.inode;
        vfile.first_data_cluster()
    }

    // pub fn set_delete_bit(&self) {
    //     let inner = self.inner.lock();
    //     inner.inode.set_delete_bit();
    // }

    pub fn get_offset(&self) -> usize {
        let inner = self.inner.lock();
        inner.offset
    }

    pub fn set_offset(&self, offset: usize) {
        log::trace!("set_offset offset: {:#X?}", offset);
        let mut inner = self.inner.lock();
        inner.offset = offset;
    }

    pub fn lseek(&self, offset: isize, whence: i32) -> isize {
        log::trace!("lseek offset: {}", offset);
        log::trace!("lseek whence: {}", whence);
        let mut inner = self.inner.lock();
        let size = inner.inode.size();
        let cur_offset: isize = inner.offset as isize;
        match whence {
            SEEK_SET => {
                if offset < 0 {
                    return -EINVAL;
                } else {
                    inner.offset = offset as usize;
                }
            }
            SEEK_CUR => {
                if cur_offset + offset < 0 {
                    return -EINVAL;
                } else {
                    inner.offset += offset as usize;
                }
            }
            SEEK_END => {
                if size as isize + offset < 0 {
                    return -EINVAL;
                } else {
                    inner.offset = (size as isize + offset) as usize;
                }
            }
            _ => return -EINVAL,
        }
        inner.offset as isize
    }
}

lazy_static! {
    // 通过 ROOT_INODE 可以实现对 fat32 的操作
    pub static ref ROOT_INODE: Arc<VFile> = {
        // 此处载入文件系统
        // println!("open fs");
        let runfs = Arc::new(RwLock::new(RunFileSystem::new(BLOCK_DEVICE.clone())));
        // println!("get root_dir");
        let root_dir = Arc::new(runfs.read().root_vfile(&runfs));
        // println!("get root_dir ok");
        root_dir
    };
}

pub fn init_rootfs() {
    let _tmp = open("/", "tmp", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
    let _dev = open("/", "dev", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
    let _var = open("/", "var", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
    let _var_tmp = open("/", "/var/tmp", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
    // let _null = open("/", "dev/null", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
}

pub fn list_apps() {
    println!("/**** APPS ****");
    for app in ROOT_INODE.ls().unwrap() {
        if !app.1.contains(FileAttributes::DIRECTORY) {
            println!("{}", app.0.bright_green());
        }
    }
    println!("**************/")
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 6;
        const TRUNC = 1 << 10;
        const DIRECTROY = 0x0200000;
        const LARGEFILE  = 0100000;
        const CLOEXEC = 02000000;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

pub fn open(
    work_path: &str,
    path: &str,
    flags: OpenFlags,
    type_: DiskInodeType,
) -> Option<Arc<OSInode>> {
    // log::debug!("work_path {}", work_path);
    // log::debug!("path {}", path);
    // DEBUG: 相对路径
    let cur_inode = {
        if work_path == "/" {
            ROOT_INODE.clone()
        } else {
            ROOT_INODE.find_vfile_bypath(work_path).unwrap()
        }
    };
    // shell 应当保证此处输入的 path 不为空
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = cur_inode.find_vfile_bypath(path) {
            inode.delete();
        }
        {
            // create file
            // log::debug!("path: {:?}", path);
            let name_path: Vec<&str> = path.rsplitn(2, '/').collect();
            // log::debug!("name_path: {:?}", name_path);
            let name = name_path[0];
            let mut prev_path = "";
            if name_path.len() == 2 {
                prev_path = name_path[1]
            }
            // log::debug!("prev_path: {:?}, name: {:?}", prev_path, name);
            if let Some(temp_inode) = cur_inode.find_vfile_bypath(prev_path) {
                let attribute = {
                    match type_ {
                        DiskInodeType::Directory => FileAttributes::DIRECTORY,
                        DiskInodeType::File => FileAttributes::FILE,
                    }
                };
                temp_inode
                    .create(name, attribute)
                    .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
            } else {
                None
            }
        }
    } else {
        cur_inode.find_vfile_bypath(path).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                // inode.clear();
            }
            // println!("open finish");
            Arc::new(OSInode::new(readable, writable, inode))
        })
        // let inode = cur_inode.find_vfile_byname("initproc").unwrap();
        // Some(Arc::new(OSInode::new(readable, writable, Arc::new(inode))))
    }
}

/// 切换工作路径, 成功, 返回inode_id, 否则返回-1
pub fn ch_dir(work_path: &str, path: &str) -> isize {
    let cur_inode = {
        if work_path == "/" || (path.len() > 0 && path.chars().nth(0).unwrap() == '/') {
            ROOT_INODE.clone()
        } else {
            //log::debug!("in cd, work_path = {:?}", work_path);
            ROOT_INODE.find_vfile_bypath(work_path).unwrap()
        }
    };
    if let Some(_tar_dir) = cur_inode.find_vfile_bypath(path) {
        // ! 当inode_id > 2^16 时，有溢出的可能（目前不会发生。。
        0
    } else {
        -1
    }
}

pub fn clear_cache() {
    ROOT_INODE.clear_cache();
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.lock();
        let mut total_read_size = 0usize;
        // buffers element is [u8], not u8
        for slice in buf.buffers.iter_mut() {
            // log::debug!("slice before read: {:#?}", slice.len());
            let read_size = inner.inode.read_at(inner.offset, *slice);
            // log::debug!("read_size after read: {:#?}", read_size);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.lock();
        let mut total_write_size = 0usize;
        for slice in buf.buffers.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
    fn available(&self) -> bool {
        true
    }
}
