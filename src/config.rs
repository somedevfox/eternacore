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
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug)]
pub struct LogLevels {
    pub error: bool,
    pub warn: bool,
    pub info: bool,
    pub debug: bool,
    pub trace: bool,
}
impl LogLevels {
    pub fn is_disabled(&self) -> bool {
        if self.error == false
            && self.warn == false
            && self.info == false
            && self.debug == false
            && self.trace == false
        {
            true
        } else {
            false
        }
    }

    pub fn to_level_filter(self) -> LevelFilter {
        // If this can be implemented in a better way - please, go ahead
        if self.trace {
            LevelFilter::Trace
        } else if self.debug {
            LevelFilter::Debug
        } else if self.info {
            LevelFilter::Info
        } else if self.warn {
            LevelFilter::Warn
        } else if self.error {
            LevelFilter::Error
        } else {
            LevelFilter::Off
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(remote = "LevelFilter")]
enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Log {
    #[serde(with = "LevelFilterDef")]
    pub max_level: LevelFilter,
    pub levels: LogLevels,
    pub format: String,
    pub debug: bool,
}
impl Default for Log {
    fn default() -> Self {
        Self {
            max_level: LevelFilter::Off,
            levels: Default::default(),
            format: String::from("%l | %t | %m"),
            debug: false,
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Intents {
    pub presence: bool,
    pub server_members: bool,
    pub message_content: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Discord {
    pub token: String,
    pub intents: Intents,
    pub shards: u64,
}
impl Default for Discord {
    fn default() -> Self {
        Self {
            token: String::new(),
            intents: Default::default(),
            shards: 5,
        }
    }
}
#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub discord: Discord,
    pub log: Log,
}
