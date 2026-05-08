// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Email filter compilation and matching engine (๑•́ ω •̀)و ✧
//!
//! **WhailMail Filter Engine**: compose, compile, and execute mail filters with
//! glob, regex, and literal patterns. Criteria combine with AND logic (all must
//! match), filters combine with OR logic (any criterion matches).
//!
//! # Example
//!
//! ```no_run
//! use whailmail_filters::{executor::execute_batch, rules::SCompiledFilter};
//!
//! // Compile a filter
//! let compiled = SCompiledFilter::compile(my_filter)?;
//!
//! // Execute against emails
//! let results = execute_batch(&[compiled], &emails);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod eval;
pub mod exec;
pub mod matcher;
pub mod rules;

// Public re-exports for convenience
pub use {
    exec::{
        execute_batch,
        execute_filter,
        execute_filters,
        filter_matched,
        filter_rejected
    },
    matcher::{EPatternComp, EPatternError, EPatternFlags, EPatternSrc},
    rules::{SCompiledCriterion, SCompiledFilter}
};
