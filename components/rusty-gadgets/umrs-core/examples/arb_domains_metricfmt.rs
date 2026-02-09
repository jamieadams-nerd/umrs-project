use umrs_core::metricfmt::{auto_format, PrefixStyle};

fn main() {
    let freq = 0.000045_f64;   // Hz
    let volt = 12_300_000.0;  // V
    let dist = 0.00000032;    // meters

    let (f_s, _, _) = auto_format(freq, PrefixStyle::AbbrevAscii);
    let (v_s, _, _) = auto_format(volt, PrefixStyle::AbbrevAscii);
    let (d_s, _, _) = auto_format(dist, PrefixStyle::AbbrevAscii);

    println!("Frequency: {}Hz", f_s.replace(" ", ""));
    println!("Voltage:   {}V",  v_s.replace(" ", ""));
    println!("Distance:  {}m",  d_s.replace(" ", ""));
}
