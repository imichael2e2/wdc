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

pub mod any {
    use super::super::w3c;
    use super::any as gecko;

    use std::collections::BTreeMap;

    ///
    /// Setters for GeckoDriver-specific capabilities request.
    pub trait GeckoCapRequSetter<'c1, 'c2> {
        fn mandate(&mut self, other: &'c1 (impl gecko::FirefoxCapaGetter + w3c::W3cCapaGetter));
        fn allow(
            &mut self,
            other: &'c2 (impl gecko::FirefoxCapaGetter + w3c::W3cCapaGetter),
        ) -> &mut Self;
    }

    ///
    /// Getters for Firefox-specific capabilities.
    pub trait FirefoxCapaGetter {
        fn binary(&self) -> Option<&str> {
            None
        }
        fn args(&self) -> Option<Vec<&str>> {
            None
        }
        fn profile(&self) -> Option<&str> {
            None
        }
        fn prefs(&self) -> Option<BTreeMap<&str, &str>> {
            None
        }
        fn android_package(&self) -> Option<&str> {
            None
        }
        fn is_insig(&self) -> bool;
    }

    ///
    /// Setters for Firefox-specific capabilities.
    pub trait FirefoxCapaSetter<'c> {
        // setter
        fn set_binary(&mut self, _arg: &'c str) {}
        fn set_args(&mut self, _arg: Vec<&'c str>) {}
        fn add_args(&mut self, _arg: &'c str) {}
        fn set_profile(&mut self, _arg: &'c str) {}
        fn set_profile_take(&mut self, _arg: String) {}
        fn set_prefs(&mut self, _arg: BTreeMap<&'c str, &'c str>) {}
        fn add_prefs(&mut self, _key: &'c str, _value: &'c str) {}
        fn set_android_package(&mut self, _arg: &'c str) {}
    }
} // any

//  following impl are tested on v0.30  //

use super::comm;
use super::w3c;
use any as gecko;

use comm::VarValTypeMap;
use gecko::{FirefoxCapaGetter, FirefoxCapaSetter};
use w3c::{W3cCapaGetter, W3cCapaSetter};

use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct FirefoxSesExtCap<'c> {
    binary: Option<Cow<'c, str>>,
    args: Option<Vec<Cow<'c, str>>>,
    profile: Option<Cow<'c, str>>,
    prefs: Option<VarValTypeMap<Cow<'c, str>, Cow<'c, str>>>,
    // android_package: Option<String>, // FIXME: finish related impl
}

#[derive(Default, Debug)]
pub struct FirefoxSesAddCap {}

// FirefoxCapa //

///
/// The Firefox-specific session capabilities.
pub type FirefoxCapa<'c> = comm::CommCapa<'c, FirefoxSesExtCap<'c>, FirefoxSesAddCap>;

impl<'r, 'c1, 'c2> FirefoxCapa<'r>
where
    'c1: 'r,
    'c2: 'r,
{
    fn from_other_as_w3c(other: &'c1 impl w3c::W3cCapaGetter) -> Self {
        let mut newme = Self::default();

        // w3c
        if let Some(v) = other.browser_name() {
            newme.set_browser_name(v);
        }
        if let Some(v) = other.browser_version() {
            newme.set_browser_version(v);
        }
        if let Some(v) = other.platform_name() {
            newme.set_platform_name(v);
        }
        if let Some(v) = other.accept_insecure_certs() {
            newme.set_accept_insecure_certs(v);
        }
        if let Some(v) = other.page_load_strategy() {
            newme.set_page_load_strategy(v);
        }
        if let Some(v) = other.proxy_type() {
            newme.set_proxy_type(v);
        }
        if let Some(v) = other.proxy_autoconfig_url() {
            newme.set_proxy_autoconfig_url(v);
        }
        if let Some(v) = other.ftp_proxy() {
            newme.set_ftp_proxy(v);
        }
        if let Some(v) = other.http_proxy() {
            newme.set_http_proxy(v);
        }
        if let Some(v) = other.no_proxy() {
            newme.set_no_proxy(v);
        }
        if let Some(v) = other.ssl_proxy() {
            newme.set_ssl_proxy(v);
        }
        if let Some(v) = other.socks_proxy() {
            newme.set_socks_proxy(v);
        }
        if let Some(v) = other.socks_version() {
            newme.set_socks_version(v);
        }
        if let Some(v) = other.window_rect() {
            newme.set_window_rect(v);
        }
        if let Some(v) = other.timeouts_script() {
            newme.set_timeouts_script(v);
        }
        if let Some(v) = other.timeouts_page_load() {
            newme.set_timeouts_page_load(v);
        }
        if let Some(v) = other.timeouts_implicit() {
            newme.set_timeouts_implicit(v);
        }
        if let Some(v) = other.strict_file_interactability() {
            newme.set_strict_file_interactability(v);
        }
        if let Some(v) = other.unhandled_prompt_behavior() {
            newme.set_unhandled_prompt_behavior(v);
        }

        if let Some(v) = other.wsurl() {
            newme.set_wsurl(v);
        }

        newme
    }

    fn from_other(other: &'c2 (impl w3c::W3cCapaGetter + gecko::FirefoxCapaGetter)) -> Self {
        // note that self likey impl FirefoxCapaGetter
        let mut newme = Self::default();

        // w3c
        if let Some(v) = other.browser_name() {
            newme.set_browser_name(v);
        }
        if let Some(v) = other.browser_version() {
            newme.set_browser_version(v);
        }
        if let Some(v) = other.platform_name() {
            newme.set_platform_name(v);
        }
        if let Some(v) = other.accept_insecure_certs() {
            newme.set_accept_insecure_certs(v);
        }
        if let Some(v) = other.page_load_strategy() {
            newme.set_page_load_strategy(v);
        }
        if let Some(v) = other.proxy_type() {
            newme.set_proxy_type(v);
        }
        if let Some(v) = other.proxy_autoconfig_url() {
            newme.set_proxy_autoconfig_url(v);
        }
        if let Some(v) = other.ftp_proxy() {
            newme.set_ftp_proxy(v);
        }
        if let Some(v) = other.http_proxy() {
            newme.set_http_proxy(v);
        }
        if let Some(v) = other.no_proxy() {
            newme.set_no_proxy(v);
        }
        if let Some(v) = other.ssl_proxy() {
            newme.set_ssl_proxy(v);
        }
        if let Some(v) = other.socks_proxy() {
            newme.set_socks_proxy(v);
        }
        if let Some(v) = other.socks_version() {
            newme.set_socks_version(v);
        }
        if let Some(v) = other.window_rect() {
            newme.set_window_rect(v);
        }
        if let Some(v) = other.timeouts_script() {
            newme.set_timeouts_script(v);
        }
        if let Some(v) = other.timeouts_page_load() {
            newme.set_timeouts_page_load(v);
        }
        if let Some(v) = other.timeouts_implicit() {
            newme.set_timeouts_implicit(v);
        }
        if let Some(v) = other.strict_file_interactability() {
            newme.set_strict_file_interactability(v);
        }
        if let Some(v) = other.unhandled_prompt_behavior() {
            newme.set_unhandled_prompt_behavior(v);
        }
        if let Some(v) = other.wsurl() {
            newme.set_wsurl(v);
        }

        // gecko
        if let Some(v) = other.binary() {
            newme.set_binary(v);
        }
        if let Some(v) = other.args() {
            newme.set_args(v);
        }
        if let Some(v) = other.profile() {
            newme.set_profile(v);
        }
        if let Some(v) = other.prefs() {
            newme.set_prefs(v);
        }

        newme
    }

    fn is_conflict_with(
        &self,
        other: &(impl w3c::W3cCapaGetter + gecko::FirefoxCapaGetter),
    ) -> bool {
        (self.browser_name().is_some() && other.browser_name().is_some())
            || (self.browser_version().is_some() && other.browser_version().is_some())
            || (self.platform_name().is_some() && other.platform_name().is_some())
            || (self.accept_insecure_certs().is_some() && other.accept_insecure_certs().is_some())
            || (self.page_load_strategy().is_some() && other.page_load_strategy().is_some())
            || (self.proxy_type().is_some() && other.proxy_type().is_some())
            || (self.window_rect().is_some() && other.window_rect().is_some())
            || (self.timeouts_script().is_some() && other.timeouts_script().is_some())
            || (self.strict_file_interactability().is_some()
                && other.strict_file_interactability().is_some())
            || (self.unhandled_prompt_behavior().is_some()
                && other.unhandled_prompt_behavior().is_some())
            || (self.binary().is_some() && other.binary().is_some())
            || (self.args().is_some() && other.args().is_some())
            || (self.profile().is_some() && other.profile().is_some())
    }

    fn is_conflict_with_as_w3c(&self, other: &impl w3c::W3cCapaGetter) -> bool {
        (self.browser_name().is_some() && other.browser_name().is_some())
            || (self.browser_version().is_some() && other.browser_version().is_some())
            || (self.platform_name().is_some() && other.platform_name().is_some())
            || (self.accept_insecure_certs().is_some() && other.accept_insecure_certs().is_some())
            || (self.page_load_strategy().is_some() && other.page_load_strategy().is_some())
            || (self.proxy_type().is_some() && other.proxy_type().is_some())
            || (self.window_rect().is_some() && other.window_rect().is_some())
            || (self.timeouts_script().is_some() && other.timeouts_script().is_some())
            || (self.strict_file_interactability().is_some()
                && other.strict_file_interactability().is_some())
            || (self.unhandled_prompt_behavior().is_some()
                && other.unhandled_prompt_behavior().is_some())
    }
}

impl gecko::FirefoxCapaGetter for FirefoxCapa<'_> {
    fn binary(&self) -> Option<&str> {
        match self.ext.binary.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn args(&self) -> Option<Vec<&str>> {
        self.ext
            .args
            .as_ref()
            .map(|v| v.iter().map(|x| x.as_ref()).collect())
    }
    fn profile(&self) -> Option<&str> {
        match self.ext.profile.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn prefs(&self) -> Option<BTreeMap<&str, &str>> {
        let prefs = self.ext.prefs.as_ref();

        match prefs {
            None => None,
            Some(v) => Some(
                v.map
                    .iter()
                    .map(|(k, v)| (k.as_ref(), v.as_ref()))
                    .collect(),
            ),
        }
    }
    fn is_insig(&self) -> bool {
        self.binary().is_none()
            && self.args().is_none()
            && self.profile().is_none()
            && self.prefs().is_none()
            && self.android_package().is_none()
    }
}

impl<'r, 'c> gecko::FirefoxCapaSetter<'c> for FirefoxCapa<'r>
where
    'c: 'r,
{
    // setter
    fn set_binary(&mut self, arg: &'c str) {
        self.ext.binary = Some(Cow::from(arg));
    }
    fn set_args(&mut self, arg: Vec<&'c str>) {
        self.ext.args = Some(arg.iter().map(|x| Cow::from(*x)).collect());
    }
    fn add_args(&mut self, arg: &'c str) {
        match self.ext.args.as_mut() {
            Some(v) => v.push(Cow::from(arg)),
            None => self.set_args(vec![arg]),
        }
    }
    fn set_profile(&mut self, arg: &'c str) {
        self.ext.binary = Some(Cow::from(arg));
    }
    fn set_profile_take(&mut self, arg: String) {
        self.ext.binary = Some(Cow::from(arg));
    }
    fn set_prefs(&mut self, arg: BTreeMap<&'c str, &'c str>) {
        let mut newone = VarValTypeMap::<Cow<'c, str>, Cow<'c, str>>::default();
        for (key, value) in arg.iter() {
            newone.map.insert(Cow::from(*key), Cow::from(*value));
        }
        self.ext.prefs = Some(newone);
    }
    fn add_prefs(&mut self, key: &'c str, value: &'c str) {
        match self.ext.prefs.as_mut() {
            Some(v) => {
                v.map.insert(Cow::from(key), Cow::from(value));
            }
            None => {
                let mut newone = BTreeMap::<&'c str, &'c str>::default();
                newone.insert(key, value);
                self.set_prefs(newone);
            }
        }
    }
}

// GeckoCapRequ //

///
/// The GeckoDriver-specific session capabilities request.
pub type GeckoCapRequ<'c> = comm::CommCapRequ<FirefoxCapa<'c>>;

impl<'r, 'c1, 'c2> w3c::W3cCapRequSetter<'c1, 'c2> for GeckoCapRequ<'r>
where
    'c1: 'r,
    'c2: 'r,
{
    fn mandate_as_w3c(&mut self, other: &'c1 impl w3c::W3cCapaGetter) {
        let newone = FirefoxCapa::from_other_as_w3c(other);

        self.always_match = newone;

        self.first_match
            .retain(|x| !self.always_match.is_conflict_with_as_w3c(x));
    }

    fn allow_as_w3c(&mut self, other: &'c2 impl w3c::W3cCapaGetter) -> &mut Self {
        if !self.always_match.is_conflict_with_as_w3c(other) {
            let newone = FirefoxCapa::from_other_as_w3c(other);
            self.first_match.push(newone);
        }
        self
    }
}

impl<'r, 'c1, 'c2> gecko::GeckoCapRequSetter<'c1, 'c2> for GeckoCapRequ<'r>
where
    'c1: 'r,
    'c2: 'r,
{
    fn mandate(&mut self, other: &'c1 (impl w3c::W3cCapaGetter + gecko::FirefoxCapaGetter)) {
        let newone = FirefoxCapa::from_other(other);

        self.always_match = newone;

        self.first_match
            .retain(|x| !self.always_match.is_conflict_with(x));
    }

    fn allow(
        &mut self,
        other: &'c2 (impl w3c::W3cCapaGetter + gecko::FirefoxCapaGetter),
    ) -> &mut Self {
        if !self.always_match.is_conflict_with(other) {
            let newone = FirefoxCapa::from_other(other);
            self.first_match.push(newone);
        }
        self
    }
}

// GeckoSessResult //

///
/// The GeckoDriver-specific session result.
pub type GeckoSessResult<'c> = comm::CommSessResult<FirefoxCapa<'c>>;

mod ser {
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    const INSIG_SNAME: &str = "-";
    const INSIG_SFLEN: usize = 1;

    use super::*;

    impl Serialize for GeckoCapRequ<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            state.serialize_field("alwaysMatch", &self.always_match)?;
            state.serialize_field("firstMatch", &self.first_match)?;

            state.end()
        }
    }

    impl Serialize for FirefoxCapa<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?; // FIXME: prefer accurate len

            if let Some(v) = self.browser_name() {
                state.serialize_field("browserName", &v)?;
            }
            if let Some(v) = self.browser_version() {
                state.serialize_field("browserVersion", &v)?;
            }
            if let Some(v) = self.platform_name() {
                state.serialize_field("platformName", &v)?;
            }
            if let Some(v) = self.accept_insecure_certs() {
                state.serialize_field("acceptInsecureCerts", &v)?;
            }
            if let Some(v) = self.strict_file_interactability() {
                state.serialize_field("strictFileInteractability", &v)?;
            }
            if let Some(v) = &self.proxy {
                state.serialize_field("proxy", v)?;
            }
            if let Some(v) = &self.timeouts {
                state.serialize_field("timeouts", v)?;
            }
            if self.wsurl.is_some() {
                state.serialize_field("webSocketUrl", &true)?;
            }
            if !self.is_insig() {
                state.serialize_field("moz:firefoxOptions", &self.ext)?;
            }

            state.end()
        }
    }

    impl Serialize for FirefoxSesExtCap<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            if let Some(v) = &self.args {
                state.serialize_field("args", &v)?;
            }
            if let Some(v) = &self.binary {
                state.serialize_field("binary", &v)?;
            }
            if let Some(v) = &self.profile {
                state.serialize_field("profile", &v)?;
            }
            if let Some(v) = &self.prefs {
                state.serialize_field("prefs", &v)?;
            }

            state.end()
        }
    }
} // ser

mod deser {
    use super::*;
    use serde::de::{Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor};

    #[derive(Debug)]
    enum Fields {
        BrowserName,
        BrowserVersion,
        PlatformName,
        AcceptInsecureCerts,
        PageLoadStrategy,
        Proxy,
        WindowRect,
        Timeouts,
        StrictFileInteractability,
        UnhandledPromptBehavior,
        WebSocketUrl,
        //
        MozAccessibilityChecks,
        MozBuildID,
        MozGeckodriverVersion,
        MozHeadless,
        MozProcessID,
        MozProfile,
        MozShutdownTimeout,
        MozUseNonSpecCompliantPointerOrigin,
        MozWebdriverClick,
        MozWindowless,
        Reserved(String),
    }

    const FIELD_JSON_NAMES: &[&str] = &[
        "browserName",
        "browserVersion",
        "platformName",
        "acceptInsecureCerts",
        "pageLoadStrategy",
        "proxy",
        "setWindowRect",
        "timeouts",
        "strictFileInteractability",
        "unhandledPromptBehavior",
        //
        "moz:accessibilityChecks",
        "moz:buildID",
        "moz:geckodriverVersion",
        "moz:headless",
        "moz:processID",
        "moz:profile",
        "moz:shutdownTimeout",
        "moz:useNonSpecCompliantPointerOrigin",
        "moz:webdriverClick",
        "moz:windowless",
        //
        "webSocketUrl", // FIXME: should not be here
    ];

    impl Fields {
        fn from_str(s: &str) -> Self {
            if s == FIELD_JSON_NAMES[0] {
                Fields::BrowserName
            } else if s == FIELD_JSON_NAMES[1] {
                Fields::BrowserVersion
            } else if s == FIELD_JSON_NAMES[2] {
                Fields::PlatformName
            } else if s == FIELD_JSON_NAMES[3] {
                Fields::AcceptInsecureCerts
            } else if s == FIELD_JSON_NAMES[4] {
                Fields::PageLoadStrategy
            } else if s == FIELD_JSON_NAMES[5] {
                Fields::Proxy
            } else if s == FIELD_JSON_NAMES[6] {
                Fields::WindowRect
            } else if s == FIELD_JSON_NAMES[7] {
                Fields::Timeouts
            } else if s == FIELD_JSON_NAMES[8] {
                Fields::StrictFileInteractability
            } else if s == FIELD_JSON_NAMES[9] {
                Fields::UnhandledPromptBehavior
            }
            //
            else if s == FIELD_JSON_NAMES[10] {
                Fields::MozAccessibilityChecks
            } else if s == FIELD_JSON_NAMES[11] {
                Fields::MozBuildID
            } else if s == FIELD_JSON_NAMES[12] {
                Fields::MozGeckodriverVersion
            } else if s == FIELD_JSON_NAMES[13] {
                Fields::MozHeadless
            } else if s == FIELD_JSON_NAMES[14] {
                Fields::MozProcessID
            } else if s == FIELD_JSON_NAMES[15] {
                Fields::MozProfile
            } else if s == FIELD_JSON_NAMES[16] {
                Fields::MozShutdownTimeout
            } else if s == FIELD_JSON_NAMES[17] {
                Fields::MozUseNonSpecCompliantPointerOrigin
            } else if s == FIELD_JSON_NAMES[18] {
                Fields::MozWebdriverClick
            } else if s == FIELD_JSON_NAMES[19] {
                Fields::MozWindowless
            } else if s == FIELD_JSON_NAMES[20] {
                Fields::WebSocketUrl
            } else {
                Fields::Reserved(s.to_string())
            }
        }
        fn to_sstr(&self) -> &'static str {
            match self {
                Fields::BrowserName => FIELD_JSON_NAMES[0],
                Fields::BrowserVersion => FIELD_JSON_NAMES[1],
                Fields::PlatformName => FIELD_JSON_NAMES[2],
                Fields::AcceptInsecureCerts => FIELD_JSON_NAMES[3],
                Fields::PageLoadStrategy => FIELD_JSON_NAMES[4],
                Fields::Proxy => FIELD_JSON_NAMES[5],
                Fields::WindowRect => FIELD_JSON_NAMES[6],
                Fields::Timeouts => FIELD_JSON_NAMES[7],
                Fields::StrictFileInteractability => FIELD_JSON_NAMES[8],
                Fields::UnhandledPromptBehavior => FIELD_JSON_NAMES[9],
                //
                Fields::MozAccessibilityChecks => FIELD_JSON_NAMES[10],
                Fields::MozBuildID => FIELD_JSON_NAMES[11],
                Fields::MozGeckodriverVersion => FIELD_JSON_NAMES[12],
                Fields::MozHeadless => FIELD_JSON_NAMES[13],
                Fields::MozProcessID => FIELD_JSON_NAMES[14],
                Fields::MozProfile => FIELD_JSON_NAMES[15],
                Fields::MozShutdownTimeout => FIELD_JSON_NAMES[16],
                Fields::MozUseNonSpecCompliantPointerOrigin => FIELD_JSON_NAMES[17],
                Fields::MozWebdriverClick => FIELD_JSON_NAMES[18],
                Fields::MozWindowless => FIELD_JSON_NAMES[19],
                //
                Fields::WebSocketUrl => FIELD_JSON_NAMES[20],
                _ => panic!("unsuppoted operation"),
            }
        }
    }

    struct FieldVisitor;

    struct StructVisitor;

    impl<'de> Visitor<'de> for FieldVisitor {
        type Value = Fields;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "expecting field")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Fields::from_str(v))
        }
    }

    #[cfg(target_family = "unix")]
    impl<'de> Visitor<'de> for StructVisitor {
        type Value = super::FirefoxCapa<'de>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "expecting struct")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            // FIXME: once wdcmd becomes crate, these should be unified
            macro_rules! decl {
                // declare a local var, consisting of field
                // identifier and its value's type.
                ($field_val:ident, fid $what_field:expr, vtype $val_type:ty) => {
                    // `None` leads to a "missing field" error
                    let mut $field_val: (Option<$val_type>, &str) =
                        (None, Fields::to_sstr(&$what_field));
                };
            }

            // each corresponds to one `Field`
            decl!(f0, fid Fields::BrowserName, vtype &'de str);
            decl!(f1, fid Fields::BrowserVersion, vtype &'de str);
            decl!(f2, fid Fields::PlatformName, vtype &'de str);
            decl!(f3, fid Fields::AcceptInsecureCerts, vtype bool);
            decl!(f4, fid Fields::PageLoadStrategy, vtype &'de str);
            // 'f5 != 'de , borrow is impossible, hence completely move
            decl!(f5, fid Fields::Proxy, vtype comm::Proxy<'de>);
            decl!(f6, fid Fields::WindowRect, vtype bool);
            decl!(f7, fid Fields::Timeouts, vtype comm::Timeouts);
            decl!(f8, fid Fields::StrictFileInteractability, vtype bool);
            decl!(f9, fid Fields::UnhandledPromptBehavior, vtype &'de str);
            decl!(f99, fid Fields::WebSocketUrl, vtype &'de str);
            decl!(f10, fid Fields::MozAccessibilityChecks, vtype bool);
            decl!(f11, fid Fields::MozBuildID, vtype String);
            decl!(f12, fid Fields::MozGeckodriverVersion, vtype &'de str);
            // decl!(f12, fid Fields::MozGeckodriverVersion, vtype String);
            decl!(f13, fid Fields::MozHeadless, vtype bool);
            decl!(f14, fid Fields::MozProcessID, vtype u32);
            decl!(f15, fid Fields::MozProfile, vtype &'de str);
            decl!(f16, fid Fields::MozShutdownTimeout, vtype u32);
            decl!(f17, fid Fields::MozUseNonSpecCompliantPointerOrigin, vtype bool);
            decl!(f18, fid Fields::MozWebdriverClick, vtype bool);
            decl!(f19, fid Fields::MozWindowless, vtype bool);

            // none or wrap a duplication error
            let mut is_dup: Option<A::Error> = None;
            macro_rules! give_value_to {
                ($field_obj:ident) => {
                    match $field_obj.0 {
                        None => $field_obj.0 = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(f0.1)),
                    }
                };
            }

            while let Some(key) = map.next_key()? {
                match key {
                    Fields::BrowserName => give_value_to!(f0),
                    Fields::BrowserVersion => give_value_to!(f1),
                    Fields::PlatformName => give_value_to!(f2),
                    Fields::AcceptInsecureCerts => give_value_to!(f3),
                    Fields::PageLoadStrategy => give_value_to!(f4),
                    Fields::Proxy => give_value_to!(f5),
                    Fields::WindowRect => give_value_to!(f6),
                    Fields::Timeouts => give_value_to!(f7),
                    Fields::StrictFileInteractability => give_value_to!(f8),
                    Fields::UnhandledPromptBehavior => give_value_to!(f9),
                    Fields::WebSocketUrl => give_value_to!(f99),
                    //
                    Fields::MozAccessibilityChecks => give_value_to!(f10),
                    Fields::MozBuildID => give_value_to!(f11),
                    Fields::MozGeckodriverVersion => give_value_to!(f12),
                    Fields::MozHeadless => give_value_to!(f13),
                    Fields::MozProcessID => give_value_to!(f14),
                    Fields::MozProfile => give_value_to!(f15),
                    Fields::MozShutdownTimeout => give_value_to!(f16),
                    Fields::MozUseNonSpecCompliantPointerOrigin => give_value_to!(f17),
                    Fields::MozWebdriverClick => give_value_to!(f18),
                    Fields::MozWindowless => give_value_to!(f19),
                    //
                    _ => {
                        map.next_value::<IgnoredAny>()?; // advance 1 token
                        crate::dbgmsg!("SKIP...key {:?} and its value", key);
                    }
                }
            }

            if let Some(e) = is_dup {
                Err(e)
            } else {
                macro_rules! unwrap_or_missing_error {
                        // unwrap a variable to itself, assuming that
			// field is type of (Options<T>, "string name")
                        ($f: ident) => {
			    let $f = $f.0.ok_or_else(|| serde::de::Error::missing_field($f.1))?;
                        };
                        ($f:ident, $($flist:ident),+) => {
			    unwrap_or_missing_error!($f);
			    unwrap_or_missing_error!($($flist),+);
                        };
		    }

                unwrap_or_missing_error!(f0, f1, f2, f3, f4, f6, f7, f8, f9, f15);

                let mut ret = super::FirefoxCapa::default();

                ret.set_browser_name(f0);
                ret.set_browser_version(f1);
                ret.set_platform_name(f2);
                ret.set_accept_insecure_certs(f3);
                ret.set_page_load_strategy(f4);
                if let Some(_f5) = f5.0 {
                    // ff 78esr has no "proxy", after that, all have "proxy"
                    ret.proxy = Some(_f5);
                }
                ret.set_window_rect(f6);
                ret.set_timeouts_script(f7.script);
                ret.set_timeouts_page_load(f7.page_load);
                ret.set_timeouts_implicit(f7.implicit);
                ret.set_strict_file_interactability(f8);
                ret.set_unhandled_prompt_behavior(f9);

                if let Some(v) = f99.0 {
                    // missing is not an error
                    ret.set_wsurl(v);
                }

                ret.set_profile(f15);

                Ok(ret)
            }
        }
    }

    #[cfg(target_family = "windows")]
    impl<'de> Visitor<'de> for StructVisitor {
        type Value = super::FirefoxCapa<'de>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "expecting struct")
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            macro_rules! decl {
                ($field_val:ident, fid $what_field:expr, vtype $val_type:ty) => {
                    let mut $field_val: (Option<$val_type>, &str) =
                        (None, Fields::to_sstr(&$what_field));
                };
            }

            decl!(f0, fid Fields::BrowserName, vtype String);
            decl!(f1, fid Fields::BrowserVersion, vtype String);
            decl!(f2, fid Fields::PlatformName, vtype String);
            decl!(f3, fid Fields::AcceptInsecureCerts, vtype bool);
            decl!(f4, fid Fields::PageLoadStrategy, vtype String);
            decl!(f5, fid Fields::Proxy, vtype comm::Proxy<'de>);
            decl!(f6, fid Fields::WindowRect, vtype bool);
            decl!(f7, fid Fields::Timeouts, vtype comm::Timeouts);
            decl!(f8, fid Fields::StrictFileInteractability, vtype bool);
            decl!(f9, fid Fields::UnhandledPromptBehavior, vtype String);
            decl!(f99, fid Fields::WebSocketUrl, vtype String);
            decl!(f10, fid Fields::MozAccessibilityChecks, vtype bool);
            decl!(f11, fid Fields::MozBuildID, vtype String);
            decl!(f12, fid Fields::MozGeckodriverVersion, vtype String);
            // decl!(f12, fid Fields::MozGeckodriverVersion, vtype String);
            decl!(f13, fid Fields::MozHeadless, vtype bool);
            decl!(f14, fid Fields::MozProcessID, vtype u32);
            decl!(f15, fid Fields::MozProfile, vtype String);
            decl!(f16, fid Fields::MozShutdownTimeout, vtype u32);
            decl!(f17, fid Fields::MozUseNonSpecCompliantPointerOrigin, vtype bool);
            decl!(f18, fid Fields::MozWebdriverClick, vtype bool);
            decl!(f19, fid Fields::MozWindowless, vtype bool);

            // none or wrap a duplication error
            let mut is_dup: Option<A::Error> = None;
            macro_rules! give_value_to {
                ($field_obj:ident) => {
                    match $field_obj.0 {
                        None => $field_obj.0 = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(f0.1)),
                    }
                };
            }

            while let Some(key) = map.next_key()? {
                match key {
                    Fields::BrowserName => give_value_to!(f0),
                    Fields::BrowserVersion => give_value_to!(f1),
                    Fields::PlatformName => give_value_to!(f2),
                    Fields::AcceptInsecureCerts => give_value_to!(f3),
                    Fields::PageLoadStrategy => give_value_to!(f4),
                    Fields::Proxy => give_value_to!(f5),
                    Fields::WindowRect => give_value_to!(f6),
                    Fields::Timeouts => give_value_to!(f7),
                    Fields::StrictFileInteractability => give_value_to!(f8),
                    Fields::UnhandledPromptBehavior => give_value_to!(f9),
                    Fields::WebSocketUrl => give_value_to!(f99),
                    //
                    Fields::MozAccessibilityChecks => give_value_to!(f10),
                    Fields::MozBuildID => give_value_to!(f11),
                    Fields::MozGeckodriverVersion => give_value_to!(f12),
                    Fields::MozHeadless => give_value_to!(f13),
                    Fields::MozProcessID => give_value_to!(f14),
                    Fields::MozProfile => give_value_to!(f15),
                    Fields::MozShutdownTimeout => give_value_to!(f16),
                    Fields::MozUseNonSpecCompliantPointerOrigin => give_value_to!(f17),
                    Fields::MozWebdriverClick => give_value_to!(f18),
                    Fields::MozWindowless => give_value_to!(f19),
                    //
                    _ => {
                        map.next_value::<IgnoredAny>()?; // advance 1 token
                        crate::dbgmsg!("SKIP...key {:?} and its value", key);
                    }
                }
            }

            if let Some(e) = is_dup {
                Err(e)
            } else {
                macro_rules! unwrap_or_missing_error {
                        ($f: ident) => {
			    let $f = $f.0.ok_or_else(|| serde::de::Error::missing_field($f.1))?;
                        };
                        ($f:ident, $($flist:ident),+) => {
			    unwrap_or_missing_error!($f);
			    unwrap_or_missing_error!($($flist),+);
                        };
		    }

                unwrap_or_missing_error!(f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f15);

                let mut ret = super::FirefoxCapa::default();

                ret.set_browser_name_take(f0);
                ret.set_browser_version_take(f1);
                ret.set_platform_name_take(f2);
                ret.set_accept_insecure_certs(f3);
                ret.set_page_load_strategy_take(f4);
                ret.proxy = Some(f5);
                ret.set_window_rect(f6);
                ret.set_timeouts_script(f7.script);
                ret.set_timeouts_page_load(f7.page_load);
                ret.set_timeouts_implicit(f7.implicit);
                ret.set_strict_file_interactability(f8);
                ret.set_unhandled_prompt_behavior_take(f9);

                if let Some(v) = f99.0 {
                    // missing is not an error
                    ret.set_wsurl_take(v);
                }

                ret.set_profile_take(f15);

                Ok(ret)
            }
        }
    }

    // trivial
    impl<'de> Deserialize<'de> for Fields {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_identifier(FieldVisitor)
        }
    }

    // trivial
    impl<'de> Deserialize<'de> for FirefoxCapa<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_struct("CmdRespChrom106", FIELD_JSON_NAMES, StructVisitor)
        }
    }
}

//  Unit Tests  //

mod utst {
    #[allow(unused)]
    use super::*;

    #[cfg(test)]
    mod w3c_compl {
        use super::*;
        use w3c::{W3cCapRequSetter, W3cCapaGetter, W3cCapaSetter};

        #[test]
        fn _0() {
            // no user intention present. this results in a invalid session,
            // #3-3 on https://w3c.github.io/webdriver/#dfn-matching-capabilities
            // will not make sure the final matched capabilities not null. And
            // this is prevented in advance per by #7.2-3-2.
            let requ = GeckoCapRequ::default();

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _1() {
            // a tolerant match, this is the least condition for a
            // valid session, see #7.2-3-2.

            match std::net::TcpStream::connect("") {
                Ok(_) => {
                    let mut requ = GeckoCapRequ::default();
                    {
                        let capa = FirefoxCapa::default();

                        requ.allow_as_w3c(&capa);

                        assert_eq!(requ.always_match.browser_name().is_none(), true);
                        assert_eq!(requ.always_match.platform_name().is_none(), true);
                        assert_eq!(requ.always_match.is_insig_as_w3c(), true);
                        assert_eq!(requ.first_match.len(), 1);
                    }
                }
                _ => {}
            }
        }

        #[test]
        fn _2() {
            let mut requ = GeckoCapRequ::default();
            let capa = FirefoxCapa::default();

            requ.allow_as_w3c(&capa);
            requ.allow_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), true);
            assert_eq!(requ.first_match.len(), 2);
        }

        #[test]
        fn _3() {
            // exact one platform
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_platform_name("linux");
            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_some(), true);
            assert_eq!(requ.always_match.platform_name().unwrap(), "linux");
            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _4() {
            // exact one platform
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.first_match.len(), 1);
            assert_eq!(
                requ.first_match.get(0).unwrap().platform_name().unwrap(),
                "linux"
            );
        }

        #[test]
        fn _5() {
            // any browser, exact one of two platforms
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);
            let mut capa = FirefoxCapa::default();
            capa.set_platform_name("win");
            requ.allow_as_w3c(&capa);

            assert_eq!(requ.first_match.len(), 2);
            assert_eq!(
                requ.first_match.get(0).unwrap().platform_name().unwrap(),
                "linux"
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().platform_name().unwrap(),
                "win"
            );
        }

        #[test]
        fn _6() {
            // exact one browser, exact one of three platforms
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            let mut capa1 = FirefoxCapa::default();
            let mut capa2 = FirefoxCapa::default();
            let mut capa3 = FirefoxCapa::default();
            capa1.set_platform_name("linux");
            capa2.set_platform_name("mac");
            capa3.set_platform_name("win");

            requ.allow_as_w3c(&capa1)
                .allow_as_w3c(&capa2)
                .allow_as_w3c(&capa3);

            assert_eq!(requ.always_match.browser_name().is_some(), true);
            assert_eq!(requ.always_match.browser_name().unwrap(), "firefox");
            assert_eq!(requ.first_match.len(), 3);
            assert_eq!(
                requ.first_match.get(0).unwrap().platform_name().unwrap(),
                "linux"
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().platform_name().unwrap(),
                "mac"
            );
            assert_eq!(
                requ.first_match.get(2).unwrap().platform_name().unwrap(),
                "win"
            );
        }

        #[test]
        fn _7() {
            // capa conflicts
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);
            let mut capa = FirefoxCapa::default();
            capa.set_browser_name("edge");
            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);
            assert_eq!(requ.always_match.browser_name().is_some(), true);
            assert_eq!(requ.always_match.browser_name().unwrap(), "firefox");
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _8() {
            // mandate() override allow()
            let mut requ = GeckoCapRequ::default();

            let mut capa = FirefoxCapa::default();
            capa.set_browser_name("edge");
            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_browser_name("edge");
            capa.set_platform_name("win");
            requ.allow_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.first_match.len(), 2);

            let mut capa = FirefoxCapa::default();
            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_some(), true);
            assert_eq!(requ.always_match.browser_name().unwrap(), "firefox");
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _9() {
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_timeouts_script(1234);
            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.timeouts_script().is_some(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _10() {
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_proxy_type("manual");
            capa.set_socks_version(5);
            capa.set_socks_proxy("127.0.0.1:10801");

            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.proxy_type().is_some(), true);
            assert_eq!(requ.always_match.socks_version().is_some(), true);
            assert_eq!(requ.always_match.socks_proxy().is_some(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            assert_eq!(requ.first_match.len(), 0);
        }
    } // w3c compliance

    #[cfg(test)]
    mod gecko_compl {
        use super::*;
        use gecko::{FirefoxCapaGetter, FirefoxCapaSetter, GeckoCapRequSetter};

        #[test]
        fn _1() {
            // bare initialization

            let requ = GeckoCapRequ::default();

            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.always_match.is_insig(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _2() {
            let mut requ = GeckoCapRequ::default();
            let capa = FirefoxCapa::default();

            requ.allow(&capa);

            assert_eq!(requ.first_match.len(), 1);
        }

        #[test]
        fn _3() {
            let mut requ = GeckoCapRequ::default();
            let capa = FirefoxCapa::default();
            ();

            requ.allow(&capa);
            requ.allow(&capa);

            assert_eq!(requ.first_match.len(), 2);
        }

        #[test]
        fn _4() {
            // `binary` field
            // exact one binary
            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_binary("/path/to/binary");
            requ.mandate(&capa);

            assert_eq!(requ.always_match.binary().is_none(), false);
            assert_eq!(requ.always_match.binary().unwrap(), "/path/to/binary");
            assert_eq!(requ.always_match.is_insig(), false);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _5() {
            // `binary` `args` field
            // exact one of two binaries
            let mut requ = GeckoCapRequ::default();
            let mut capa1 = FirefoxCapa::default();
            let mut capa2 = FirefoxCapa::default();

            capa1.set_binary("/path/to/binary");
            capa1.add_args("--compact-dump");
            capa2.set_binary("/path/to/binary-beta");
            capa2.add_args("--verbose-dump");
            capa2.add_args("--headless");
            requ.allow(&capa1).allow(&capa2);

            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.first_match.len(), 2);
            assert_eq!(
                requ.first_match.get(0).unwrap().binary().unwrap(),
                "/path/to/binary"
            );
            assert_eq!(
                requ.first_match.get(0).unwrap().args().unwrap(),
                vec!["--compact-dump"]
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().binary().unwrap(),
                "/path/to/binary-beta"
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().args().unwrap(),
                vec!["--verbose-dump", "--headless"]
            );
        }

        #[test]
        fn _6() {
            // `binary` `args` field
            // exact one args, exact one of two binaries
            let mut requ = GeckoCapRequ::default();
            let mut capa1 = FirefoxCapa::default();
            let mut capa2 = FirefoxCapa::default();
            let mut capa3 = FirefoxCapa::default();

            capa1.add_args("--headless");
            capa1.add_args("--no-proxy-server");
            requ.mandate(&capa1);
            capa2.set_binary("/path/to/binary");
            capa3.set_binary("/path/to/binary-beta");
            requ.allow(&capa2).allow(&capa3);

            assert_eq!(requ.always_match.args().is_some(), true);
            assert_eq!(requ.always_match.args().unwrap().len(), 2);
            assert_eq!(
                requ.always_match.args().unwrap().get(0).unwrap(),
                &"--headless"
            );
            assert_eq!(
                requ.always_match.args().unwrap().get(1).unwrap(),
                &"--no-proxy-server"
            );
            assert_eq!(requ.first_match.len(), 2);
            assert_eq!(
                requ.first_match.get(0).unwrap().binary().unwrap(),
                "/path/to/binary"
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().binary().unwrap(),
                "/path/to/binary-beta"
            );
        }

        #[test]
        fn _7() {
            // `binary` field
            // mandate() override allow()
            let mut requ = GeckoCapRequ::default();

            let mut capa = FirefoxCapa::default();
            capa.set_binary("/path/to/binary_v1.0");
            requ.allow(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_binary("/path/to/binary_v2.0");
            requ.allow(&capa);

            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.first_match.len(), 2);

            let mut capa = FirefoxCapa::default();
            capa.set_binary("/path/to/binary_v3.0");
            requ.mandate(&capa);

            assert_eq!(requ.always_match.binary().is_some(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _8() {
            // `prefs` field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_binary("/path/to/firefox");
            capa.add_prefs("browser.urlbar.placeholderName", "My Search Engine");
            capa.add_prefs("javascript.enabled", "false");

            requ.mandate(&capa);

            assert_eq!(requ.always_match.binary().is_some(), true);
            assert_eq!(requ.always_match.prefs().is_some(), true);
            assert_eq!(requ.always_match.prefs().unwrap().len(), 2);
            assert_eq!(requ.first_match.len(), 0);
        }
    } // gecko compliance

    #[cfg(test)]
    mod together_compl {
        use super::*;
        use gecko::{FirefoxCapaGetter, FirefoxCapaSetter, GeckoCapRequSetter};
        use w3c::{W3cCapRequSetter, W3cCapaGetter, W3cCapaSetter};

        #[test]
        fn _0() {
            // use as w3c conf works just partly

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox");
            capa.set_browser_version("106.0.0");
            capa.set_binary("/path/to/firefox");
            capa.add_args("--headless");

            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            // all gecko-* are insig:
            assert_eq!(requ.always_match.is_insig(), true);
            //  no bin:
            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _1() {
            // use as gecko conf works fine

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox");
            capa.set_browser_version("106.0.0");
            capa.set_binary("/path/to/firefox");
            capa.add_args("--headless");

            requ.mandate(&capa); // use gecko own impl mandate

            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            // not all gecko-* are insig:
            assert_eq!(requ.always_match.is_insig(), false);
            // there is bin
            assert_eq!(requ.always_match.binary().is_some(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _2() {
            // use as gecko conf works fine

            let mut requ = GeckoCapRequ::default();
            let mut capa1 = FirefoxCapa::default();
            let mut capa2 = FirefoxCapa::default();
            let mut capa3 = FirefoxCapa::default();

            capa1.set_browser_name("firefox");
            requ.mandate(&capa1);

            capa2.set_platform_name("linux");
            capa2.add_args("--headless");
            capa3.set_platform_name("win");
            capa3.add_args("--headless");
            requ.allow(&capa2).allow(&capa3);

            assert_eq!(requ.always_match.browser_name().is_some(), true);
            assert_eq!(requ.always_match.browser_name().unwrap(), "firefox");
            assert_eq!(requ.first_match.len(), 2);
            assert_eq!(
                requ.first_match.get(0).unwrap().platform_name().unwrap(),
                "linux"
            );
            assert_eq!(
                requ.first_match.get(0).unwrap().args().unwrap(),
                vec!["--headless"]
            );
            assert_eq!(
                requ.first_match.get(1).unwrap().platform_name().unwrap(),
                "win"
            );
            assert_eq!(
                requ.first_match.get(0).unwrap().args().unwrap(),
                vec!["--headless"]
            );
        }
    } // together

    #[cfg(test)]
    mod ser {
        use super::*;
        use gecko::GeckoCapRequSetter;
        use w3c::{W3cCapRequSetter, W3cCapaSetter};

        use serde_test::assert_ser_tokens;
        use serde_test::Token;

        #[test]
        fn _1() {
            let requ = GeckoCapRequ::default();

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _2() {
            let mut requ = GeckoCapRequ::default();
            let capa = FirefoxCapa::default();

            requ.allow(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(1) },
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{},"firstMatch":[{}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _3() {
            // currently, multiple empty ele inside am is valid,
            // but this should be FIXME in the future.

            let mut requ = GeckoCapRequ::default();
            let capa = FirefoxCapa::default();

            requ.allow(&capa);
            requ.allow(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(2) },
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?.
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{},"firstMatch":[{},{}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _4() {
            // set 1 field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_platform_name("linux");

            requ.mandate_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("platformName"),
                    Token::Str("linux"),
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"platformName":"linux"},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _44() {
            // set 1 field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_timeouts_script(1234);

            requ.mandate_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("timeouts"),
                    Token::Struct {
                        name: "Timeouts",
                        len: 3,
                    },
                    Token::Str("script"),
                    Token::U32(1234),
                    Token::Str("pageLoad"),
                    Token::U32(300_000),
                    Token::Str("implicit"),
                    Token::U32(0),
                    Token::StructEnd,
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"timeouts":{"script":1234,"pageLoad":300000,"implicit":0}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _444() {
            // set proxy field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_proxy_type("manual");
            capa.set_socks_version(5);
            capa.set_socks_proxy("127.0.0.1:1080");
            capa.add_no_proxy("192.168.1.0/24");

            requ.mandate_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("proxy"),
                    Token::Struct {
                        name: "Proxy",
                        len: 8,
                    },
                    Token::Str("proxyType"),
                    Token::Str("manual"),
                    Token::Str("noProxy"),
                    Token::Seq { len: Some(1) },
                    Token::Str("192.168.1.0/24"),
                    Token::SeqEnd,
                    Token::Str("socksProxy"),
                    Token::Str("127.0.0.1:1080"),
                    Token::Str("socksVersion"),
                    Token::U8(5),
                    Token::StructEnd,
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct ?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"proxy":{"proxyType":"manual","noProxy":["192.168.1.0/24"],"socksProxy":"127.0.0.1:1080","socksVersion":5}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _4444() {
            // set 1 field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.enable_bidi();

            requ.mandate_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("webSocketUrl"),
                    Token::Bool(true),
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"webSocketUrl":true},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _5() {
            // set two fields

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_platform_name("linux");
            capa.set_browser_name("firefox");
            // NOTE that here setting order has no effect on final tokens,
            // final token stream will be compliant to the order specified
            // on https://w3c.github.io/webdriver/#dfn-table-of-standard-capabilities

            requ.mandate_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("browserName"),
                    Token::Str("firefox"),
                    Token::Str("platformName"),
                    Token::Str("linux"),
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"browserName":"firefox","platformName":"linux"},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _6() {
            //  mandate 1, allow 2

            let mut requ = GeckoCapRequ::default();

            let mut capa = FirefoxCapa::default();
            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_platform_name("win");
            requ.allow_as_w3c(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("browserName"),
                    Token::Str("firefox"),
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(2) },
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("platformName"),
                    Token::Str("linux"),
                    Token::StructEnd,
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("platformName"),
                    Token::Str("win"),
                    Token::StructEnd,
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"browserName":"firefox"},"firstMatch":[{"platformName":"linux"},{"platformName":"win"}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _7() {
            // allow two platforms, mandate one args

            let mut requ = GeckoCapRequ::default();

            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox");
            capa.add_args("--headless");
            requ.mandate(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_platform_name("linux");
            capa.set_binary("/usr/bin/firefox");
            requ.allow(&capa);

            let mut capa = FirefoxCapa::default();
            capa.set_platform_name("win");
            capa.set_binary("/c/program/firefox");
            requ.allow(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("browserName"),
                    Token::Str("firefox"),
                    Token::Str("moz:firefoxOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("args"),
                    Token::Seq { len: Some(1) },
                    Token::Str("--headless"),
                    Token::SeqEnd,
                    Token::StructEnd,
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(2) },
                    // list ele
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("platformName"),
                    Token::Str("linux"),
                    Token::Str("moz:firefoxOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("binary"),
                    Token::Str("/usr/bin/firefox"),
                    Token::StructEnd,
                    Token::StructEnd,
                    // list ele END
                    // list ele
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("platformName"),
                    Token::Str("win"),
                    Token::Str("moz:firefoxOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("binary"),
                    Token::Str("/c/program/firefox"),
                    Token::StructEnd,
                    Token::StructEnd,
                    // list ele END
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"browserName":"firefox","moz:firefoxOptions":{"args":["--headless"]}},"firstMatch":[{"platformName":"linux","moz:firefoxOptions":{"binary":"/usr/bin/firefox"}},{"platformName":"win","moz:firefoxOptions":{"binary":"/c/program/firefox"}}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _8() {
            // `prefs` field

            let mut requ = GeckoCapRequ::default();
            let mut capa = FirefoxCapa::default();

            capa.set_browser_name("firefox-nightly");
            capa.set_binary("/path/to/firefox");
            capa.add_prefs("browser.urlbar.placeholderName", "My Search Engine");
            capa.add_prefs("javascript.enabled", "false");

            requ.mandate(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("browserName"),
                    Token::Str("firefox-nightly"),
                    Token::Str("moz:firefoxOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("binary"),
                    Token::Str("/path/to/firefox"),
                    Token::Str("prefs"),
                    Token::Map { len: Some(2) },
                    Token::Str("browser.urlbar.placeholderName"),
                    Token::Str("My Search Engine"),
                    Token::Str("javascript.enabled"),
                    Token::Bool(false),
                    Token::MapEnd,
                    Token::StructEnd,
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"browserName":"firefox-nightly","moz:firefoxOptions":{"binary":"/path/to/firefox","prefs":{"browser.urlbar.placeholderName":"My Search Engine","javascript.enabled":false}}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _88() {
            // `prefs` field, use non-std way

            let mut requ = GeckoCapRequ::default();

            let mut capa = FirefoxCapa::default();
            capa.set_proxy_type("manual");
            capa.set_socks_version(5);
            capa.set_socks_proxy("127.0.0.1:1080");
            capa.add_prefs("network.proxy.socks_remote_dns", "true");
            requ.mandate(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("proxy"),
                    Token::Struct {
                        name: "Proxy",
                        len: 8,
                    },
                    Token::Str("proxyType"),
                    Token::Str("manual"),
                    Token::Str("socksProxy"),
                    Token::Str("127.0.0.1:1080"),
                    Token::Str("socksVersion"),
                    Token::U8(5),
                    Token::StructEnd,
                    Token::Str("moz:firefoxOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("prefs"),
                    Token::Map { len: Some(1) },
                    Token::Str("network.proxy.socks_remote_dns"),
                    Token::Bool(true),
                    Token::MapEnd,
                    Token::StructEnd,
                    Token::StructEnd,
                    Token::Str("firstMatch"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"proxy":{"proxyType":"manual","socksProxy":"127.0.0.1:1080","socksVersion":5},"moz:firefoxOptions":{"prefs":{"network.proxy.socks_remote_dns":true}}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }
    } // ser

    #[cfg(test)]
    mod deser {
        use super::*;
        use w3c::W3cSessResultGetter;

        #[test]
        fn _1() {
            let resp_str = r#"{"value":{"sessionId":"314a5046-3bd1-4f4b-bb5b-811166386d0e","capabilities":{"acceptInsecureCerts":false,"browserName":"firefox","browserVersion":"102.9.0","moz:accessibilityChecks":false,"moz:buildID":"20230309232621","moz:geckodriverVersion":"0.32.2","moz:headless":false,"moz:processID":149672,"moz:profile":"/tmp/rust_mozprofilebDGEzq","moz:shutdownTimeout":60000,"moz:useNonSpecCompliantPointerOrigin":false,"moz:webdriverClick":true,"moz:windowless":false,"pageLoadStrategy":"normal","platformName":"linux","platformVersion":"6.1.0-6-amd64","proxy":{},"setWindowRect":true,"strictFileInteractability":false,"timeouts":{"implicit":0,"pageLoad":300000,"script":30000},"unhandledPromptBehavior":"dismiss and notify"}}}"#;

            let obj = serde_json::from_slice::<GeckoSessResult>(resp_str.as_bytes()).unwrap();

            assert_eq!(obj.session_id(), "314a5046-3bd1-4f4b-bb5b-811166386d0e");
        }

        #[test]
        fn _11() {
            let resp_str = r#"{"value":{"sessionId":"f1425a03-0245-43f6-8cdc-541af79a7720","capabilities":{"acceptInsecureCerts":false,"browserName":"firefox","browserVersion":"102.9.0","moz:accessibilityChecks":false,"moz:buildID":"20230309232621","moz:debuggerAddress":"localhost:9222","moz:geckodriverVersion":"0.33.0","moz:headless":false,"moz:processID":457134,"moz:profile":"/tmp/rust_mozprofilezTTjQ9","moz:shutdownTimeout":60000,"moz:useNonSpecCompliantPointerOrigin":false,"moz:webdriverClick":true,"moz:windowless":false,"pageLoadStrategy":"normal","platformName":"linux","platformVersion":"6.1.0-7-amd64","proxy":{},"setWindowRect":true,"strictFileInteractability":false,"timeouts":{"implicit":0,"pageLoad":300000,"script":30000},"unhandledPromptBehavior":"dismiss and notify","webSocketUrl":"ws://localhost:9222/session/f1425a03-0245-43f6-8cdc-541af79a7720"}}}"#;

            let obj = serde_json::from_slice::<GeckoSessResult>(resp_str.as_bytes()).unwrap();

            assert_eq!(obj.session_id(), "f1425a03-0245-43f6-8cdc-541af79a7720");
            assert_eq!(
                obj.wsurl().unwrap(),
                "ws://localhost:9222/session/f1425a03-0245-43f6-8cdc-541af79a7720"
            );
        }
    } // deser
} // utst
