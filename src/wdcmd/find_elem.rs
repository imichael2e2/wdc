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

#[derive(Debug, serde::Deserialize)]
struct WebElem {
    #[serde(rename = "element-6066-11e4-a52e-4f735466cecf")]
    webid: String,
}

///
/// The filter to find elements.
#[derive(Default, serde::Serialize, Debug)]
pub struct FindElemFilter {
    using: String,
    value: String,
}

///
/// Customize element filter.
pub trait FindElemFilterSetter {
    fn set_using(&mut self, arg: &str);
    fn set_value(&mut self, arg: &str);
}

///
/// Get details of "Find Element" command processing result.
pub trait FindElemResultGetter {
    fn eleid(&self) -> &str;
}

///
/// Get details of "Find Element" command processing result.
pub trait FindElemsResultGetter {
    fn eleids(&self) -> Vec<&str>;
}

///
/// The "Find Element" command processing result.
#[derive(Debug, serde::Deserialize)]
pub struct FindElemResult {
    value: WebElem,
}

///
/// The "Find Elements" command processing result.
#[derive(Debug, serde::Deserialize)]
pub struct FindElemsResult {
    value: Vec<WebElem>,
}

impl FindElemFilterSetter for FindElemFilter {
    fn set_using(&mut self, arg: &str) {
        self.using = arg.to_string();
    }
    fn set_value(&mut self, arg: &str) {
        self.value = arg.to_string();
    }
}

impl FindElemResultGetter for FindElemResult {
    fn eleid(&self) -> &str {
        self.value.webid.as_ref()
    }
}

impl FindElemsResultGetter for FindElemsResult {
    fn eleids(&self) -> Vec<&str> {
        self.value.iter().map(|x| x.webid.as_ref()).collect()
    }
}
