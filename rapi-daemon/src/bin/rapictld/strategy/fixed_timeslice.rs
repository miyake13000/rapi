use super::{Job, Strategy};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FixedTimeslice {
    timeslice: Duration,
    last_resumed: Instant,
    last_stopped: Instant,
}

impl FixedTimeslice {
    pub fn new(timeslice: Duration) -> Self {
        Self {
            timeslice,
            last_resumed: Instant::now(),
            last_stopped: Instant::now(),
        }
    }
}

impl Strategy for FixedTimeslice {
    fn job_starts(&mut self) {
        self.last_resumed = Instant::now();
    }

    fn should_stop_job(&mut self, _job: Arc<RwLock<Job>>) -> bool {
        if self.timeslice.is_zero() {
            false
        } else if self.last_resumed.elapsed() >= self.timeslice {
            self.last_stopped = Instant::now();
            true
        } else {
            false
        }
    }

    fn should_start_job(&mut self, _job: Arc<RwLock<Job>>) -> bool {
        if self.timeslice.is_zero() {
            true
        } else if self.last_stopped.elapsed() >= self.timeslice {
            self.last_resumed = Instant::now();
            true
        } else {
            false
        }
    }
}
