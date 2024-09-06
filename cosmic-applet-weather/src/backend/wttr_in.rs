// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::{DateTime, Local, NaiveTime};
use serde::Deserialize;

use crate::weather::{Condition, MoonPhase, Temperature, WeatherSource, WindSpeed};

const BASE_URL: &str = "https://wttr.in";

pub struct WttrIn {
    client: reqwest::Client,
}

impl WeatherSource for WttrIn {
    const fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    pub current_condition: CurrentCondition,
    #[serde(skip)]
    pub nearest_area: Option<Vec<NearestArea>>,
    #[serde(skip)]
    pub request: Option<Vec<WeatherRequest>>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(deserialize_with = "Temperature::from_str_celsius")]
    feels_like_c: Temperature,
    #[serde(deserialize_with = "Temperature::from_str_fahrenheit")]
    feels_like_f: Temperature,
    cloud_cover: u16,
    humidity: u16,
    local_obs_date_time: DateTime<Local>,
    observation_time: NaiveTime,
    precip_inches: f32,
    precip_mm: f32,
    pressure: u16,
    pressure_inches: u16,
    #[serde(deserialize_with = "Temperature::from_str_celsius")]
    temp_c: Temperature,
    #[serde(deserialize_with = "Temperature::from_str_fahrenheit")]
    temp_f: Temperature,
    #[serde(deserialize_with = "Condition::wwo_from_str")]
    weather_code: Condition,
    #[serde(deserialize_with = "")]
    weather_desc: Vec<Condition>,
    #[serde(skip)]
    weather_icon_url: Option<Vec<reqwest::Url>>,
    wind_dir_16_point: String,
    wind_dir_degree: u16,
    #[serde(deserialize_with = "WindSpeed::from_str_khm")]
    wind_speed_kmph: WindSpeed,
    #[serde(deserialize_with = "WindSpeed::from_str_mph")]
    wind_speed_miles: WindSpeed,
}

#[derive(Debug, Deserialize)]
struct NearestArea {
    area_name: Vec<String>,
    country: Vec<String>,
    latitude: f64,
    longitude: f64,
    population: u32,
    region: Vec<String>,
    weather_url: Option<Vec<reqwest::Url>>,
}

#[derive(Debug, Deserialize)]
struct WeatherRequest {
    pub query: String,
    #[serde(rename = "type")]
    pub qtype: String,
}

#[derive(Debug, Deserialize)]
struct Weather {
    astronomy: Vec<Astronomy>,
    #[serde(deserialize_with = "Temperature::from_str_celsius")]
    avg_temp_c: Temperature,
    #[serde(deserialize_with = "Temperature::from_str_fahrenheit")]
    avg_temp_f: Temperature,
    date: DateTime<Local>,
    hourly: Vec<WeatherHourly>
}

#[derive(Debug, Deserialize)]
struct Astronomy {
    moon_illumination: u8,
    moon_phase: MoonPhase,
    moonrise: NaiveTime,
    moonset: NaiveTime,
    sunrise: NaiveTime,
    sunset: NaiveTime
}
