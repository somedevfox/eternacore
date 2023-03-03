use std::sync::Arc;

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
use serenity::{
    http::Http,
    model::prelude::{
        command::CommandType,
        interaction::application_command::{
            CommandData, CommandDataOption, CommandDataOptionValue,
        },
        ChannelId, GuildId,
    },
};

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    options: Vec<CommandDataOption>,
}
impl Command {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn options(&self) -> Vec<CommandDataOption> {
        self.options.clone()
    }
}

pub struct Request {
    command: Command,
    pub http: Arc<Http>,
    pub guild: Option<GuildId>,
    pub channel: ChannelId,
}
impl Request {
    pub fn from_command_data_option_slice(
        http: Arc<Http>,
        channel: ChannelId,
        command_data: &CommandData,
    ) -> Self {
        Self {
            command: Command {
                name: command_data.name.clone(),
                options: command_data.options.clone(),
            },
            http,
            guild: command_data.guild_id,
            channel,
        }
    }

    pub fn command(&self) -> Command {
        self.command.clone()
    }
}
