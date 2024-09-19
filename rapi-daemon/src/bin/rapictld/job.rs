use std::collections::HashMap;

type NodeID = usize;
type Pid = u32;

#[derive(Debug, Clone)]
pub struct Job(HashMap<(NodeID, Pid), ProcessStatus>);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProcessStatus {
    Calculating,
    Communicating,
    Waiting,
}

impl Job {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn change_state(&mut self, node_id: NodeID, pid: Pid, status: ProcessStatus) {
        *self.0.get_mut(&(node_id, pid)).unwrap() = status;
    }

    pub fn append(&mut self, node_id: NodeID, pid: Pid) {
        self.0.insert((node_id, pid), ProcessStatus::Calculating);
    }

    pub fn remove(&mut self, node_id: NodeID, pid: Pid) {
        self.0.remove(&(node_id, pid));
    }

    pub fn is_running(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn n_processes(&self) -> usize {
        self.0.len()
    }

    pub fn num_of(&self, status: ProcessStatus) -> usize {
        self.0.values().filter(|&&v| v == status).count()
    }

    pub fn rate_of(&self, status: ProcessStatus) -> f64 {
        let n_processes = self.n_processes();
        if n_processes == 0 {
            0.0
        } else {
            self.num_of(status) as f64 / n_processes as f64 * 100.0
        }
    }
}
