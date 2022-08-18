use super::{finfo, Dirent, File, Stat, DT_DIR, DT_REG, DT_UNKNOWN};
use crate::config::{MMAP_BASE, PAGE_SIZE};
use crate::mm::{
    frame_alloc, Frame, MapPermission, MapType, Section, UserBuffer, VirtAddr, KERNEL_SPACE,
};
use crate::owo_colors::OwoColorize;
use crate::syscall::EINVAL;
use crate::{drivers::BLOCK_DEVICE, println};
use alloc::collections::VecDeque;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
use core::arch::asm;
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

    // // only for exec file to save memory compare to Vec[u8]
    // pub fn mmap_to_kernel(&self) -> &'static [u8] {
    //     let size = self.get_size();
    //     let start_va = VirtAddr::from(MMAP_BASE);
    //     let end_va = VirtAddr::from(MMAP_BASE + size);
    //     let mut section = Section::new(
    //         ".mmap_elf".to_string(),
    //         start_va,
    //         end_va,
    //         MapType::Framed,
    //         MapPermission::R | MapPermission::W,
    //     );
    //     let ks_page_table = &mut KERNEL_SPACE.lock().page_table;
    //     section.map(ks_page_table);
    //     let mut buffer = [0u8; 512];
    //     let mut inner = self.inner.lock();
    //     loop {
    //         // println!("here0_1");
    //         let len = inner.inode.read_at(inner.offset, &mut buffer);
    //         // println!("here0_2 len: {}", len);
    //         if len == 0 {
    //             break;
    //         }
    //         section.copy_data(ks_page_table, &buffer[..len], inner.offset);
    //         inner.offset += len;
    //     }
    //     let mut ks_lock = KERNEL_SPACE.lock();
    //     ks_lock.mmap_sections.push(section);
    //     unsafe { core::slice::from_raw_parts_mut(MMAP_BASE as *mut u8, self.get_size()) }
    // }

    // only for exec file to save memory compare to Vec[u8]
    pub fn mmap_to_kernel(&self) {
        let size = self.get_size();
        let start_va = VirtAddr::from(MMAP_BASE);
        let end_va = VirtAddr::from(MMAP_BASE + size);
        let mut section = Section::new(
            ".mmap_elf".to_string(),
            start_va,
            end_va,
            MapType::Framed,
            MapPermission::R | MapPermission::W,
        );
        // get enough frames
        let mut frames: VecDeque<Arc<Frame>> = VecDeque::new();
        for _ in section.vpn_range {
            frames.push_back(Arc::new(frame_alloc().unwrap()));
        }
        // println!("mmap allocate frames: {}", frames.len());
        // copy data to frames
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; PAGE_SIZE];
        loop {
            // println!("here0_1");
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            // println!("here0_2 len: {}", len);
            let dst = &mut frames[inner.offset / PAGE_SIZE].ppn.get_bytes_array()[..len];
            dst.copy_from_slice(&buffer[..len]);
            if len < buffer.len() {
                break;
            }
            inner.offset += len;
        }
        // map secton with existed frames;
        let mut ks_lock = KERNEL_SPACE.lock();
        section.map_with_frames(&mut ks_lock.page_table, frames);
        ks_lock.mmap_sections.push(section);
        // println!("end mmap to kernel");
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

    // TODO: create with long file name entry
    pub fn create(&self, path: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
        let inner = self.inner.lock();
        let cur_inode = inner.inode.clone();
        if !cur_inode.is_dir() {
            log::warn!("[create]:{} is not a directory!", path);
            return None;
        }
        let mut pathv: Vec<&str> = path.split('/').collect();
        // log::debug!("pathv: {:#?}", pathv);
        let (readable, writable) = flags.read_write();
        // if already exists, delete
        if let Some(inode) = cur_inode.find_vfile_bypath(path) {
            inode.delete();
        }
        // create file
        let filename = pathv.pop().unwrap();
        let mut attribute = if flags.contains(OpenFlags::DIRECTROY) {
            FileAttributes::DIRECTORY
        } else {
            FileAttributes::FILE
        };
        if !writable {
            attribute |= FileAttributes::READ_ONLY;
        }
        let create_path = path.trim_end_matches(filename);
        if let Some(temp_inode) = cur_inode.find_vfile_bypath(create_path) {
            temp_inode
                .create(filename, attribute)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            cur_inode
                .create(filename, attribute)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    }

    // only clear data
    // pub fn clear(&self) {
    //     let inner = self.inner.lock();
    //     inner.inode.clear();
    // }

    // delete data and entry
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
    let _proc = open("/", "proc", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    let _mounts = open("/", "proc/mounts", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    let _meminfo = open(
        "/",
        "proc/meminfo",
        OpenFlags::CREATE | OpenFlags::DIRECTROY,
    )
    .unwrap();
    let _ls = open("/", "ls", OpenFlags::CREATE).unwrap();
    // let _busybox_cmd_bak = open("/", "busybox_cmd.bak", OpenFlags::CREATE).unwrap();
    let _tmp = open("/", "tmp", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    let _dev = open("/", "dev", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    // let _var = open("/", "var", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    // let _var_tmp = open("/", "var/tmp", OpenFlags::CREATE | OpenFlags::DIRECTROY).unwrap();
    let _var_tmp_lmbench = open("/", "var/tmp/lmbench", OpenFlags::CREATE).unwrap();
    // let _null = open("/", "dev/null", OpenFlags::CREATE, DiskInodeType::Directory).unwrap();
}

pub fn list_rootfs() {
    println!("************** RootFS START **************");
    for app in ROOT_INODE.ls().unwrap() {
        if !app.1.contains(FileAttributes::DIRECTORY) {
            println!("{}", app.0.bright_green());
        } else {
            println!("{}", app.0.bright_blue());
        }
    }
    println!("************** RootFS END **************");
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY    =   0o0;
        const WRONLY    =   0o1;
        const RDWR      =   0o2;
        const CREATE    =   0o100;
        const EXCL      =   0o200;
        const NOCTTY    =   0o400;
        const TRUNC     =   0o1000;
        const APPEND    =   0o2000;
        const NONBLOCK  =   0o4000;
        const DSYNC     =   0o10000;
        const ASYNC     =   0o20000;
        const DIRECT    =   0o40000;
        const LARGEFILE =   0o100000;
        const DIRECTROY =   0o200000;
        const NOFOLLOW  =   0o400000;
        const CLOEXEC   =   0o2000000;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.contains(Self::RDONLY) {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

// assume that path not null
pub fn open(work_path: &str, path: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let (readable, writable) = flags.read_write();
    if path == "/" {
        return Some(Arc::new(OSInode::new(
            readable,
            writable,
            ROOT_INODE.clone(),
        )));
    }
    let cur_inode = {
        if work_path == "/" {
            ROOT_INODE.clone()
        } else {
            ROOT_INODE.find_vfile_bypath(work_path).unwrap()
        }
    };
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = cur_inode.find_vfile_bypath(path) {
            inode.delete();
        }
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
            let attribute = if flags.contains(OpenFlags::DIRECTROY) {
                FileAttributes::DIRECTORY
            } else {
                FileAttributes::FILE
            };
            let vfile = temp_inode
                .create(name, attribute)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
                .unwrap();
            if flags.contains(OpenFlags::APPEND) {
                vfile.set_offset(vfile.get_size());
            }
            return Some(vfile);
        } else {
            return None;
        }
    } else {
        cur_inode.find_vfile_bypath(path).map(|inode| {
            // if flags.contains(OpenFlags::TRUNC) {
            //     // inode.clear();
            // }
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
    fn read_available(&self) -> bool {
        self.readable
    }
    fn write_available(&self) -> bool {
        self.writable
    }
}
