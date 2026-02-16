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
    pub fn new(enabled: bool, enforcing: bool) -> Self {
        Self { enabled, enforcing }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enforcing(&self) -> bool {
        self.enforcing
    }
}
