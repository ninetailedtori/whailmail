// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Filter execution and batch application
//!
//! Apply compiled filters to collections of emails and collect results.

use {
    crate::rules::SCompiledFilter,
    tracing::{debug, info},
    whailmail_common::SEmail
};

/// Result of applying a filter to an email
#[derive(Debug, Clone)]
pub struct SFilterResult
{
    /// The email's message ID
    pub message_id:  String,
    /// Which filter matched (if any)
    pub filter_name: String,
    /// Whether the email matched the filter
    pub matched:     bool
}

/// Execute a single filter against an email
///
/// Returns a `SFilterResult` with the matching outcome.
pub fn execute_filter(filter: &SCompiledFilter, email: &SEmail)
-> SFilterResult
{
    let matched = filter.is_match_any(email);

    debug!(
        "Filter execution: email '{}' vs filter '{}' = {}",
        email.message_id, filter.filter.name, matched
    );

    SFilterResult {
        message_id: email.message_id.clone(),
        filter_name: filter.filter.name.clone(),
        matched
    }
}

/// Execute multiple filters against a single email
///
/// Returns a `Vec` of results, one per filter.
pub fn execute_filters(
    filters: &[SCompiledFilter],
    email: &SEmail
) -> Vec<SFilterResult>
{
    filters.iter().map(|f| execute_filter(f, email)).collect()
}

/// Execute filters against a batch of emails
///
/// Returns a `Vec<Vec<SFilterResult>>` with results for each email.
pub fn execute_batch(
    filters: &[SCompiledFilter],
    emails: &[SEmail]
) -> Vec<Vec<SFilterResult>>
{
    info!(
        "Executing {} filter(s) against {} email(s)",
        filters.len(),
        emails.len()
    );

    let results = emails
        .iter()
        .map(|email| execute_filters(filters, email))
        .collect();

    debug!("Batch filter execution complete");
    results
}

/// Apply filters and collect matched emails
///
/// Useful for filtering operations: returns only emails that matched at least
/// one filter.
pub fn filter_matched<'a>(
    filters: &[SCompiledFilter],
    emails: &'a [SEmail]
) -> Vec<&'a SEmail>
{
    emails
        .iter()
        .filter(|email| filters.iter().any(|f| f.is_match_any(email)))
        .collect()
}

/// Apply filters and collect rejected emails
///
/// Useful for exclusion operations: returns only emails that didn't match any
/// filter.
pub fn filter_rejected<'a>(
    filters: &[SCompiledFilter],
    emails: &'a [SEmail]
) -> Vec<&'a SEmail>
{
    emails
        .iter()
        .filter(|email| !filters.iter().any(|f| f.is_match_any(email)))
        .collect()
}
