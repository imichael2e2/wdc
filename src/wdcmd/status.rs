// Copyright (C) 2023 Michael Lee <imichael2e2@proton.me/...@gmail.com>
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
