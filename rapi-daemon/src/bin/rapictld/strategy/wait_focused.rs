use super::super::job::ProcessStatus;
use super::{Job, Strategy};
use log::*;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct WaitFocused {
    timeslice_min: Duration,
    timeslice_max: Duration,
    sleep_time: Duration,
    last_resumed: Instant,
    last_stopped: Instant,
    threshold: f64,
}

impl WaitFocused {
    pub fn new(
        timeslice_min: Duration,
        timeslice_max: Duration,
        sleep_time: Duration,
        threshold: f64,
    ) -> Self {
        Self {
            timeslice_min,
            timeslice_max,
            sleep_time,
            last_resumed: Instant::now(),
            last_stopped: Instant::now(),
            threshold,
        }
    }
}

impl Strategy for WaitFocused {
    fn job_starts(&mut self) {
        self.last_resumed = Instant::now();
        debug!("Job start in: {:?}", self.last_resumed);
    }

    fn should_stop_job(&mut self, job: std::sync::Arc<std::sync::RwLock<Job>>) -> bool {
        let running_time = self.last_resumed.elapsed();
        if running_time >= self.timeslice_max {
            self.last_stopped = Instant::now();
            debug!(
                "Stop job because running_time ({:?}) >= max_ts ({:?})",
                running_time, self.timeslice_max
            );
            return true;
        }
        let rate_waiting = job.read().unwrap().rate_of(ProcessStatus::Waiting);
        if running_time >= self.timeslice_min && rate_waiting >= self.threshold {
            self.last_stopped = Instant::now();
            debug!(
                "Stop job because rate_waiting ({}%) >= threshold ({}%)",
                rate_waiting, self.threshold
            );
            return true;
        }
        debug!(
            "! running_time ({:?}) >= max_ts ({:?}) && ! rate_waiting ({}%) >= threshold ({}%)",
            running_time, self.timeslice_max, rate_waiting, self.threshold
        );
        false
    }

    fn should_start_job(&mut self, _job: Arc<RwLock<Job>>) -> bool {
        let sleeping_time = self.last_stopped.elapsed();
        if sleeping_time >= self.sleep_time {
            self.last_resumed = Instant::now();
            debug!(
                "Start job because sleeping_time ({:?}) >= sleep_time ({:?})",
                sleeping_time, self.sleep_time,
            );
            true
        } else {
            trace!(
                "! sleeping_time ({:?}) >= sleep_time ({:?})",
                sleeping_time,
                self.sleep_time,
            );
            false
        }
    }
}
