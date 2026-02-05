use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pool: Vec<PoolConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PoolConfig {
    pub name: String,
    pub mount_point: String,
    pub lifecycle: Vec<LifecycleConfig>,
}

#[derive(Debug, Deserialize)]
pub struct LifecycleConfig {
    pub state: String,

    #[serde(rename = "path")]
    pub paths: Vec<PathConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PathConfig {
    pub path: String,
    pub class: String,
}
