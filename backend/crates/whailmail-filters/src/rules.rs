// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Filter compilation and criterion structures
//!
//! Converts raw filter definitions into compiled forms ready for matching.

use {
    crate::matcher::{EPatternComp, EPatternError, EPatternSrc},
    chrono::{DateTime, Utc},
    tracing::{debug, info},
    whailmail_common::types::SFilter
};

/// Compile glob into pattern (internal helper)
macro_rules! compile_glob {
    ($crit:expr, $field:ident) => {
        $crit
            .$field
            .as_ref()
            .map(|p| {
                EPatternComp::from_source(EPatternSrc::Glob(
                    p.clone(),
                    Default::default()
                ))
            })
            .transpose()?
    };
}

/// A single compiled criterion where all fields AND together
/// (everything must match for this criterion to match)
#[derive(Debug, Clone)]
pub struct SCompiledCriterion
{
    pub from_ptn: Option<EPatternComp>,
    pub to_ptn:   Option<EPatternComp>,
    pub cc_ptn:   Option<EPatternComp>,
    pub subj_ptn: Option<EPatternComp>,
    pub body_ptn: Option<EPatternComp>,

    pub has_attachments: Option<bool>,
    pub flags:           Option<Vec<String>>,
    pub min_size_bytes:  Option<u64>,
    pub max_size_bytes:  Option<u64>,

    pub received_after:  Option<DateTime<Utc>>,
    pub received_before: Option<DateTime<Utc>>,

    pub is_read:    Option<bool>,
    pub is_starred: Option<bool>
}

/// Compiled filter with all criteria ready to match against emails
///
/// Criteria are combined with OR logic (email matches if it matches ANY
/// criterion).
pub struct SCompiledFilter
{
    pub filter:            SFilter,
    pub compiled_criteria: Vec<SCompiledCriterion>
}

impl SCompiledFilter
{
    /// Compile a filter from its raw definition.
    ///
    /// Returns error if any glob or regex pattern is invalid.
    pub fn compile(filter: SFilter) -> Result<Self, EPatternError>
    {
        debug!(
            "Compiling filter '{}' with {} criteria",
            filter.name,
            filter.criteria.len()
        );

        let compiled_criteria = filter
            .criteria
            .iter()
            .map(|crit| {
                Ok(SCompiledCriterion {
                    from_ptn:        compile_glob!(crit, from_ptn),
                    to_ptn:          compile_glob!(crit, to_ptn),
                    cc_ptn:          compile_glob!(crit, cc_ptn),
                    subj_ptn:        compile_glob!(crit, subj_ptn),
                    body_ptn:        compile_glob!(crit, body_ptn),
                    has_attachments: crit.has_attachments,
                    flags:           crit.flags.clone(),
                    min_size_bytes:  crit.min_size_bytes,
                    max_size_bytes:  crit.max_size_bytes,
                    received_after:  crit.received_after,
                    received_before: crit.received_before,
                    is_read:         crit.is_read,
                    is_starred:      crit.is_starred
                })
            })
            .collect::<Result<Vec<_>, EPatternError>>()?;

        info!("Filter '{}' compiled successfully", filter.name);
        Ok(SCompiledFilter {
            filter,
            compiled_criteria
        })
    }
}
