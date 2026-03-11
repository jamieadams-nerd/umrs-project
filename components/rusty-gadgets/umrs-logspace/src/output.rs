// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Opertator)
//
use crate::model::*;
use umrs_core::i18n;

pub fn print_pools(pools: &[ResourcePool]) {
    for pool in pools {
        println!("{:<25}: {}", i18n::tr("Resource Pool"), pool.name);
        println!("  {:<23}: {}", i18n::tr("Mount point"), pool.mount_point);
        println!(
            "  {:<23}: {} MiB",
            i18n::tr("Total space"),
            mib(pool.total_bytes)
        );
        println!(
            "  {:<23}: {} MiB",
            i18n::tr("Free space"),
            mib(pool.free_bytes)
        );

        for lifecycle in &pool.lifecycles {
            println!("  {}: {:?}", i18n::tr("Lifecycle"), lifecycle.state);
            for consumer in &lifecycle.consumers {
                println!(
                    "    {:<12} {:>6} MiB ",
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

fn mib(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}
