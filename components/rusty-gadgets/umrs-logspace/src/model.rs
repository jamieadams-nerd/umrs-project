use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcePool {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub lifecycles: Vec<LifecycleUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleUsage {
    pub state: LifecycleState,
    pub consumers: Vec<LogConsumer>,
    pub total_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConsumer {
    pub class: LogClass,
    pub bytes_used: u64,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum LifecycleState {
    Active,
    Inactive,
    Archived,
    Offline,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogClass {
    Audit,
    System,
    Application,
    Umrs,
}
