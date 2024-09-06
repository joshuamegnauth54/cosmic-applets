// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Error {
    BadAuth,
    RateLimited(Option<Duration>),
    Request(reqwest::Error)
}
