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
use super::{framework::Request, Command, Response};
use serenity::async_trait;

pub struct Ping;
#[async_trait]
impl Command for Ping {
    fn name<'s>(&self) -> &'s str {
        "ping"
    }
    fn description<'s>(&self) -> &'s str {
        "Ping? Pong!"
    }

    async fn run(&self, _req: Request, res: Response) {
        res.send_ephemeral_message("Pong!", true).await;
    }
}
