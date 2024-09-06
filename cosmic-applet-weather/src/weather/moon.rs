// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoonPhase {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    ThirdQuarter,
    WaningCrescent,
}

impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::New => "🌑",
            Self::WaxingCrescent => "🌒",
            Self::FirstQuarter => "🌓",
            Self::WaxingGibbous => "🌔",
            Self::Full => "🌕",
            Self::WaningGibbous => "🌖",
            Self::ThirdQuarter => "🌗",
            Self::WaningCrescent => "🌘",
        }
    }
}
