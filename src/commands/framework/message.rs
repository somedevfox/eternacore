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
pub struct Message {
    content: String,
}
impl Message {
    pub fn new(content: impl ToString) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    pub fn content(mut self, new_content: impl ToString) -> Self {
        self.content = new_content.to_string();
        self
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl<'msg> Into<Message> for &'msg str {
    fn into(self) -> Message {
        Message::new(self)
    }
}
impl Into<Message> for String {
    fn into(self) -> Message {
        Message::new(self)
    }
}
