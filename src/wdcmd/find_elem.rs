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
