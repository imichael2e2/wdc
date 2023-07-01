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

#![cfg_attr(doc_cfg, feature(doc_cfg))]
// #![feature(doc_auto_cfg)]

//!
//! A **W**eb**D**river **C**lient library.
//!
//! # Overview
//!
//! WebDriver is a widely-adopted W3C standard for web browser automation. It
//! defines algorithms in a platform-, language-independent manner. It
//! opens the possibility of using system programming languages such as Rust
//! or C/C++ for WebUI testing/browser automation, which leads to better
//! performance, efficiency, and safety.
//!
//! This crate provides a pure Rust implementation of
//! the client-side WebDriver, with the following goals:
//!
//! 1. **Standard Compliance**: it tracks both classical WebDriver and modern
//! BiDi standards.
//!
//! 2. **Low Overhead**: it has no runtime dependencies.
//!
//! 3. **Excellent Performance**: it strives for zero-copy operations.
//!
//! # Examples:
//! _Note: Assume using GeckoDriver, with default settings._
//!
//!
//! ## Navigate to website:
//!
//! ```ignore
//! use wdc::{GeckoDriver, WdcError, WebDrvClient};
//!
//! go_w3c().unwrap();
//!
//! fn go_w3c() -> Result<(), WdcError> {
//!    let wdc: WebDrvClient<GeckoDriver> = wdc::init("127.0.0.1", 4444, 10)?;
//!    let url = "https://www.w3.org/standards";
//!
//!    wdc.navi_to(url)?;
//!    // ...whatever tests/automation on "w3.org"
//!
//!    Ok(())
//! }
//! ```
//!
//! ## Run Javascript on website:
//!
//! ```ignore
//! use wdc::{GeckoDriver, WdcError, WebDrvClient};
//!
//! check_out_w3c_history().unwrap();
//!
//! fn check_out_w3c_history() -> Result<(), WdcError> {
//!    let wdc: WebDrvClient<GeckoDriver> = wdc::init("127.0.0.1", 4444, 10)?;
//!    let url = "https://www.w3.org/Consortium/facts.html";
//!
//!    let js_result = wdc.navi_to(url)?.exec_sync(
//!        "return document.getElementById('history').nextElementSibling.innerText;",
//!        vec![],
//!    )?;
//!
//!    let w3c_history = String::from_utf8_lossy(&js_result);
//!    assert!(w3c_history.contains("Tim Berners-Lee"));
//!    assert!(w3c_history.contains("World Wide Web"));
//!    assert!(w3c_history.contains("HTML"));
//!
//!    Ok(())
//! }
//! ```

#[macro_use]
mod private_dbg;

#[cfg(feature = "bidi")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "bidi")))]
#[allow(clippy::all)]
pub mod bidi;

#[allow(clippy::len_zero, clippy::manual_map, clippy::redundant_closure)]
pub mod wdcmd;

#[allow(clippy::len_zero, clippy::identity_op)]
mod httpp;

#[cfg(feature = "bidi")]
#[allow(clippy::len_zero, clippy::identity_op, clippy::needless_range_loop)]
mod wsp;

// STRUCT //

// WdcError //

///
/// The WebDriver client-specific errors.
#[derive(PartialEq, Debug)]
pub enum WdcError {
    ///
    /// A possible bug found.
    Buggy,
    BusyCreateSession,
    DriverNotReadyBusySession,
    NotReadyForNewSession,
    ///
    /// The operation is not supported by corresponding WebDriver client.
    UnsupportedOperation,
    ///
    /// The WebDriver server is not ready for command processing.
    WebDriverNotReady,
    ///
    /// The connection to WebDriver server cannot be established.
    WebDriverRemoteConnectionFailed,
    ///
    /// The command cannot be processed successfully by WebDriver server.
    ///
    /// The first field corresponds to the "error" field of standard WebDriver
    /// error response, the second corresponds to the "message" field.
    BadDrvCmd(String, String),
}

#[allow(clippy::len_zero)]
mod genericdrv;

#[cfg(feature = "firefox")]
mod geckodrv;

#[cfg(feature = "chromium")]
mod chromedrv;

// Generic //

pub use genericdrv::init;
pub use genericdrv::init_singl;
pub use genericdrv::CreateW3cSession;
pub use genericdrv::CreateWebDrvClient;
pub use genericdrv::RendVendor;
pub use genericdrv::SessionMeta;
pub use genericdrv::WebDrvClient;

#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
pub use geckodrv::init_singl_ff;
#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
pub use geckodrv::GeckoDriver;

#[cfg(feature = "chromium")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "chromium")))]
pub use chromedrv::init_singl_ch;
#[cfg(feature = "chromium")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "chromium")))]
pub use chromedrv::ChromeDriver;
