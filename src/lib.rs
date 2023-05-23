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
//! ```
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
//! ```
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

/// steal from dbg!
macro_rules! dbgg {
    () => {
        #[cfg(feature = "devel")]
        dbg!();
    };
    ($val:expr $(,)?) => {
        #[cfg(feature = "devel")]
        dbg!($val);
    };
    ($($val:expr),+ $(,)?) => {
        #[cfg(feature = "devel")]
        ($(dbg!($val)),+);
    };
}
macro_rules! dbgmsg {
    ($fmtstr:expr) => {
        #[cfg(feature = "devel")]
        let dbgmsg = format!($fmtstr);
        #[cfg(feature = "devel")]
        dbg!(dbgmsg);
    };
    ($fmtstr:expr, $($val:expr),+ $(,)?) => {
        #[cfg(feature = "devel")]
        let dbgmsg = format!($fmtstr, $($val),+);
        #[cfg(feature = "devel")]
        dbg!(dbgmsg);
    };
}
macro_rules! run_diag {
    ($phase:expr, $blk:block) => {
        #[cfg(feature = "diag")]
        let start = std::time::Instant::now();

        $blk;

        #[cfg(feature = "diag")]
        let dura = start.elapsed();
        #[cfg(feature = "diag")]
        let diag_msg = format!("{}: {:?}", $phase, dura);
        #[cfg(feature = "diag")]
        dbg!(diag_msg);
    };
}
pub(crate) use dbgg;
pub(crate) use dbgmsg;
pub(crate) use run_diag;

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
