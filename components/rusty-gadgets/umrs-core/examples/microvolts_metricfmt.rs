use umrs_core::metricfmt::{
    format_in_prefix,
    PrefixStyle,
    SIPrefix,
    FormatWarning,
};

fn main() {
    let t = 0.000000000001234_f64; // 1.234 picoseconds

    let (s1, w1) = format_in_prefix(t, SIPrefix::Pico, PrefixStyle::AbbrevAscii);
    let (s2, w2) = format_in_prefix(t, SIPrefix::Milli, PrefixStyle::AbbrevAscii);

    println!("Forced pico: {}s  {:?}", s1.replace(" ", ""), w1);
    println!("Forced milli: {}s {:?}", s2.replace(" ", ""), w2);

    if w2 == Some(FormatWarning::PrecisionLoss) {
        eprintln!("WARNING: Forced scale too coarse; value lost at display precision");
    }
}
