// SPDX-License-Identifier: MIT                                                                     
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//!
//! SELinux Multi-Category Security (MCS) Namespace
//!
//! In the context of SELinux, MCS stands for Multi-Category Security. It is a security mechanism
//! used to compartmentalize data by assigning specific "categories" (labels) to files and
//! processes, ensuring that only authorized entities can access them. 
//!
//! MCS is technically a simplified version of Multi-Level Security (MLS). While MLS uses
//! hierarchical "sensitivity levels" (e.g., Public vs. Top Secret), MCS uses non-hierarchical
//! categories (e.g., Project A, Payroll, or HR).
//!
// ===========================================================================
pub mod setrans;
