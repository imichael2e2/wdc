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
/// [W3C](https://w3c.github.io/webdriver) standard mandatory behaviour.
mod w3c;

///
/// [GeckoDriver](https://chromedriver.chromium.org/home) related implementations.
mod gec;

///
/// [ChromeDriver](https://chromedriver.chromium.org/home) related implementations.
#[cfg_attr(not(feature = "chromium"), allow(dead_code))]
mod chr;

///
/// Common implementations shared across various vendors.
mod comm;

pub use w3c::{W3cCapRequSetter, W3cCapaGetter, W3cCapaSetter, W3cSessResultGetter};

#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
pub use gec::any::{FirefoxCapaGetter, FirefoxCapaSetter, GeckoCapRequSetter};
#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
pub use gec::{FirefoxCapa, GeckoCapRequ, GeckoSessResult};

#[cfg(feature = "chromium")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "chromium")))]
pub use chr::any::{ChromCapRequSetter, ChromiumCapaGetter, ChromiumCapaSetter};
#[cfg(feature = "chromium")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "chromium")))]
pub use chr::{ChromCapRequ, ChromSessResult, ChromiumCapa};

#[cfg(feature = "dev")]
pub use comm::{CommCapRequ, CommCapa, CommSessResult};
