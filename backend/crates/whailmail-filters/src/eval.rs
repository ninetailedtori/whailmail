// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Email evaluation against compiled filters
//!
//! Core matching logic: does an email satisfy a filter's criteria?

use {
    crate::rules::{SCompiledCriterion, SCompiledFilter},
    tracing::debug,
    whailmail_common::SEmail
};

/// Check if pattern matches email field (internal)
macro_rules! pattern_check {
    (
        $crit:expr, $email:expr, $crit_field:ident, $email_field:ident,is_match
    ) => {
        if let Some(p) = &$crit.$crit_field
        {
            if !p.is_match(&$email.$email_field)
            {
                return false;
            }
        }
    };

    (
        $crit:expr,
        $email:expr,
        $crit_field:ident,
        $email_field:ident,is_match_any
    ) => {
        if let Some(p) = &$crit.$crit_field
        {
            if !p.is_match_any(&$email.$email_field)
            {
                return false;
            }
        }
    };

    (
        $crit:expr, $email:expr, $crit_field:ident, $email_field:ident,contains
    ) => {
        if let Some(substr) = &$crit.$crit_field
        {
            if !$email.$email_field.contains(substr.as_str())
            {
                return false;
            }
        }
    };

    ($crit:expr, $email:expr, $crit_field:ident, $email_field:ident,eq) => {
        if let Some(val) = $crit.$crit_field
        {
            if $email.$email_field != val
            {
                return false;
            }
        }
    };

    (
        $crit:expr, $email:expr, $crit_field:ident, $email_field:ident,any_flag
    ) => {
        if let Some(filter_flags) = &$crit.$crit_field
        {
            if !$email.$email_field.iter().any(|f| filter_flags.contains(f))
            {
                return false;
            }
        }
    };
}

#[inline]
fn check_bounds<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: Option<T>,
    max: Option<T>,
    label: &str
) -> bool
{
    min.is_none_or(|m| {
        if value >= m
        {
            true
        }
        else
        {
            debug!("{} rejected: {} < min {}", label, value, m);
            false
        }
    }) && max.is_none_or(|m| {
        if value <= m
        {
            true
        }
        else
        {
            debug!("{} rejected: {} > max {}", label, value, m);
            false
        }
    })
}

impl SCompiledFilter
{
    /// Check if email matches ANY criterion in this filter (OR logic)
    ///
    /// Returns `true` if the email satisfies at least one criterion.
    pub fn is_match_any(&self, email: &SEmail) -> bool
    {
        let matched = self
            .compiled_criteria
            .iter()
            .any(|crit| self.is_match_all(crit, email));

        debug!(
            "is_match_any: email {} has {} matches for at least one of \
             criteria under filter {}",
            email.message_id,
            if matched { "" } else { "no" },
            self.filter.name
        );
        matched
    }

    /// Check if email matches ALL fields in a criterion (AND logic)
    ///
    /// Returns `true` if the email satisfies every field in the criterion.
    fn is_match_all(&self, crit: &SCompiledCriterion, email: &SEmail) -> bool
    {
        // Pattern matching (address, subject, body)
        pattern_check!(crit, email, from_ptn, from, is_match);
        pattern_check!(crit, email, to_ptn, to, is_match_any);
        pattern_check!(crit, email, cc_ptn, cc, is_match_any);
        pattern_check!(crit, email, subj_ptn, subject, is_match);
        pattern_check!(crit, email, body_ptn, body_text, is_match);

        // State checks
        pattern_check!(crit, email, has_attachments, has_attachments, eq);
        pattern_check!(crit, email, is_read, is_read, eq);
        pattern_check!(crit, email, is_starred, is_starred, eq);

        // Flag checks (ANY of the filter flags must be present)
        pattern_check!(crit, email, flags, flags, any_flag);

        // Size checks
        let email_size = email.body_text.len() + email.subject.len();
        if !check_bounds(
            email_size as u64,
            crit.min_size_bytes,
            crit.max_size_bytes,
            "Email size"
        )
        {
            return false;
        }

        // Date checks
        if !check_bounds(
            email.received_at,
            crit.received_after,
            crit.received_before,
            "Email received_at"
        )
        {
            return false;
        }

        debug!(
            "is_match_all: email {} matches all criteria under filter {}",
            email.message_id, self.filter.name
        );
        true
    }
}
