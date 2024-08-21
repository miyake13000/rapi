use super::{Job, Strategy};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct WaitFocused {
    timeslice_min: Duration,
    timeslice_max: Duration,
    sleep_time: Duration,
    last_resumed: Instant,
    last_stopped: Instant,
}

impl WaitFocused {
    pub fn new(timeslice_min: Duration, timeslice_max: Duration, sleep_time: Duration) -> Self {
        Self {
            timeslice_min,
            timeslice_max,
            sleep_time,
            last_resumed: Instant::now(),
            last_stopped: Instant::now(),
        }
    }
}

impl Strategy for WaitFocused {
    fn job_starts(&mut self) {
        self.last_resumed = Instant::now();
    }

    fn should_stop_job(&mut self, job: std::sync::Arc<std::sync::RwLock<Job>>) -> bool {
        let running_time = self.last_resumed.elapsed();
        let n_communicating = job.read().unwrap().n_waiting();
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
        if self.last_stopped.elapsed() >= self.sleep_time {
            self.last_resumed = Instant::now();
            true
        } else {
            false
        }
    }
}
