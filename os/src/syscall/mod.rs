//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

mod fs;
mod process;

use alloc::collections::btree_map::BTreeMap;
use fs::*;
use lazy_static::lazy_static;
use process::*;

use crate::{config::MAX_APP_NUM, sync::UPSafeCell, task::get_current_task_id};

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    add_current_syscall_count(syscall_id);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

/// Add 1 to the number of times the current task has called the syscall with `syscall_id`.
fn add_current_syscall_count(syscall_id: usize) {
    let current_task_id = get_current_task_id();
    let mut counts = SYSCALL_COUNT.counts.exclusive_access();
    let valid_id = match syscall_id {
        SYSCALL_WRITE => SYSCALL_WRITE,
        SYSCALL_EXIT => SYSCALL_EXIT,
        SYSCALL_YIELD => SYSCALL_YIELD,
        SYSCALL_GET_TIME => SYSCALL_GET_TIME,
        SYSCALL_TRACE => SYSCALL_TRACE,
        _ => return,
    };

    *counts[current_task_id].entry(valid_id).or_insert(0) += 1;
}

/// Get the number of times the current task has called the syscall with `syscall_id`.
pub fn get_current_syscall_count(syscall_id: usize) -> usize {
    let current_task_id = get_current_task_id();
    let counts = SYSCALL_COUNT.counts.exclusive_access();
    *counts[current_task_id].get(&syscall_id).unwrap_or(&0)
}

/// A structure to keep track of syscall counts.
pub struct SyscallCount {
    counts: UPSafeCell<[BTreeMap<usize, usize>; MAX_APP_NUM]>,
}

lazy_static! {
    /// Global variable: SYSCALL_COUNT
    pub static ref SYSCALL_COUNT: SyscallCount = {
        const ARRAY_REPEAT_VALUE: BTreeMap<usize, usize> = BTreeMap::new();
        let arr: [BTreeMap<usize, usize>; MAX_APP_NUM] = [ARRAY_REPEAT_VALUE; MAX_APP_NUM];
        SyscallCount {
            counts: unsafe {
                UPSafeCell::new(arr)
            },
        }
    };
}
