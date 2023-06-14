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
/// Setters for standard-compliance capabilities request.
///
/// This is the request for which any standard-compliance WebDriver session
/// would ask.
pub trait W3cCapRequSetter<'c1, 'c2> {
    fn mandate_as_w3c(&mut self, other: &'c1 impl W3cCapaGetter);
    fn allow_as_w3c(&mut self, other: &'c2 impl W3cCapaGetter) -> &mut Self;
}

///
/// Getters for standard-compliance capabilities.
///
/// Note that each capability has a specicial property of _significance_,
/// by which the maximum flexibility of seriazlization can achieve.
pub trait W3cCapaGetter {
    // getter
    fn browser_name(&self) -> Option<&str> {
        None
    }
    fn browser_version(&self) -> Option<&str> {
        None
    }
    fn platform_name(&self) -> Option<&str> {
        None
    }
    fn accept_insecure_certs(&self) -> Option<bool> {
        None
    }
    fn page_load_strategy(&self) -> Option<&str> {
        None
    }
    //
    fn proxy_type(&self) -> Option<&str> {
        None
    }
    fn proxy_autoconfig_url(&self) -> Option<&str> {
        None
    }
    fn ftp_proxy(&self) -> Option<&str> {
        None
    }
    fn http_proxy(&self) -> Option<&str> {
        None
    }
    fn no_proxy(&self) -> Option<Vec<&str>> {
        None
    }
    fn ssl_proxy(&self) -> Option<&str> {
        None
    }
    fn socks_proxy(&self) -> Option<&str> {
        None
    }
    fn socks_version(&self) -> Option<u8> {
        None
    }
    //
    fn window_rect(&self) -> Option<bool> {
        None
    }
    //
    fn timeouts_script(&self) -> Option<u32> {
        None
    }
    fn timeouts_page_load(&self) -> Option<u32> {
        None
    }
    fn timeouts_implicit(&self) -> Option<u32> {
        None
    }
    //
    fn strict_file_interactability(&self) -> Option<bool> {
        None
    }
    fn unhandled_prompt_behavior(&self) -> Option<&str> {
        None
    }
    fn wsurl(&self) -> Option<&str> {
        None
    }

    ///
    /// Check whether all fields are insignificant, i.e. no need to
    /// be serialized.
    fn is_insig_as_w3c(&self) -> bool;
}

///
/// Setters for standard-compliance capabilities.
///
/// These are the capabilities that any standard-compliance WebDriver session
/// would have. They have effect on eventual new session. By this, user can tune
/// the behavior the WebDriver has.
///
/// There are totally three types of "capabilities", as per W3C standard:
/// 1. [Standard Capabilities](https://w3c.github.io/webdriver/#dfn-table-of-standard-capabilities)
/// 2. [Extension Capabilities](https://w3c.github.io/webdriver/#dfn-extension-capability)
/// 3. [Additional Capabilities](https://w3c.github.io/webdriver/#dfn-additional-webdriver-capability)
///
/// This specifies the __standard__ ones.
pub trait W3cCapaSetter<'c> {
    ///
    /// Set a browser name.
    fn set_browser_name(&mut self, arg: &'c str);
    fn set_browser_name_take(&mut self, arg: String);
    fn set_browser_name_owned(&mut self, arg: &str);
    ///
    /// Set a browser name, copy needed.
    fn set_browser_version(&mut self, arg: &'c str);
    fn set_browser_version_take(&mut self, arg: String);
    fn set_browser_version_owned(&mut self, arg: &str);
    fn set_platform_name(&mut self, arg: &'c str);
    fn set_platform_name_take(&mut self, arg: String);
    fn set_platform_name_owned(&mut self, arg: &str);
    fn set_accept_insecure_certs(&mut self, arg: bool);
    fn set_page_load_strategy(&mut self, arg: &'c str);
    fn set_page_load_strategy_take(&mut self, arg: String);
    fn set_page_load_strategy_owned(&mut self, arg: &str);
    //
    fn set_proxy_type(&mut self, arg: &'c str);
    fn set_proxy_autoconfig_url(&mut self, arg: &'c str);
    fn set_ftp_proxy(&mut self, arg: &'c str);
    fn set_http_proxy(&mut self, arg: &'c str);
    fn set_no_proxy(&mut self, arg: Vec<&'c str>);
    fn add_no_proxy(&mut self, arg: &'c str);
    fn set_ssl_proxy(&mut self, arg: &'c str);
    fn set_socks_proxy(&mut self, arg: &'c str);
    fn set_socks_proxy_owned(&mut self, arg: &str);
    fn set_socks_proxy_take(&mut self, arg: String);
    fn set_socks_version(&mut self, arg: u8);
    //
    fn set_window_rect(&mut self, arg: bool);
    //
    ///
    /// Set timeout for script evaluation, in milliseconds.
    ///
    /// The script evaluation is typically fired by _ExecuteSync_
    /// or _ExecuteAsync_ command.
    fn set_timeouts_script(&mut self, arg: u32);
    ///
    /// Set timeout for page loading, in milliseconds.
    fn set_timeouts_page_load(&mut self, arg: u32);
    ///
    /// Set timeout for ???, in milliseconds.
    fn set_timeouts_implicit(&mut self, arg: u32);
    //
    fn set_strict_file_interactability(&mut self, arg: bool);
    fn set_unhandled_prompt_behavior(&mut self, arg: &'c str);
    fn set_unhandled_prompt_behavior_take(&mut self, arg: String);
    fn set_unhandled_prompt_behavior_owned(&mut self, arg: &str);
    fn set_wsurl(&mut self, arg: &'c str);
    fn set_wsurl_take(&mut self, arg: String);
    fn enable_bidi(&mut self);
}

///
/// Getters for standard-compliance session result.
///
/// This is result that any standard-compliance WebDriver server would send back
/// to client after creating a session.
pub trait W3cSessResultGetter {
    ///
    /// The [session ID](https://w3c.github.io/webdriver/#dfn-session-id)
    /// of binding session.
    fn session_id(&self) -> &str;
    fn wsurl(&self) -> Option<&str>;
}
