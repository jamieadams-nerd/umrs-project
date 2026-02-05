use umrs_core::sizefmt::{auto_format, format_in_unit, SizeBase, SizeUnit};

fn main() {
    let bytes = 1_500_000u128;

    // 1) Auto-pick the most appropriate unit
    let si = auto_format(bytes, SizeBase::Decimal);
    let iec = auto_format(bytes, SizeBase::Binary);

    println!("Auto (SI):  {}", si);   // "1.5 MB"
    println!("Auto (IEC): {}", iec);  // "1.43 MiB"

    // 2) Force a specific unit
    let as_mb  = format_in_unit(bytes, SizeUnit::MB);
    let as_mib = format_in_unit(bytes, SizeUnit::MiB);
    let as_gb  = format_in_unit(bytes, SizeUnit::GB);

    println!("Forced MB:  {}", as_mb);   // "1.5 MB"
    println!("Forced MiB: {}", as_mib);  // "1.43 MiB"
    println!("Forced GB:  {}", as_gb);   // "0.00 GB"
}
