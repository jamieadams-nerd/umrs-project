// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Opertator)
mod config;
mod config_loader;
mod measure;
mod model;
mod output;

use clap::Parser;
use umrs_core::i18n;

#[derive(Parser)]
#[command(name = "umrs-logspace")]
#[command(about = "Resource-pool–centric log space analysis")]
struct Cli {
    #[arg(long, default_value = "/etc/umrs/logspace.toml")]
    config: String,

    #[arg(long)]
    json: bool,
}

fn main() {
    i18n::init("umrs-logspace");
    env_logger::init();

    let cli = Cli::parse();

    let config =
        config_loader::load_config(&cli.config).expect("configuration error");

    let pools =
        measure::measure_from_config(&config).expect("measurement error");

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&pools).unwrap());
    } else {
        output::print_pools(&pools);
    }
}
