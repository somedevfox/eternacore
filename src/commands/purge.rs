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
use super::framework::{Command, Request, Response};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
    },
};

pub struct Purge;

#[async_trait]
impl Command for Purge {
    fn name<'s>(&self) -> &'s str {
        "purge"
    }
    fn description<'s>(&self) -> &'s str {
        "Bulk delete up to 100 messages"
    }

    fn layout<'s>(
        &self,
        command: &'s mut CreateApplicationCommand,
    ) -> &'s mut CreateApplicationCommand {
        self.default_layout(command).create_option(|option| {
            option
                .name("number")
                .description("Number of messages to delete")
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .max_int_value(100)
                .required(true)
        })
    }

    async fn run(&self, req: Request, res: Response) {
        let command = req.command();
        let num_to_delete = *if let CommandDataOptionValue::Integer(int) =
            command.options()[0].resolved.as_ref().unwrap()
        {
            int
        } else {
            unreachable!()
        };

        let messages = match req
            .channel
            .messages(req.http.clone(), |get_messages| {
                get_messages.limit(num_to_delete as u64)
            })
            .await
        {
            Ok(m_ids) => m_ids,
            Err(_) => {
                res.send_ephemeral_message("Cannot view current channel!", true)
                    .await;
                return;
            }
        };
        if messages.is_empty() {
            res.send_ephemeral_message("Nothing left to delete.", true)
                .await;
            return;
        }

        res.send_ephemeral_message(if let Err(_) = req.channel.delete_messages(req.http, messages).await {
            String::from("I don't have enough permissions to bulk delete messages, please, enable `Manage Messages` permission and execute the command again.")
        } else {
			format!("{} messages deleted successfully!", num_to_delete)
		}, true).await;
    }
}
