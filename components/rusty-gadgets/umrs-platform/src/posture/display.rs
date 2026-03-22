// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # Display Annotations — Human-Readable Value Formatting
//!
//! Pure formatting functions that translate raw indicator values into
//! operator-facing display strings. These functions are the canonical
//! source for value annotations — any consumer (TUI, CLI, JSON output,
//! assessment reports) that needs human-readable indicator values must
//! call these functions rather than duplicating the annotation tables.
//!
//! ## Key Exported Functions
//!
//! - [`annotate_live_value`] — main entry point; routes any `LiveValue` to
//!   the appropriate annotator for the given `IndicatorId`
//! - [`annotate_integer`] — unsigned integer annotations (sysctl level values)
//! - [`annotate_signed_integer`] — signed integer annotations (e.g., `perf_event_paranoid`)
//!
//! ## Design
//!
//! Functions in this module are pure: they take typed inputs and return
//! `String` values. They perform no I/O and hold no state. Callers are
//! responsible for deciding how to render the returned string.
//!
//! The "absent" sentinel: `LiveValue::Text("absent")` is the internal value
//! used when a cmdline token is not present and when no modprobe blacklist
//! entry exists. `annotate_live_value` maps this to `"Not Present"` so
//! operators see an expected-absence label rather than an internal sentinel.
//!
//! ## Compliance
//!
//! - NIST SP 800-53 SA-5 (System Documentation) — centralised display logic
//!   ensures consistent, well-documented operator-facing output across all consumers.
//! - NIST SP 800-53 AU-3 (Content of Audit Records) — annotations provide
//!   the security-meaning context required for useful audit records.

use super::indicator::{IndicatorId, LiveValue};

// ---------------------------------------------------------------------------
// Primary entry point
// ---------------------------------------------------------------------------

/// Translate a `LiveValue` into an operator-readable display string.
///
/// Routes to `annotate_integer` or `annotate_signed_integer` for numeric
/// variants. Booleans are rendered as `"enabled"` / `"disabled"`. Text
/// values are passed through, except the internal sentinel `"absent"` which
/// is mapped to `"Not Present"`.
///
/// NIST SP 800-53 SA-5: operator-facing strings are defined in one place and
/// consumed by all display layers.
/// NIST SP 800-53 AU-3: annotated values provide security-meaning context in
/// audit output.
#[must_use = "annotated display string should be rendered to the operator"]
pub fn annotate_live_value(id: IndicatorId, live: &LiveValue) -> String {
    match live {
        LiveValue::Bool(true) => "enabled".to_owned(),
        LiveValue::Bool(false) => "disabled".to_owned(),
        LiveValue::Text(s) => {
            // "absent" is the internal sentinel used by the posture snapshot
            // when a cmdline token is not present in /proc/cmdline, and by
            // the modprobe reader when no blacklist entry exists. Display it
            // as "Not Present" so operators know this is an expected absence,
            // not a probe failure or I/O error.
            if s == "absent" {
                "Not Present".to_owned()
            } else {
                s.clone()
            }
        }
        LiveValue::Integer(v) => annotate_integer(id, u64::from(*v)),
        LiveValue::SignedInteger(v) => {
            annotate_signed_integer(id, i64::from(*v))
        }
    }
}

// ---------------------------------------------------------------------------
// Unsigned integer annotations
// ---------------------------------------------------------------------------

/// Annotate an unsigned integer value with its plain-language security meaning.
///
/// Returns `"<n> (<description>)"` for known indicator/value pairs, or
/// `"<n>"` when no annotation is defined for that combination. The annotation
/// table covers all sysctl-based indicators that have semantically meaningful
/// integer levels.
///
/// NIST SP 800-53 AU-3: annotated values provide the security-meaning context
/// required for operators to act on audit findings without a reference guide.
#[must_use = "annotated display string should be rendered to the operator"]
pub fn annotate_integer(id: IndicatorId, v: u64) -> String {
    let annotation: Option<&'static str> = match id {
        IndicatorId::RandomizeVaSpace => match v {
            0 => Some("ASLR disabled"),
            1 => Some("partial randomization"),
            2 => Some("full ASLR"),
            _ => None,
        },
        IndicatorId::KptrRestrict => match v {
            0 => Some("pointers visible"),
            1 => Some("hidden from unprivileged"),
            2 => Some("hidden from all users"),
            _ => None,
        },
        IndicatorId::UnprivBpfDisabled => match v {
            0 => Some("unprivileged BPF allowed"),
            1 => Some("restricted to CAP_BPF"),
            _ => None,
        },
        IndicatorId::YamaPtraceScope => match v {
            0 => Some("unrestricted"),
            1 => Some("children only"),
            2 => Some("admin only"),
            3 => Some("no attach"),
            _ => None,
        },
        IndicatorId::DmesgRestrict => match v {
            0 => Some("world-readable"),
            1 => Some("restricted"),
            _ => None,
        },
        IndicatorId::ModulesDisabled => match v {
            0 => Some("loading allowed"),
            1 => Some("loading locked"),
            _ => None,
        },
        IndicatorId::UnprivUsernsClone => match v {
            0 => Some("restricted"),
            1 => Some("allowed"),
            _ => None,
        },
        IndicatorId::Sysrq => match v {
            0 => Some("fully disabled"),
            1 => Some("all functions enabled"),
            _ => None,
        },
        IndicatorId::SuidDumpable => match v {
            0 => Some("no core dumps"),
            1 => Some("core dumps enabled"),
            2 => Some("readable by root only"),
            _ => None,
        },
        IndicatorId::ProtectedSymlinks | IndicatorId::ProtectedHardlinks => {
            match v {
                0 => Some("not protected"),
                1 => Some("protected"),
                _ => None,
            }
        }
        IndicatorId::ProtectedFifos | IndicatorId::ProtectedRegular => {
            match v {
                0 => Some("not protected"),
                1 => Some("partial protection"),
                2 => Some("fully protected"),
                _ => None,
            }
        }
        IndicatorId::FipsEnabled => match v {
            0 => Some("Disabled"),
            1 => Some("Enabled"),
            _ => None,
        },
        IndicatorId::NfConntrackAcct => match v {
            0 => Some("accounting off"),
            1 => Some("accounting on"),
            _ => None,
        },
        // Boolean-style indicators are handled via LiveValue::Bool.
        // CmdlineAbsent indicators render via LiveValue::Bool or LiveValue::Text.
        _ => None,
    };

    if let Some(note) = annotation {
        format!("{v} ({note})")
    } else {
        v.to_string()
    }
}

// ---------------------------------------------------------------------------
// Signed integer annotations
// ---------------------------------------------------------------------------

/// Annotate a signed integer value with its plain-language security meaning.
///
/// `perf_event_paranoid` is the primary signed indicator; negative values
/// grant broader access than zero. Returns `"<n> (<description>)"` for
/// known pairs, or `"<n>"` when no annotation is defined.
///
/// NIST SP 800-53 AU-3: negative kernel values (e.g., `-1` for `perf_event_paranoid`)
/// must be labelled clearly so operators understand the security implication
/// without consulting kernel documentation.
#[must_use = "annotated display string should be rendered to the operator"]
pub fn annotate_signed_integer(id: IndicatorId, v: i64) -> String {
    let annotation: Option<&'static str> = match id {
        IndicatorId::PerfEventParanoid => match v {
            i64::MIN..=-1 => Some("fully open"),
            0 => Some("kernel profiling allowed"),
            1 => Some("user profiling allowed"),
            2.. => Some("restricted"),
        },
        _ => None,
    };

    if let Some(note) = annotation {
        format!("{v} ({note})")
    } else {
        v.to_string()
    }
}
