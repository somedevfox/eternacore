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
use super::{Request, Response};
use serenity::builder::CreateApplicationCommand;

/// Representation of both a **message** and an **interaction** command.
#[async_trait]
pub trait Command: Send + Sync + 'static {
    /// Name of the command which will be used when registering it.\
    /// Naming rules:
    ///  - Lowercase-only
    ///  - No spaces
    fn name<'s>(&self) -> &'s str;
    /// Description of the command which will be shown in slash command preview or `help` message command.
    fn description<'s>(&self) -> &'s str {
        ""
    }

    fn default_layout<'s>(
        &self,
        command: &'s mut CreateApplicationCommand,
    ) -> &'s mut CreateApplicationCommand {
        command.name(self.name()).description(self.description())
    }

    /// A layout of a command and definition of acceptable arguments.
    fn layout<'s>(
        &self,
        command: &'s mut CreateApplicationCommand,
    ) -> &'s mut CreateApplicationCommand {
        self.default_layout(command)
    }

    /// Asynchronous method which will be ran when user executes the command.\
    /// While [Response] structure is command model agnostic, you can retrieve specific model by using [`if let`](https://doc.rust-lang.org/book/ch06-03-if-let.html) syntax.
    async fn run(&self, req: Request, res: Response);
}
impl<T: Command + ?Sized> Register for T {
    fn register<'reg>(
        &'reg self,
        command: &'reg mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        self.layout(command)
    }
}
pub trait Register {
    fn register<'reg>(
        &'reg self,
        command: &'reg mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand;
}
