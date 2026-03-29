use umrs_core::human::metricfmt::{
    FormatOptions, PrefixStyle, auto_format_with_options,
};

fn main() {
    let v = 0.00000098765_f64;

    let opts = FormatOptions {
        decimals: 4,
        auto_target_min: 0.5,
        auto_target_max: 500.0,
    };

    let (s, _, _) =
        auto_format_with_options(v, PrefixStyle::AbbrevAscii, &opts);
    let final_str = format!("{}Hz", s.replace(" ", ""));

    println!("{}", final_str);
}
