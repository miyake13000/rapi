mod comm_focused;
mod fixed_timeslice;
mod wait_focused;

use std::sync::{Arc, RwLock};

pub use super::job::Job;

pub use comm_focused::CommFocused;
pub use fixed_timeslice::FixedTimeslice;
pub use wait_focused::WaitFocused;

pub trait Strategy {
    fn job_starts(&mut self);
    fn should_stop_job(&mut self, job: Arc<RwLock<Job>>) -> bool;
    fn should_start_job(&mut self, job: Arc<RwLock<Job>>) -> bool;
}
