use std::sync::atomic::{AtomicBool, Ordering};

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
    http::HttpError,
    model::{
        channel::Message as SerenityMessage,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

pub enum Response {
    InteractionCommand {
        context: Context,
        command: ApplicationCommandInteraction,
    },
    Message {
        context: Context,
        message: SerenityMessage,
    },
}
impl Response {
    pub fn from_interaction_command(
        context: Context,
        interaction_command: ApplicationCommandInteraction,
    ) -> Self {
        Self::InteractionCommand {
            context,
            command: interaction_command,
        }
    }
    pub fn from_message(context: Context, message: SerenityMessage) -> Self {
        Self::Message { context, message }
    }

    pub async fn send_message(&self, message: impl Into<super::Message>) {
        self.send_ephemeral_message(message, false).await
    }

    pub async fn send_ephemeral_message(
        &self,
        message: impl Into<super::Message>,
        ephemeral: bool,
    ) {
        let message = message.into();
        match self {
            Response::InteractionCommand { context, command } => {
                if let Err(why) = command
                    .create_interaction_response(context, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|data| {
                                data.content(message.get_content()).ephemeral(ephemeral)
                            })
                    })
                    .await
                {
                    if let serenity::Error::Http(http_error) = why {
                        if let HttpError::UnsuccessfulRequest(http_error_response) = *http_error {
                            if http_error_response.error.code == 40060 {
                                // interaction has already been acknowledged
                                if let Err(why) = command
                                    .create_followup_message(context, |followup| {
                                        followup.content(message.get_content())
                                    })
                                    .await
                                {
                                    error!(
                                        "Couldn't make a follow-up message to interaction command `{}`: {}",
                                        command.id,
                                        why.to_string()
                                    );
                                }
                            } else {
                                error!(
                                    "Couldn't respond to an interaction command `{}`: {}",
                                    command.id, http_error_response.error.message
                                );
                            }
                        }
                    }
                }
            }
            Response::Message {
                context: _,
                message: _,
            } => {
                // TODO:
            }
        }
    }

    /// Checks if the issued command is an interaction.
    pub fn is_interaction_command(&self) -> bool {
        if let Self::InteractionCommand {
            context: _,
            command: _,
        } = self
        {
            true
        } else {
            false
        }
    }
    /// Checks if the command was issued from an ordinary message.
    pub fn is_message_command(&self) -> bool {
        !self.is_interaction_command()
    }

    pub fn to_application_command_interaction(
        self,
    ) -> Option<(Context, ApplicationCommandInteraction)> {
        if let Self::InteractionCommand { context, command } = self {
            Some((context, command))
        } else {
            None
        }
    }
    pub fn to_message(self) -> Option<(Context, SerenityMessage)> {
        if let Self::Message { context, message } = self {
            Some((context, message))
        } else {
            None
        }
    }
}
