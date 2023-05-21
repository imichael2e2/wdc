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
pub use gec::any::{FirefoxCapaGetter, FirefoxCapaSetter, GeckoCapRequSetter};
#[cfg(feature = "firefox")]
pub use gec::{FirefoxCapa, GeckoCapRequ, GeckoSessResult};

#[cfg(feature = "chromium")]
pub use chr::any::{ChromCapRequSetter, ChromiumCapaGetter, ChromiumCapaSetter};
#[cfg(feature = "chromium")]
pub use chr::{ChromCapRequ, ChromSessResult, ChromiumCapa};

#[cfg(feature = "devel")]
pub use comm::{CommCapRequ, CommCapa, CommSessResult};
