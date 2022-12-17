//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{munmap, mmap, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, get_ti};
use crate::timer::get_time_us;
use crate::mm::{VirtAddr, PhysAddr, PageTable};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let va = VirtAddr::from(_ts as usize);
    let page_table = PageTable::from_token(current_user_token());
    let ppn = page_table.translate(va.floor()).unwrap().ppn();
    let pa: PhysAddr = ppn.into();
    let _ts = (pa.0 + va.page_offset()) as *mut TimeVal;
    unsafe {
        *_ts = TimeVal {
            sec: _us / 1_000_000, 
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    mmap(_start, _len, _port)
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    munmap(_start, _len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let va = VirtAddr::from(ti as usize);
    let page_table = PageTable::from_token(current_user_token());
    let ppn = page_table.translate(va.floor()).unwrap().ppn();
    let pa: PhysAddr = ppn.into();
    let ti = (pa.0 + va.page_offset()) as *mut TaskInfo;
    unsafe {
        *ti = get_ti();
    }
    0
}
