//! Implementation of  [`DeadlockDetector`]

use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct DeadlockDetector  {
    /// available\[rid\]
    pub available: Vec<usize>,
    /// allocation\[tid\]\[rid\]
    pub allocation: Vec<Vec<usize>>,
    /// need\[tid\]\[rid\]
    pub need: Vec<Vec<usize>>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        DeadlockDetector {
            available: Vec::new(),
            allocation: vec![vec![]],
            need: vec![vec![]],
        }
    }

    pub fn add_need(&mut self, rid: usize, tid: usize) {
        self.need[tid][rid] += 1;
    }

    pub fn remove_need(&mut self, rid: usize, tid: usize) {
        self.need[tid][rid] -= 1;
    }

    pub fn is_safe(&self, rid: usize, tid: usize) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.need.len()];

        if self.need[tid][rid] == 0 || self.available[rid] == 0 {
            return false;
        }

        // 先tid，再rid
        let mut made_progress = true;
        while made_progress {
            made_progress = false;

            for i in 0..finish.len() {
                if !finish[i] && self.need[i].iter().zip(&work).all(|(n, w)| n <= w) {
                    for j in 0..work.len() {
                        work[j] += self.allocation[i][j];
                    }
                    finish[i] = true;
                    made_progress = true;
                }
            }
        }

        finish.iter().all(|&x| x)
    }
    
    pub fn lock(&mut self, rid: usize, tid: usize) {
        self.available[rid] -= 1;
        self.allocation[tid][rid] += 1;
        self.need[tid][rid] -= 1;
    }

    pub fn unlock(&mut self, rid: usize, tid: usize) {
        self.available[rid] += 1;
        self.allocation[tid][rid] -= 1;
    }

    pub fn add_thread(&mut self, tid: usize) {
        if self.allocation.len() <= tid {
            self.allocation.resize(tid + 1, vec![0; self.available.len()]);
        }

        if self.need.len() <= tid {
            self.need.resize(tid + 1, vec![0; self.available.len()]);
        }
    }
}

/// mutex detect
impl DeadlockDetector {
    pub fn add_mutex(&mut self, mutex_id: usize) {
        if self.available.len() <= mutex_id {
            self.available.resize(mutex_id + 1, 1);
        }

        self.allocation.iter_mut().for_each(|item| {
            if item.len() <= mutex_id {
                item.resize(mutex_id + 1, 0);
            }
        });

        self.need.iter_mut().for_each(|item| {
            if item.len() <= mutex_id {
                item.resize(mutex_id + 1, 0);
            }
        });
    }
}

/// semaphore detect
impl DeadlockDetector {
    pub fn add_semaphore(&mut self, semaphore_id: usize, count: usize) {
        if self.available.len() <= semaphore_id {
            self.available.resize(semaphore_id + 1, count);
        }

        self.allocation.iter_mut().for_each(|item| {
            if item.len() <= semaphore_id {
                item.resize(semaphore_id + 1, 0);
            }
        });

        self.need.iter_mut().for_each(|item| {
            if item.len() <= semaphore_id {
                item.resize(semaphore_id + 1, 0);
            }
        });
    }
}
