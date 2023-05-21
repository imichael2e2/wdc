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

//!
//! The module dedicated to WebDriver commands.
//!
//! Most WebDriver nodes interact with others through
//! WebDriver [command](https://w3c.github.io/webdriver/#commands)
//! in an active [session](https://w3c.github.io/webdriver/#commands).
//! This module encompasses the essential supports for
//! standard-mandated commands, such as Capability Pre-processing, Session
//! Customization, Command De/Serialization, etc.
//!
//! Wdcmd leverages [Serde](https://serde.rs) as de/serialization framework,
//! which allows for zero-copy
//! deserialization, ensuring a low memory footprint and latency as long as
//! being used properly.
//!
//! Wdcmd does not assume the node's type, as
//! [defined](https://w3c.github.io/webdriver/#nodes) by the W3C standard,
//! making it ready for any automation task with arbitrary node
//! orchestration. For instance, one client with one proxy and several servers.

///
/// Webdriver "Error Report" command.
pub mod err;

///
/// Webdriver "New Session" command.
pub mod session;

///
/// Webdriver "Status" command.
pub mod status;

///
/// Webdriver "Navigate To" command.
pub mod get_url;

///
/// Webdriver "Find Element(s)" command.
pub mod find_elem;

///
/// Webdriver "Perform Actions" command.
pub mod actions;
