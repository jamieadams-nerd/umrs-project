// SPDX-License-Identifier: MIT                                                                     
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
//! SELinux runtime status inspection.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelinuxStatus {
    enabled: bool,
    enforcing: bool,
}

impl SelinuxStatus {
    #[must_use]
    pub const fn new(enabled: bool, enforcing: bool) -> Self {
        Self { enabled, enforcing }
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub const fn enforcing(&self) -> bool {
        self.enforcing
    }
}
