use humansize::{BINARY, DECIMAL, format_size};

#[derive(Debug, Clone, Copy)]
pub enum SizeBase {
    Decimal, // 1000-based (KB, MB, GB, …)
    Binary,  // 1024-based (KiB, MiB, GiB, …)
}

#[derive(Debug, Clone, Copy)]
pub enum SizeUnit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
    EB,
    ZB,
    YB,

    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
    EiB,
    ZiB,
    YiB,
}

fn unit_divisor(unit: SizeUnit) -> u128 {
    match unit {
        SizeUnit::B => 1,

        // Decimal (SI)
        SizeUnit::KB => 1_000u128.pow(1),
        SizeUnit::MB => 1_000u128.pow(2),
        SizeUnit::GB => 1_000u128.pow(3),
        SizeUnit::TB => 1_000u128.pow(4),
        SizeUnit::PB => 1_000u128.pow(5),
        SizeUnit::EB => 1_000u128.pow(6),
        SizeUnit::ZB => 1_000u128.pow(7),
        SizeUnit::YB => 1_000u128.pow(8),

        // Binary (IEC)
        SizeUnit::KiB => 1_024u128.pow(1),
        SizeUnit::MiB => 1_024u128.pow(2),
        SizeUnit::GiB => 1_024u128.pow(3),
        SizeUnit::TiB => 1_024u128.pow(4),
        SizeUnit::PiB => 1_024u128.pow(5),
        SizeUnit::EiB => 1_024u128.pow(6),
        SizeUnit::ZiB => 1_024u128.pow(7),
        SizeUnit::YiB => 1_024u128.pow(8),
    }
}

fn unit_label(unit: SizeUnit) -> &'static str {
    match unit {
        SizeUnit::B => "B",

        SizeUnit::KB => "KB",
        SizeUnit::MB => "MB",
        SizeUnit::GB => "GB",
        SizeUnit::TB => "TB",
        SizeUnit::PB => "PB",
        SizeUnit::EB => "EB",
        SizeUnit::ZB => "ZB",
        SizeUnit::YB => "YB",

        SizeUnit::KiB => "KiB",
        SizeUnit::MiB => "MiB",
        SizeUnit::GiB => "GiB",
        SizeUnit::TiB => "TiB",
        SizeUnit::PiB => "PiB",
        SizeUnit::EiB => "EiB",
        SizeUnit::ZiB => "ZiB",
        SizeUnit::YiB => "YiB",
    }
}

pub fn auto_format(bytes: u128, base: SizeBase) -> String {
    match base {
        SizeBase::Decimal => format_size(bytes as u64, DECIMAL),
        SizeBase::Binary => format_size(bytes as u64, BINARY),
    }
}

pub fn format_in_unit(bytes: u128, unit: SizeUnit) -> String {
    let divisor = unit_divisor(unit);
    let value = bytes as f64 / divisor as f64;

    let formatted = if value >= 100.0 {
        format!("{:.0}", value)
    } else if value >= 10.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    };

    format!("{} {}", formatted, unit_label(unit))
}
