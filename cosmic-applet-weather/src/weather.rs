// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod condition;
pub mod moon;
pub mod temperature;
pub mod wind_speed;

pub use condition::*;
pub use moon::*;
pub use temperature::*;
pub use wind_speed::*;

use async_trait::async_trait;

use crate::error::Error;

/// A source of weather information such a REST API.
#[async_trait]
pub trait WeatherSource {
    fn new(client: reqwest::Client) -> Self;

    /// The current weather as closest to now as possible.
    async fn now(&mut self) -> Result<Weather, Error>;
    async fn hours(&mut self, hours: u32) -> Result<Vec<Weather>, Error>;
    async fn days(&mut self, days: u32) -> Result<Vec<Weather>, Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct Weather {
    pub temp: Option<Temperature>,
    pub condition: Option<Condition>,
    pub humidity: Option<f64>,
    pub moon_phase: Option<MoonPhase>,
    pub wind_speed: Option<WindSpeed>,
}
