mod round_robin;

use crate::config::PAGE_SIZE;
use crate::cpu::take_my_cpu;
use crate::fs::{open, DiskInodeType, OpenFlags};
use crate::mm::add_free;
use crate::task::{ProcessControlBlock, TaskContext, TaskControlBlock};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::arch::global_asm;
use lazy_static::*;
use round_robin::RoundRobinScheduler;
use spin::Mutex;

pub trait Scheduler {
    fn schedule(&self);
    fn add_task(&self, task: Arc<TaskControlBlock>);
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>>;
}

lazy_static! {
    pub static ref SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();
    pub static ref PID2PCB: Mutex<BTreeMap<usize, Arc<ProcessControlBlock>>> =
        Mutex::new(BTreeMap::new());
    pub static ref TID2TCB: Mutex<BTreeMap<usize, Arc<TaskControlBlock>>> =
        Mutex::new(BTreeMap::new());
}

pub fn schedule() {
    SCHEDULER.schedule()
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    SCHEDULER.add_task(task);
}

#[allow(unused)]
pub fn have_ready_task() -> bool {
    SCHEDULER.have_ready_task()
}

pub fn pid2process(pid: usize) -> Option<Arc<ProcessControlBlock>> {
    let map = PID2PCB.lock();
    map.get(&pid).map(Arc::clone)
}

pub fn insert_into_pid2process(pid: usize, process: Arc<ProcessControlBlock>) {
    PID2PCB.lock().insert(pid, process);
}

pub fn remove_from_pid2process(pid: usize) {
    let mut map = PID2PCB.lock();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2process!", pid);
    }
}

pub fn tid2task(tid: usize) -> Option<Arc<TaskControlBlock>> {
    let map = TID2TCB.lock();
    map.get(&tid).map(Arc::clone)
}

pub fn insert_into_tid2task(tid: usize, task: Arc<TaskControlBlock>) {
    TID2TCB.lock().insert(tid, task);
}

pub fn remove_from_tid2task(tid: usize) {
    let mut map = TID2TCB.lock();
    if map.remove(&tid).is_none() {
        panic!("cannot find tid {} in tid2task!", tid);
    }
}

global_asm!(include_str!("schedule.S"));
extern "C" {
    pub fn __schedule(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

lazy_static! {
    pub static ref INITPROC: Arc<ProcessControlBlock> = {
        let inode = open("/", "initproc", OpenFlags::RDONLY, DiskInodeType::File).unwrap();
        // println!("open initproc finish");
        let v = inode.read_all();
        // println!("read_all initproc finish");
        ProcessControlBlock::new(v.as_slice())
    };
}

pub fn add_initproc() {
    add_initproc_into_fs();
    let _initproc = INITPROC.clone();
    // println!("add_initproc finish");
}

pub fn save_current_and_back_to_schedule(current_task_cx_ptr: *mut TaskContext) {
    let mut cpu = take_my_cpu();
    let idle_task_cx_ptr = cpu.get_idle_task_cx_ptr();
    drop(cpu);
    unsafe { __schedule(current_task_cx_ptr, idle_task_cx_ptr) };
}

// Write initproc & user_shell into file system to be executed
// And then release them to fram_allocator
#[allow(unused)]
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
    // if let Some(inode) = open("/", "initproc", OpenFlags::RDONLY, DiskInodeType::File) {
    //     println!("Already have initproc in FS");
    //     inode.delete();
    // }
    // if let Some(inode) = open("/", "user_shell", OpenFlags::RDONLY, DiskInodeType::File) {
    //     println!("Already have init user_shell in FS");
    //     inode.delete();
    // }

    //Write apps initproc to disk from mem
    // if let Some(inode) = open("/", "initproc", OpenFlags::CREATE, DiskInodeType::File) {
    //     let mut data: Vec<&'static mut [u8]> = Vec::new();
    //     data.push(unsafe {
    //         core::slice::from_raw_parts_mut(app_start[0] as *mut u8, app_start[1] - app_start[0])
    //     });
    //     inode.write(UserBuffer::new(data));
    // } else {
    //     panic!("initproc create fail!");
    // }
    //Write apps user_shell to disk from mem
    // if let Some(inode) = open("/", "user_shell", OpenFlags::CREATE, DiskInodeType::File) {
    //     let mut data: Vec<&'static mut [u8]> = Vec::new();
    //     data.push(unsafe {
    //         core::slice::from_raw_parts_mut(app_start[1] as *mut u8, app_start[2] - app_start[1])
    //     });
    //     //data.extend_from_slice()
    //     // println!("Start write user_shell ");
    //     inode.write(UserBuffer::new(data));
    //     // println!("User_shell OK");
    // } else {
    //     panic!("user_shell create fail!");
    // }
    // recycle pages
    let mut start_ppn = app_start[0] / PAGE_SIZE + 1;
    while start_ppn < app_start[2] / PAGE_SIZE {
        add_free(start_ppn);
        start_ppn += 1;
    }
}
