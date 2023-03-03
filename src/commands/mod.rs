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
use framework::{command::Register, Command, Response};
use once_cell::sync::Lazy;
use serenity::{
    model::prelude::application::command::Command as SerenityCommand, prelude::Context,
};

pub mod framework;

//? Commands go here!
pub mod ping;
pub mod purge;

pub static COMMANDS: Lazy<Vec<Box<dyn Command>>> = Lazy::new(|| {
    let mut vector: Vec<Box<dyn Command>> = Vec::new();

    //? Commands go here
    vector.push(Box::new(ping::Ping));
    vector.push(Box::new(purge::Purge));

    vector
});

pub async fn register(context: &Context) {
    let _ = SerenityCommand::create_global_application_command(context.http.clone(), |commands| {
        for command in COMMANDS.iter() {
            command.register(commands);
        }
        commands
    })
    .await;
}
