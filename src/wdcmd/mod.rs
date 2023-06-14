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
