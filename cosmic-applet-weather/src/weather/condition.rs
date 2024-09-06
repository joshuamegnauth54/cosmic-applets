// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    /// Little or no clouds; also called Sunny
    Clear,
    PartlyCloudy,
    Cloudy,
    /// Very cloudy
    Overcast,
    Fog,
    LightShowers,
    HeavyShowers,
    ThunderyShowers,
    LightRain,
    HeavyRain,
    ThunderyHeavyRain,
    LightSleet,
    LightSleetShowers,
    LightSnowShowers,
    HeavySnowShowers,
    ThunderySnowShowers,
    LightSnow,
    HeavySnow,
}

impl Condition {
    /// Weather World Online code
    ///
    /// See: https://www.worldweatheronline.com/developer/api/docs/weather-icons.aspx
    pub const fn from_wwo(code: u16) -> Option<Self> {
        use Condition::*;
        match code {
            113 => Some(Clear),
            116 => Some(PartlyCloudy),
            119 => Some(Cloudy),
            122 => Some(Overcast),
            143 | 248 | 260 => Some(Fog),
            176 | 263 | 353 => Some(LightShowers),
            179 | 362 | 365 | 374 => Some(LightSleetShowers),
            182 | 185 | 281 | 284 | 311 | 314 | 317 | 350 | 377 => Some(LightSleet),
            200 | 386 => Some(ThunderyShowers),
            227 | 320 => Some(LightSnow),
            230 | 338 => Some(HeavySnow),
            266 | 293 | 296 => Some(LightRain),
            299 | 305 | 356 => Some(HeavyShowers),
            302 | 308 | 359 => Some(HeavyRain),
            320 => Some(LightSnow),
            323 | 326 | 368 => Some(LightSnowShowers),
            329 | 332 => Some(HeavySnow),
            335 | 371 | 395 => Some(HeavySnowShowers),
            389 => Some(ThunderyHeavyRain),
            392 => Some(ThunderySnowShowers),
            _ => None,
        }
    }

    pub fn wwo_from_str(s: &str) -> Option<Self> {
        s.parse().map(Self::from_wwo).ok()
    }
}

impl fmt::Display for Condition {
    // Borrowed from wttr.in
    // See: https://github.com/chubin/wttr.in/blob/master/lib/constants.py
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {}
}
