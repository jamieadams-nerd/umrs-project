use umrs_core::console::ansi::*;

fn main() {
    print!(
        "{}{}Hello, ANSI world!{}",
        AnsiColor::Cyan.start(),
        BOLD,
        RESET
    );
}
