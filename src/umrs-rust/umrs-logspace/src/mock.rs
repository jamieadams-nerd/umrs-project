use crate::model::*;

pub fn sample_pools() -> Vec<ResourcePool> {
    vec![
        ResourcePool {
            name: "var-log".to_string(),
            mount_point: "/var/log".to_string(),
            total_bytes: 50 * 1024 * 1024 * 1024,
            free_bytes: 12 * 1024 * 1024 * 1024,
            lifecycles: vec![
                LifecycleUsage {
                    state: LifecycleState::Active,
                    consumers: vec![
                        LogConsumer {
                            class: LogClass::Audit,
                            bytes_used: 8 * 1024 * 1024 * 1024,
                        },
                        LogConsumer {
                            class: LogClass::System,
                            bytes_used: 4 * 1024 * 1024 * 1024,
                        },
                    ],
                },
                LifecycleUsage {
                    state: LifecycleState::Archived,
                    consumers: vec![
                        LogConsumer {
                            class: LogClass::Audit,
                            bytes_used: 20 * 1024 * 1024 * 1024,
                        },
                    ],
                },
            ],
        },
    ]
}
