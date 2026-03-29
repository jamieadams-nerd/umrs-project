//
// Example for size formatting.
//

// Crate loading ordering...

// 1) Standard Library
use std::process::ExitCode;

// 2) External crates (third-party)
// use clap::Parser

// 3) local crates (my workspace)
use umrs_core::human::metricfmt::{
    FormatOptions, FormatWarning, PrefixStyle, SIPrefix, auto_format,
    auto_format_with_options, format_in_prefix,
};

fn main() -> ExitCode {
    println!(
        "\nExample 1 — Auto-scaling for time (seconds → ms / µs / ns / etc.)"
    );
    let t1 = 0.000001234_f64; // 1.234 microseconds
    let t2 = 12.345678_f64; // ~12 seconds
    let t3 = 0.000000000045; // 45 picoseconds

    let (s1, _, w1) = auto_format(t1, PrefixStyle::AbbrevAscii);
    assert_eq!(s1, "1.23 u");
    assert_eq!(w1, None);

    let (s2, _, w2) = auto_format(t2, PrefixStyle::AbbrevAscii);
    assert_eq!(s2, "12.35 ");
    assert_eq!(w2, None);

    let (s3, _, w3) = auto_format(t3, PrefixStyle::AbbrevAscii);
    assert_eq!(s3, "45.00 p");
    assert_eq!(w3, None);

    let _msg = format!("t1 auto: {}s  {:?}", s1.replace(" ", ""), w1);
    let _msg = format!("t2 auto: {}s  {:?}", s2.replace(" ", ""), w2);
    let _msg = format!("t3 auto: {}s  {:?}", s3.replace(" ", ""), w3);

    println!("\nExample 2 — Auto-scaling with full-text prefixes");
    let v = 0.0000023_f64; // volts
    let (s, _, _) = auto_format(v, PrefixStyle::FullText);
    let _final_str = format!("{} volts", s.trim());

    println!("\nExample 3 — Forced prefix (and detecting precision loss)");
    let t = 0.000000000001234_f64; // 1.234 picoseconds

    let (s1, w1) =
        format_in_prefix(t, SIPrefix::Pico, PrefixStyle::AbbrevAscii);
    let (s2, w2) =
        format_in_prefix(t, SIPrefix::Milli, PrefixStyle::AbbrevAscii);

    let _msg = format!("Forced pico: {}s  {:?}", s1.replace(" ", ""), w1);
    let _msg = format!("Forced milli: {}s {:?}", s2.replace(" ", ""), w2);

    if w2 == Some(FormatWarning::PrecisionLoss) {
        // value lost at display precision — caller should warn
    }

    println!("\nExample 4 — Using custom formatting options");
    let v = 0.00000098765_f64;

    let opts = FormatOptions {
        decimals: 4,
        auto_target_min: 0.5,
        auto_target_max: 500.0,
    };

    let (s, _, _) =
        auto_format_with_options(v, PrefixStyle::AbbrevAscii, &opts);
    let _final_str = format!("{}Hz", s.replace(" ", ""));

    println!(
        "\nExample 5 — Voltage, frequency, and arbitrary domains (unit-agnostic)"
    );
    let freq = 0.000045_f64; // Hz
    let volt = 12_300_000.0; // V
    let dist = 0.00000032; // meters

    let (f_s, _, _) = auto_format(freq, PrefixStyle::AbbrevAscii);
    let (v_s, _, _) = auto_format(volt, PrefixStyle::AbbrevAscii);
    let (d_s, _, _) = auto_format(dist, PrefixStyle::AbbrevAscii);

    let _msg = format!("Frequency: {}Hz", f_s.replace(" ", ""));
    let _msg = format!("Voltage:   {}V", v_s.replace(" ", ""));
    let _msg = format!("Distance:  {}m", d_s.replace(" ", ""));

    ExitCode::SUCCESS
}
