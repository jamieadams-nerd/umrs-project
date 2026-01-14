mod model;
mod mock;
mod output;

use clap::Parser;

#[derive(Parser)]
#[command(name = "umrs-logspace")]
#[command(about = "Resource-pool–centric log space analysis", long_about = None)]
struct Cli {
    #[arg(long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let pools = mock::sample_pools();

    if cli.json {
        let json = serde_json::to_string_pretty(&pools)
            .expect("failed to serialize state");
        println!("{}", json);
    } else {
        output::print_pools(&pools);
    }
}
