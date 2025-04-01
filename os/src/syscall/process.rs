//! Process management syscalls
use crate::{config::PAGE_SIZE, mm::{PageTable, PhysPageNum, VirtAddr}, task::{change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next}, timer::get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// write a usize data from kernel space to user space with the page table
fn write_usize_to_userspace(page_table: &PageTable, va: VirtAddr, data: usize) {
    let vpn = va.floor();
    let ppn: PhysPageNum = page_table.translate(vpn).unwrap().ppn();
    let va_start = va.0 - vpn.0 * PAGE_SIZE;
    let bytes = ppn.get_bytes_array();
    let data_bytes = data.to_le_bytes();
    bytes[va_start..va_start + data_bytes.len()].copy_from_slice(&data_bytes);
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let sec = us / 1_000_000;
    let usec = us % 1_000_000;

    let page_table = PageTable::from_token(current_user_token());
    let va = VirtAddr::from(_ts as usize);

    write_usize_to_userspace(&page_table, va, sec);
    write_usize_to_userspace(&page_table, VirtAddr::from(va.0 + 8), usec);
    
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
