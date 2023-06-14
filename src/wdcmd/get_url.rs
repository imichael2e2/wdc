// Copyright (C) 2023 Michael Lee <imichael2e2@proton.me OR ...@gmail.com>
//
// Licensed under the MIT License <LICENSE-MIT or
// https://opensource.org/license/mit> or the GNU General Public License,
// Version 3.0 or any later version <LICENSE-GPL or
// https://www.gnu.org/licenses/gpl-3.0.txt>, at your option.
//
// This file may not be copied, modified, or distributed except except in
// compliance with either of the licenses.
//

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
