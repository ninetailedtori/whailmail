// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

use {
    globset::{GlobBuilder, GlobMatcher},
    regex::Regex,
    std::fmt,
    whailmail_common::types::SFilter
};

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

// Internal representation
#[derive(Debug, Clone)]
pub enum EPatternComp
{
    Glob
    {
        source:  String,
        matcher: GlobMatcher
    },
    Regex
    {
        source: String, compiled: Regex
    }
}

#[derive(Debug, Clone)]
pub enum EPatternSrc
{
    Glob(String),
    Regex(String)
}

impl EPatternComp
{
    pub fn from_source(source: EPatternSrc) -> Result<Self, EPatternError>
    {
        match source
        {
            | EPatternSrc::Glob(pattern) =>
            {
                let glob = GlobBuilder::new(&pattern)
                    .case_insensitive(true)
                    .build()
                    .map_err(|e| EPatternError::InvalidGlob(e.to_string()))?;
                Ok(EPatternComp::Glob {
                    source:  pattern,
                    matcher: glob.compile_matcher()
                })
            },
            | EPatternSrc::Regex(pattern) =>
            {
                let compiled = Regex::new(&pattern)
                    .map_err(|e| EPatternError::InvalidRegex(e.to_string()))?;
                Ok(EPatternComp::Regex {
                    source: pattern,
                    compiled
                })
            }
        }
    }

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
            } => compiled.is_match(text)
        }
    }
}

pub struct SCompiledFilter
{
    pub filter:              SFilter,
    pub compiled_conditions: SCompiledConditions
}

pub struct SCompiledConditions
{
    pub from_pattern:    Option<EPatternComp>,
    pub to_pattern:      Option<EPatternComp>,
    pub subject_pattern: Option<EPatternComp>,
    pub body_contains:   Option<String>
}

impl SCompiledFilter
{
    pub fn compile(filter: SFilter) -> Result<Self, EPatternError>
    {
        Ok(SCompiledFilter {
            compiled_conditions: SCompiledConditions {
                from_pattern:    filter
                    .conditions
                    .from_pattern
                    .as_ref()
                    .map(|p| {
                        EPatternComp::from_source(EPatternSrc::Glob(p.clone()))
                    })
                    .transpose()?,
                to_pattern:      filter
                    .conditions
                    .to_pattern
                    .as_ref()
                    .map(|p| {
                        EPatternComp::from_source(EPatternSrc::Glob(p.clone()))
                    })
                    .transpose()?,
                subject_pattern: filter
                    .conditions
                    .subject_pattern
                    .as_ref()
                    .map(|p| {
                        EPatternComp::from_source(EPatternSrc::Glob(p.clone()))
                    })
                    .transpose()?,
                body_contains:   filter.conditions.body_contains.clone()
            },
            filter
        })
    }

    pub fn matches(&self, email: &SEmail) -> bool
    {
        let conditions = &self.compiled_conditions;

        if let Some(p) = &conditions.from_pattern
        {
            if !p.is_match(&email.from)
            {
                return false;
            }
        }

        if let Some(p) = &conditions.to_pattern
        {
            if !p.is_match(&email.to)
            {
                return false;
            }
        }

        if let Some(p) = &conditions.subject_pattern
        {
            if !p.is_match(&email.subject)
            {
                return false;
            }
        }

        if let Some(substr) = &conditions.body_contains
        {
            if !email.body.contains(substr.as_str())
            {
                return false;
            }
        }

        true
    }
}

pub struct SEmail
{
    pub from:    String,
    pub to:      String,
    pub subject: String,
    pub body:    String
}
