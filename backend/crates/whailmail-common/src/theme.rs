// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EFlavour
{
    Latte,
    Frappe,
    Macchiato,
    Mocha
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EAccent
{
    Rosewater,
    Flamingo,
    Pink,
    Mauve,
    Red,
    Maroon,
    Peach,
    Yellow,
    Green,
    Teal,
    Sky,
    Sapphire,
    Blue,
    Lavender
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SThemeConfig
{
    pub flavour: EFlavour,
    pub accent:  EAccent
}

impl Default for SThemeConfig
{
    fn default() -> Self
    {
        Self {
            flavour: EFlavour::Macchiato,
            accent:  EAccent::Lavender
        }
    }
}
