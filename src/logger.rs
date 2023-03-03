// Copyright (C) 2023 Egor Poleshko
//
// This file is part of Eternacore.
//
// Eternacore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Eternacore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Eternacore.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::atomic::{AtomicUsize, Ordering};

use log::{Level, LevelFilter, Log};

pub const ANSI_DEFAULT: &str = "\x1B[0m";
pub const ANSI_FOREGROUND_RED: &str = "\x1B[31m";
pub const ANSI_FOREGROUND_DARK_GREEN: &str = "\x1B[32m";
pub const ANSI_FOREGROUND_DARK_YELLOW: &str = "\x1B[33m";
pub const ANSI_FOREGROUND_BRIGHT_BLUE: &str = "\x1B[94m";
pub const ANSI_FOREGROUND_DARK_MAGENTA: &str = "\x1B[95m";

// Thanks to pretty_env_logger for reliable implementation
// https://github.com/seanmonstar/pretty-env-logger/blob/master/src/lib.rs#L228
pub static TARGET_MAX_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn level_to_color(level: Level) -> &'static str {
    match level {
        Level::Error => ANSI_FOREGROUND_RED,
        Level::Warn => ANSI_FOREGROUND_DARK_YELLOW,
        Level::Info => ANSI_FOREGROUND_BRIGHT_BLUE,
        Level::Debug => ANSI_FOREGROUND_DARK_GREEN,
        Level::Trace => ANSI_FOREGROUND_DARK_MAGENTA,
    }
}
fn level_padding(level: Level) -> &'static str {
    if level == Level::Warn || level == Level::Info {
        " "
    } else {
        ""
    }
}
// TODO
fn generate_padding(_v: usize) -> String {
    /*let mut padding = String::new();
    for _ in 0..v {
        padding.push(' ');
    }
    padding
    */
    String::new()
}

#[derive(Debug)]
pub struct Logger(eternacore::config::Log);
impl Logger {
    pub fn from_config(conf: eternacore::config::Log) -> Result<(), log::SetLoggerError> {
        // Check whether config is valid
        if !conf.levels.is_disabled() && conf.max_level != LevelFilter::Off {
            panic!("both `max_level` and `levels` properties in eternacore.toml are defined");
        }

        // Set maximum level
        log::set_max_level(if conf.max_level != LevelFilter::Off {
            conf.max_level
        } else {
            conf.levels.to_level_filter()
        });

        // Set logger
        log::set_boxed_logger(Box::new(Self(conf)))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if (!self.0.levels.is_disabled() && self.0.max_level != LevelFilter::Off)
            || metadata.level().to_level_filter() <= self.0.max_level
        {
            true
        } else {
            false
        }
    }
    fn log(&self, record: &log::Record) {
        let target_max_width = TARGET_MAX_WIDTH.load(Ordering::Relaxed);
        let target_name_length = record.target().len();
        let target_max_width = if target_name_length > target_max_width {
            TARGET_MAX_WIDTH.store(target_name_length, Ordering::Relaxed);
            target_name_length - target_max_width
        } else {
            target_max_width
        };

        if !self.enabled(record.metadata())
            || (!self.0.debug && !record.target().starts_with(env!("CARGO_PKG_NAME")))
        {
            return;
        }
        println!(
            "{}",
            self.0
                .format
                .replace(
                    "%l",
                    &format!(
                        "{}{}{}{}",
                        level_padding(record.level()),
                        level_to_color(record.level()),
                        record.level(),
                        ANSI_DEFAULT
                    )
                )
                .replace(
                    "%t",
                    &format!("{}{}", record.target(), generate_padding(target_max_width))
                )
                .replace("%m", record.args().to_string().as_str())
        );
    }
    fn flush(&self) {}
}
