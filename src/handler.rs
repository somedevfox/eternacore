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
use crate::commands::{
    framework::{Request, Response},
    register, COMMANDS,
};
use serenity::{
    async_trait,
    model::application::interaction::Interaction,
    model::prelude::*,
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, _ready: Ready) {
        register(&context).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            for defined_command in COMMANDS.iter() {
                if command.data.name == defined_command.name().to_string() {
                    defined_command
                        .run(
                            Request::from_command_data_option_slice(
                                ctx.http.clone(),
                                command.channel_id,
                                &command.data,
                            ),
                            Response::from_interaction_command(ctx, command),
                        )
                        .await;
                    break;
                }
            }
        }
    }

    async fn message(&self, _context: Context, _new_message: Message) {}
}
