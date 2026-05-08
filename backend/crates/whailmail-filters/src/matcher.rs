// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Email pattern matching engine
//!
//! Supports glob patterns, regexes, and plain string matching with configurable
//! flags.

use {
    globset::{GlobBuilder, GlobMatcher},
    regex::{Regex, RegexBuilder},
    std::fmt,
    tracing::{debug, error, trace}
};

/// Pattern matching error types
#[derive(Debug, Clone)]
pub enum EPatternError
{
    InvalidGlob(String),
    InvalidRegex(String)
}

impl fmt::Display for EPatternError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            | EPatternError::InvalidGlob(msg) =>
            {
                write!(f, "Invalid glob: {}", msg)
            },
            | EPatternError::InvalidRegex(msg) =>
            {
                write!(f, "Invalid regex: {}", msg)
            }
        }
    }
}

impl std::error::Error for EPatternError {}

/// Flags controlling pattern matching behaviour
#[derive(Debug, Clone, Copy)]
pub struct EPatternFlags
{
    /// Whether to match the full string.
    ///
    /// - `false`: partial match (substring match)
    /// - `true`: full-string match
    pub full_match:           bool,
    pub case_insensitive:     bool,
    pub multi_line:           bool,
    pub dot_matches_new_line: bool
}

impl Default for EPatternFlags
{
    fn default() -> Self
    {
        EPatternFlags {
            full_match:           false,
            case_insensitive:     false,
            multi_line:           false,
            dot_matches_new_line: false
        }
    }
}

/// Pattern source (pre-compilation)
#[derive(Debug, Clone)]
pub enum EPatternSrc
{
    Glob(String, EPatternFlags),
    Regex(String, EPatternFlags),
    String(String, EPatternFlags)
}

/// Compiled pattern (glob, regex, or literal string)
#[derive(Debug, Clone)]
pub enum EPatternComp
{
    Glob
    {
        source:  String,
        matcher: GlobMatcher,
        flags:   EPatternFlags
    },
    Regex
    {
        source:   String,
        compiled: Regex,
        flags:    EPatternFlags
    },
    String(String, EPatternFlags)
}

impl EPatternComp
{
    /// Compile a pattern from source. Returns error if glob/regex is invalid
    pub fn from_source(source: EPatternSrc) -> Result<Self, EPatternError>
    {
        match source
        {
            | EPatternSrc::Glob(pattern, flags) =>
            {
                debug!(
                    "Compiling glob pattern (case_insensitive={}): {}",
                    flags.case_insensitive, pattern
                );
                trace!(
                    "Globs are automatically full-match, multi-line, and . is \
                     a literal. Discarding those flags for {}",
                    pattern
                );
                let glob = GlobBuilder::new(&pattern)
                    .case_insensitive(flags.case_insensitive)
                    .build()
                    .map_err(|e| {
                        error!(
                            "Glob compilation failed for {}: {}",
                            pattern, e
                        );
                        EPatternError::InvalidGlob(e.to_string())
                    })?;
                Ok(EPatternComp::Glob {
                    source: pattern,
                    matcher: glob.compile_matcher(),
                    flags
                })
            },
            | EPatternSrc::Regex(pattern, flags) =>
            {
                let pattern_with_anchors = if flags.full_match
                {
                    format!("^(?:{})$", pattern)
                }
                else
                {
                    pattern.clone()
                };
                debug!(
                    "Compiling regex (full_match={}, case_insensitive={}): {}",
                    flags.full_match, flags.case_insensitive, pattern
                );
                let compiled = RegexBuilder::new(&pattern_with_anchors)
                    .case_insensitive(flags.case_insensitive)
                    .multi_line(flags.multi_line)
                    .dot_matches_new_line(flags.dot_matches_new_line)
                    .unicode(true)
                    .build()
                    .map_err(|e| {
                        error!(
                            "Regex compilation failed for {}: {}",
                            pattern, e
                        );
                        EPatternError::InvalidRegex(e.to_string())
                    })?;
                Ok(EPatternComp::Regex {
                    source: pattern,
                    compiled,
                    flags
                })
            },
            | EPatternSrc::String(pattern, flags) =>
            {
                debug!(
                    "Using string pattern (full_match={}): {}",
                    flags.full_match, pattern
                );
                Ok(EPatternComp::String(pattern, flags))
            }
        }
    }

    /// Check if the pattern matches the text
    #[inline]
    pub fn is_match(&self, text: &str) -> bool
    {
        match self
        {
            | EPatternComp::Glob {
                matcher, ..
            } => matcher.is_match(text),
            | EPatternComp::Regex {
                compiled, ..
            } => compiled.is_match(text),
            | EPatternComp::String(substr, flags) =>
            {
                match flags.full_match
                {
                    | true => text == substr,
                    | false => text.contains(substr)
                }
            },
        }
    }

    /// Check if the pattern matches any text in the slice
    pub fn is_match_any<S: AsRef<str>>(&self, texts: &[S]) -> bool
    {
        texts.iter().any(|text| self.is_match(text.as_ref()))
    }
}
