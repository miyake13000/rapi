use super::{Job, Strategy};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CommFocused {
    timeslice_min: Duration,
    timeslice_max: Duration,
    switching_interval: Duration,
    last_resumed: Instant,
    last_stopped: Instant,
}

impl CommFocused {
    pub fn new(timeslice_min: Duration, timeslice_max: Duration) -> Self {
        Self {
            timeslice_min,
            timeslice_max,
            switching_interval: timeslice_min,
            last_resumed: Instant::now(),
            last_stopped: Instant::now(),
        }
    }
}

impl Strategy for CommFocused {
    fn job_starts(&mut self) {
        self.last_resumed = Instant::now();
    }

    fn should_stop_job(&mut self, job: std::sync::Arc<std::sync::RwLock<Job>>) -> bool {
        let running_time = self.last_resumed.elapsed();
        let n_communicating = job.read().unwrap().n_communicating();
        if running_time >= self.timeslice_max {
            self.last_stopped = Instant::now();
            return true;
        }
        if running_time >= self.timeslice_min && n_communicating > 0 {
            self.last_stopped = Instant::now();
            return true;
        }
        false
    }

    fn should_start_job(&mut self, _job: Arc<RwLock<Job>>) -> bool {
        if self.last_stopped.elapsed() >= self.switching_interval {
            self.last_resumed = Instant::now();
            true
        } else {
            false
        }
    }
}
