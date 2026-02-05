//use std::io::{self, Write};
//use std::thread;
//use std::time::Duration;
use std::sync::atomic::Ordering;

use umrs_core::console::*;

fn main() {
    macros::init(); // optional but recommended
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

    console_event!(ConsoleEvent::BeginTask {
        name: "Test the console_*() macros",
    });
    console_info!("TEST OF THE console_info()!");
    console_warn!("TEST OF THE console_warn()!");
    console_info!("Second test of the console_info()!");
    console_status!(true, "Found file");
    console_status!(false, "Couldn't find directory");
    console_status!(false, "Unable to load configuration.");
    console_error!("Some Error message using console_error!()");
    console_info!("Third test of the console_info()!");
    console_event!(ConsoleEvent::EndTask {
        name: "Finished testing console_*() macros",
    });

    println!("\n");
    verbose!("Console's verbose!() macro test");

    console_status!(true, "Update file");
    console_status!(false, "Update file");

    // ==============================================================
    // Check the low-level console::ansi module
    // ==============================================================
    //
    // -----------------------------------------------------------------
    // Basic color + attribute usage
    println!(
        "{}{}Basic ANSI color + bold demo{}",
        AnsiColor::Cyan.start(),
        BOLD,
        RESET
    );

    println!(
        "{}Red{} {}Green{} {}Blue{}",
        AnsiColor::Red.start(),
        RESET,
        AnsiColor::Green.start(),
        RESET,
        AnsiColor::Blue.start(),
        RESET
    );

    // -----------------------------------------------------------------
    // Scoped coloring pattern
    // -----------------------------------------------------------------
    console_event!(ConsoleEvent::BeginTask {
        name: "ANSI Module / Scoped coloring...",
    });
    verbose!(" ");
    //println!("{}INFO{}  Everything is fine", AnsiColor::Green.start(), RESET);
   // println!("{}WARN{}  Something looks odd", AnsiColor::Yellow.start(), RESET);

    // Manually assembled message from Ansi but prefer that we use console_error!()
    //println!(
        //"{}ERROR{} Something failed",
        //AnsiColor::BrightRed.start(),
        //RESET
    //);
    console_event!(ConsoleEvent::FileOpen { path: "/etc/passwd" });
    console_event!(ConsoleEvent::FileClose { path: "/etc/passwd" });
    console_event!(ConsoleEvent::DataRead { path: "/etc/passwd" });
    console_event!(ConsoleEvent::DataWrote { path: "/etc/passwd" });
    verbose!(" ");
    console_event!(ConsoleEvent::EndTask {
        name: "ANSI Module Tests."
    });

    // -----------------------------------------------------------------
    // Cursor visibility & shape
    // -----------------------------------------------------------------
    //println!();
    //println!("Hiding cursor for 2 seconds...");
    //print!("{CURSOR_HIDE}{CURSOR_STEADY_BAR}");
    //io::stdout().flush().unwrap();
    //thread::sleep(Duration::from_secs(2));
    //print!("{CURSOR_DEFAULT}{CURSOR_SHOW}");
    //io::stdout().flush().unwrap();
    //println!("Cursor restored.");

    // -----------------------------------------------------------------
    // Cursor movement and in-place updates (spinner-style)
    // -----------------------------------------------------------------
    //println!();
    //println!("Spinner-style in-place update:");

    //print!("{CURSOR_HIDE}");
    //for i in 0..5 {
        //print!(
            //"{}{}Working… step {}{}",
            //cursor_to(1, 1),
            //AnsiColor::BrightCyan.start(),
            //i,
            //RESET
        //);
        //io::stdout().flush().unwrap();
        //thread::sleep(Duration::from_millis(400));
    //}
    //print!("{CURSOR_SHOW}");
    //io::stdout().flush().unwrap();
    //println!();

    // -----------------------------------------------------------------
    // Line clearing
    // -----------------------------------------------------------------
    //println!();
    //println!("This line will be cleared...");
    //thread::sleep(Duration::from_secs(1));
//
    //print!(
        //"{}{}Line cleared and replaced{}",
        //CLEAR_LINE,
        //AnsiColor::BrightGreen.start(),
        //RESET
    //);
    //println!();

    // -----------------------------------------------------------------
    // Cursor save / restore (overlay)
    // -----------------------------------------------------------------
    //println!();
    //println!("Demonstrating cursor save/restore:");
    //print!("{CURSOR_SAVE}");
    //println!("Main content line");

    //print!(
        //"{}{}[STATUS: OK]{}",
        //cursor_to(1, 40),
        //AnsiColor::BrightWhite.start(),
        //RESET
    //);

    //print!("{CURSOR_RESTORE}");
    //println!();

    // -----------------------------------------------------------------
    // Truecolor (24-bit RGB)
    // -----------------------------------------------------------------
    //println!();
    //println!("Truecolor gradient demo:");
//
    //for i in 0..=255 {
        //print!("{}", fg_rgb(i, 255 - i, 128));
        //print!("█");
    //}
    //println!("{RESET}");

    // -----------------------------------------------------------------
    // Banner / header composition
    // -----------------------------------------------------------------
    println!();
    println!(
        "{}{}UMRS ANSI DEMO{}",
        AnsiColor::BrightWhite.start(),
        BOLD,
        RESET
    );
    println!("────────────────────────");

    // -----------------------------------------------------------------
    // Screen clearing (commented out by default)
    // -----------------------------------------------------------------
    //println!();
    //println!("(Screen clear examples are commented out)");
    // print!("{CLEAR_SCREEN}");
    // print!("{CURSOR_HOME}");

    // -----------------------------------------------------------------
    // Query sequences (not executed; shown for reference)
    // -----------------------------------------------------------------
    //println!();
    //println!("Query sequences (not sent):");
    //println!("  Cursor position query: {:?}", QUERY_CURSOR_POSITION);
    //println!("  Device attributes:     {:?}", QUERY_DEVICE_ATTRS);
    //println!("  Term size (chars):     {:?}", QUERY_TERM_SIZE_CHARS);
    //println!("  Term size (pixels):   {:?}", QUERY_TERM_SIZE_PIXELS);

    // Final newline ensures clean terminal state
    println!();



}
