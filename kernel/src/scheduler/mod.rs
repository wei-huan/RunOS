mod round_robin;

use crate::config::PAGE_SIZE;
use crate::fs::{open, DiskInodeType, File, OpenFlags};
use crate::mm::{add_free, UserBuffer};
use crate::task::{TaskContext, TaskControlBlock};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::global_asm;
use lazy_static::*;
use round_robin::RoundRobinScheduler;

pub trait Scheduler {
    fn schedule(&self) -> !;
    fn add_task(&self, task: Arc<TaskControlBlock>);
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>>;
}

lazy_static! {
    pub static ref SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();
}

pub fn schedule() -> ! {
    SCHEDULER.schedule()
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    SCHEDULER.add_task(task);
}

global_asm!(include_str!("schedule.S"));
extern "C" {
    pub fn __schedule(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
    pub fn __schedule_new(next_task_cx_ptr: *const TaskContext) -> !;
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open("/", "initproc", OpenFlags::RDONLY, DiskInodeType::File).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

// Write initproc & user_shell into file system to be executed
// And then release them to fram_allocator
pub fn add_initproc_into_fs() {
    extern "C" {
        fn _num_app();
    }
    extern "C" {
        fn _app_names();
    }
    let num_app_ptr = _num_app as usize as *mut usize;
    // let start = _app_names as usize as *const u8;
    let app_start = unsafe { core::slice::from_raw_parts_mut(num_app_ptr.add(1), 3) };

    // find if there already exits
    if let Some(inode) = open("/", "initproc", OpenFlags::RDONLY, DiskInodeType::File) {
        // println!("Already have initproc in FS");
        inode.delete();
    }
    if let Some(inode) = open("/", "user_shell", OpenFlags::RDONLY, DiskInodeType::File) {
        // println!("Already have init user_shell in FS");
        inode.delete();
    }
    //Write apps initproc to disk from mem
    if let Some(inode) = open("/", "initproc", OpenFlags::CREATE, DiskInodeType::File) {
        // println!("Create initproc ");
        let mut data: Vec<&'static mut [u8]> = Vec::new();
        data.push(unsafe {
            core::slice::from_raw_parts_mut(app_start[0] as *mut u8, app_start[1] - app_start[0])
        });
        // println!("Start write initproc ");
        inode.write(UserBuffer::new(data));
        // println!("Init_proc OK");
    } else {
        panic!("initproc create fail!");
    }
    //Write apps user_shell to disk from mem
    if let Some(inode) = open("/", "user_shell", OpenFlags::CREATE, DiskInodeType::File) {
        // println!("Create user_shell ");
        let mut data: Vec<&'static mut [u8]> = Vec::new();
        data.push(unsafe {
            core::slice::from_raw_parts_mut(app_start[1] as *mut u8, app_start[2] - app_start[1])
        });
        //data.extend_from_slice(  )
        // println!("Start write user_shell ");
        inode.write(UserBuffer::new(data));
        // println!("User_shell OK");
    } else {
        panic!("user_shell create fail!");
    }
    // recycle pages
    let mut start_ppn = app_start[0] / PAGE_SIZE + 1;
    while start_ppn < app_start[2] / PAGE_SIZE {
        add_free(start_ppn);
        start_ppn += 1;
    }
}

pub fn add_initproc() {
    add_initproc_into_fs();
    add_task(INITPROC.clone());
}

#[inline(always)]
pub fn go_to_schedule() -> ! {
    let schedule_task = TaskContext::goto_schedule();
    unsafe { __schedule_new(&schedule_task as *const TaskContext) };
}

#[inline(always)]
pub fn save_current_and_goto_schedule(current_task_cx_ptr: *mut TaskContext) {
    let schedule_task = TaskContext::goto_schedule();
    unsafe { __schedule(current_task_cx_ptr, &schedule_task as *const TaskContext) };
}
