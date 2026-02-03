use umrs_core::console::{self, VERBOSE};
use std::sync::atomic::Ordering;
use umrs_core::console::ConsoleEvent;
use umrs_core::prelude::*;

fn main() {
    console::init(); // optional but recommended
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--verbose" | "-v" => {
                VERBOSE.store(true, Ordering::Relaxed);
            }
            _ => {
                // handle other args
            }
        }
    }

    console_info!("This is a test");

    verbose!("Verbose test");

    console_error!("Error message");

    console_status!(true, "Update file");
    console_status!(false, "Update file");

    console_event!(ConsoleEvent::BeginTask {
        name: "Extract Unicode Hieroglyphs",
    });

    println!("Hello, world!");
}
