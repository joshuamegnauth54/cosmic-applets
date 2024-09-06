// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod applet;
pub mod backend;
pub mod config;
pub mod error;
mod localize;
pub mod weather;

use applet::Applet;

pub fn run() -> cosmic::iced::Result {
    localize::localize();
    cosmic::applet::run::<Applet>(true, ())
}
