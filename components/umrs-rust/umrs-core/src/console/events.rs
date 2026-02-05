// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//
use crate::verbose;

#[macro_export]
macro_rules! console_event {
    ($event:expr) => {{ $crate::console::__console_emit($event) }};
}

pub enum ConsoleEvent<'a> {
    // Common pairs of events
    BeginTask {
        name: &'a str,
    },
    EndTask {
        name: &'a str,
    },

    FileOpen {
        path: &'a str,
    },
    FileClose {
        path: &'a str,
    },

    DataRead {
        path: &'a str,
    },
    DataWrote {
        path: &'a str,
    },

    // Single Events
    FileNotFound {
        path: &'a str,
    },
}

impl<'a> ConsoleEvent<'a> {
    /// Render into a human-facing message.
    ///
    /// Private: this is presentation logic, not API.
    fn render(&self) -> String {
        match self {
            // Common Paired Events
            ConsoleEvent::BeginTask {
                name,
            } => format!("\u{27E6}  Begin. {}", name),
            ConsoleEvent::EndTask {
                name,
            } => format!("\u{27E7}  End. {}", name),

            ConsoleEvent::FileOpen {
                path,
            } => format!("\u{1F5C0}  Opening {}", path),
            ConsoleEvent::FileClose {
                path,
            } => format!("\u{2394}  Closing {}", path),

            ConsoleEvent::DataRead {
                path,
            } => format!("\u{26C1}  Read {}", path),
            ConsoleEvent::DataWrote {
                path,
            } => format!("\u{26C3}  Wrote {}", path),

            // Common Single events
            ConsoleEvent::FileNotFound {
                path,
            } => format!("\u{2715}  Not found: {}", path),
            // Common Single events
        }
    }
}

// ==================================================================
// PRIVATE Private emission helper. -- Never part of the public API.
// ==================================================================
#[allow(unused)]
pub fn __console_emit(event: ConsoleEvent<'_>) {
    let message = event.render();
    verbose!("  {}", message);
}
