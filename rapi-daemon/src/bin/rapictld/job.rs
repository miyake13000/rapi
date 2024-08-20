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

    pub fn n_communicating(&self) -> usize {
        self.0
            .values()
            .filter(|&&v| v == ProcessStatus::Communicating)
            .count()
    }

    pub fn n_waiting(&self) -> usize {
        self.0
            .values()
            .filter(|&&v| v == ProcessStatus::Waiting)
            .count()
    }
}
