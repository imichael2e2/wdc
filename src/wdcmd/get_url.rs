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
/// Get details of “Get Current Url” command processing result.
pub trait GetUrlResultGetter {
    fn url(&self) -> &str;
}

///
/// The “Get Current Url” command processing result.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetUrlResult {
    value: String,
}

impl GetUrlResultGetter for GetUrlResult {
    fn url(&self) -> &str {
        self.value.as_ref()
    }
}
