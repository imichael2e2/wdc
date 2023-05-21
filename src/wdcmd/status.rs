// Copyright (C) 2023  Michael Lee
//
// This file is part of Wdc.
//
// Wdc is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Wdc is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Wdc. If not, see <https://www.gnu.org/licenses/>.

///
/// Get details of “Status” command processing result.
pub trait DrvStatResultGetter {
    fn ready(&self) -> bool;
    fn msg(&self) -> &str;
}

///
/// The “Status” command processing result.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DrvStatResult {
    value: DrvStatus,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DrvStatus {
    ready: bool,
    message: String,
}

impl DrvStatResultGetter for DrvStatResult {
    fn ready(&self) -> bool {
        self.value.ready
    }
    fn msg(&self) -> &str {
        self.value.message.as_ref()
    }
}
