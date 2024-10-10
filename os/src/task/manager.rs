//!Implementation of [`TaskManager`]
use super::{TaskControlBlock, BIG_STRIDE};
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut min_stride_task: Option<Arc<TaskControlBlock>> = None;

        let mut idx = 0;
        for task in self.ready_queue.iter() {
            if let Some(cur_min_task) = &min_stride_task {
                if task.inner_exclusive_access().priority
                    < cur_min_task.inner_exclusive_access().priority
                {
                    min_stride_task = Some(task.clone());
                }
            } else {
                min_stride_task = Some(task.clone());
            }
            idx += 1;
        }

        let task = min_stride_task.unwrap();
        let mut inner = task.inner_exclusive_access();
        inner.priority += BIG_STRIDE / inner.priority;
        if self.ready_queue.is_empty() {
            return None;
        } else {
            self.ready_queue.remove(idx);
            Some(task.clone())
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
