use crate::model::*;

pub fn print_pools(pools: &[ResourcePool]) {
    for pool in pools {
        println!("Resource Pool: {}", pool.name);
        println!("  Mount point : {}", pool.mount_point);
        println!("  Total space : {} GiB", gib(pool.total_bytes));
        println!("  Free space  : {} GiB", gib(pool.free_bytes));

        for lifecycle in &pool.lifecycles {
            println!("  Lifecycle: {:?}", lifecycle.state);
            for consumer in &lifecycle.consumers {
                println!(
                    "    {:<12} {:>6} GiB",
                    format!("{:?}", consumer.class),
                    gib(consumer.bytes_used)
                );
            }
        }

        println!();
    }
}

fn gib(bytes: u64) -> u64 {
    bytes / 1024 / 1024 / 1024
}
