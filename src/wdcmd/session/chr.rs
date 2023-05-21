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
    use super::any as chrom;

    use std::collections::BTreeMap;

    ///
    /// Setters for ChromeDriver-specific capabilities request.
    pub trait ChromCapRequSetter<'c1, 'c2> {
        fn mandate(&mut self, other: &'c1 (impl chrom::ChromiumCapaGetter + w3c::W3cCapaGetter));
        fn allow(
            &mut self,
            other: &'c2 (impl chrom::ChromiumCapaGetter + w3c::W3cCapaGetter),
        ) -> &mut Self;
    }

    ///
    /// Getters for Chromium-specific capabilities.
    pub trait ChromiumCapaGetter {
        // getter
        fn args(&self) -> Option<Vec<&str>> {
            None
        }
        fn binary(&self) -> Option<&str> {
            None
        }
        fn extensions(&self) -> Option<Vec<&str>> {
            None
        }
        fn prefs(&self) -> Option<BTreeMap<&str, &str>> {
            None
        }
        fn detach(&self) -> Option<bool> {
            None
        }
        fn debugger_address(&self) -> Option<&str> {
            None
        }
        fn exclude_switches(&self) -> Option<Vec<&str>> {
            None
        }
        fn minidump_path(&self) -> Option<&str> {
            None
        }
        fn window_types(&self) -> Option<Vec<&str>> {
            None
        }
        fn is_insig(&self) -> bool {
            self.binary().is_none()
                && self.args().is_none()
                && self.extensions().is_none()
                && self.prefs().is_none()
                && self.detach().is_none()
                && self.debugger_address().is_none()
                && self.exclude_switches().is_none()
                && self.minidump_path().is_none()
                && self.window_types().is_none()
        }
    }

    ///
    /// Setters for Chromium-specific capabilities.
    pub trait ChromiumCapaSetter<'c> {
        // setter
        #[allow(unused_variables)]
        fn set_args(&mut self, arg: Vec<&'c str>) {}
        #[allow(unused_variables)]
        fn add_args(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_binary(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_extensions(&mut self, arg: Vec<&'c str>) {}
        #[allow(unused_variables)]
        fn add_extensions(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_prefs(&mut self, arg: BTreeMap<&'c str, &'c str>) {}
        #[allow(unused_variables)]
        fn add_prefs(&mut self, key: &'c str, value: &'c str) {}
        #[allow(unused_variables)]
        fn set_detach(&mut self, arg: bool) {}
        #[allow(unused_variables)]
        fn set_debugger_address(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_debugger_address_take(&mut self, arg: String) {}
        #[allow(unused_variables)]
        fn set_exclude_switches(&mut self, arg: Vec<&'c str>) {}
        #[allow(unused_variables)]
        fn add_exclude_switches(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_minidump_path(&mut self, arg: &'c str) {}
        #[allow(unused_variables)]
        fn set_window_types(&mut self, arg: Vec<&'c str>) {}
        #[allow(unused_variables)]
        fn add_window_types(&mut self, arg: &'c str) {}
    }

    ///
    /// Types that have alien capabilities.
    ///
    /// "alien" here means these capabilities are neither standard ones,
    /// nor extension ones, nor additional ones, according to the standard.
    pub trait ChromiumSesAlnCap<'c> {
        // getter
        fn chromedriver_version_as_alien(&self) -> Option<&str> {
            None
        }
        ///
        /// The user data of Chromium instance.
        fn user_data_dir_as_alien(&self) -> Option<&str> {
            None
        }
        fn network_connection_enabled_as_alien(&self) -> Option<bool> {
            None
        }
        fn webauthn_extension_cred_blob_as_alien(&self) -> Option<bool> {
            None
        }
        fn webauthn_extension_large_blob_as_alien(&self) -> Option<bool> {
            None
        }
        // v111
        fn webauthn_extension_min_pin_length_as_alien(&self) -> Option<bool> {
            None
        }
        // v111
        fn webauthn_extension_prf_as_alien(&self) -> Option<bool> {
            None
        }
        fn webauthn_virtual_authenticators_as_alien(&self) -> Option<bool> {
            None
        }

        // setter

        fn set_chromedriver_version_as_alien(&mut self, _arg: &'c str) {}
        fn set_chromedriver_version_take_as_alien(&mut self, _arg: String) {}
        fn set_user_data_dir_as_alien(&mut self, _arg: &'c str) {}
        fn set_user_data_dir_take_as_alien(&mut self, _arg: String) {}
        fn set_network_connection_enabled_as_alien(&mut self, _arg: bool) {}
        fn set_webauthn_extension_cred_blob_as_alien(&mut self, _arg: bool) {}
        fn set_webauthn_extension_large_blob_as_alien(&mut self, _arg: bool) {}
        fn set_webauthn_extension_min_pin_length_as_alien(&mut self, _arg: bool) {}
        fn set_webauthn_extension_prf_as_alien(&mut self, _arg: bool) {}
        fn set_webauthn_virtual_authenticators_as_alien(&mut self, _arg: bool) {}
    }
} // any

//  following impl are tested on v106  //

use super::comm;
use super::w3c;
use any as chrom;

use chrom::{ChromiumCapaGetter, ChromiumCapaSetter, ChromiumSesAlnCap};
use comm::VarValTypeMap;
use w3c::{W3cCapaGetter, W3cCapaSetter};

use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct ChromiumSesExtCap<'c> {
    args: Option<Vec<Cow<'c, str>>>,
    binary: Option<Cow<'c, str>>,
    extensions: Option<Vec<Cow<'c, str>>>,
    prefs: Option<VarValTypeMap<Cow<'c, str>, Cow<'c, str>>>,
    detach: Option<bool>,
    debugger_address: Option<Cow<'c, str>>,
    exclude_switches: Option<Vec<Cow<'c, str>>>,
    minidump_path: Option<Cow<'c, str>>,
    window_types: Option<Vec<Cow<'c, str>>>,
}

#[doc(hidden)]
pub mod alien {
    use std::borrow::Cow;

    // actual data stor
    #[derive(Default, Debug)]
    pub struct ChromiumSesAlnCap<'c> {
        pub(super) chromedriver_version: Option<Cow<'c, str>>,
        pub(super) user_data_dir: Option<Cow<'c, str>>,
        pub(super) network_connection_enabled: Option<bool>,
        // webauthn_* are here because the lack of official docs, not technically
        pub(super) webauthn_extension_cred_blob: Option<bool>,
        pub(super) webauthn_extension_large_blob: Option<bool>,
        pub(super) webauthn_virtual_authenticators: Option<bool>,
    }

    // only used when deser capa, thus only vis for super
    #[derive(Default, Debug)]
    pub(super) struct Chrome<'c> {
        pub(super) chromedriver_version: Cow<'c, str>,
        pub(super) user_data_dir: Cow<'c, str>,
    }

    pub(super) type NetworkConnectionEnable = bool;

    pub(super) type WebauthnExtensionCredBlob = bool;

    pub(super) type WebauthnExtensionLargeBlob = bool;

    pub(super) type WebauthnVirtualAuthenticators = bool;

    mod deser {
        use serde::de::{
            Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor,
        };
        use std::borrow::Cow;

        #[derive(Debug)]
        enum Fields {
            ChromedriverVersion,
            UserDataDir,
            Reserved(String),
        }

        const FIELD_JSON_NAMES: &[&str] = &["chromedriverVersion", "userDataDir"];

        impl Fields {
            fn from_str(s: &str) -> Self {
                if s == FIELD_JSON_NAMES[0] {
                    Fields::ChromedriverVersion
                } else if s == FIELD_JSON_NAMES[1] {
                    Fields::UserDataDir
                } else {
                    Fields::Reserved(s.to_string())
                }
            }
            fn to_sstr(&self) -> &'static str {
                match self {
                    Fields::ChromedriverVersion => FIELD_JSON_NAMES[0],
                    Fields::UserDataDir => FIELD_JSON_NAMES[1],
                    _ => panic!("unsuppoted operation"),
                }
            }
        }

        struct FieldVisitor;

        struct StructVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected field")
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
            type Value = super::Chrome<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut f0: (Option<&'de str>, &str) =
                    (None, Fields::to_sstr(&Fields::ChromedriverVersion));
                let mut f1: (Option<&'de str>, &str) =
                    (None, Fields::to_sstr(&Fields::UserDataDir));

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::ChromedriverVersion => match f0.0 {
                            None => f0.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field(f0.1)),
                        },
                        Fields::UserDataDir => match f1.0 {
                            None => f1.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field(f1.1)),
                        },

                        _ => {
                            map.next_value::<IgnoredAny>()?; // advance 1 token

                            crate::dbgmsg!("SKIP...key {:?} and its value", key);
                        }
                    }
                }

                if let Some(e) = is_dup {
                    Err(e)
                } else {
                    // missing field is an error
                    let f0 = f0.0.ok_or_else(|| DeError::missing_field(f0.1))?;
                    let f1 = f1.0.ok_or_else(|| DeError::missing_field(f1.1))?;

                    Ok(super::Chrome {
                        chromedriver_version: Cow::from(f0),
                        user_data_dir: Cow::from(f1),
                    })
                }
            }
        }

        #[cfg(target_family = "windows")]
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = super::Chrome<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut f0: (Option<String>, &str) =
                    (None, Fields::to_sstr(&Fields::ChromedriverVersion));
                let mut f1: (Option<String>, &str) = (None, Fields::to_sstr(&Fields::UserDataDir));

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::ChromedriverVersion => match f0.0 {
                            None => f0.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field(f0.1)),
                        },
                        Fields::UserDataDir => match f1.0 {
                            None => f1.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field(f1.1)),
                        },

                        _ => {
                            map.next_value::<IgnoredAny>()?; // advance 1 token

                            crate::dbgmsg!("SKIP...key {:?} and its value", key);
                        }
                    }
                }

                if let Some(e) = is_dup {
                    Err(e)
                } else {
                    // missing field is an error
                    let f0 = f0.0.ok_or_else(|| DeError::missing_field(f0.1))?;
                    let f1 = f1.0.ok_or_else(|| DeError::missing_field(f1.1))?;

                    Ok(super::Chrome {
                        chromedriver_version: Cow::from(f0),
                        user_data_dir: Cow::from(f1),
                    })
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

        //  trivial
        impl<'de> Deserialize<'de> for super::Chrome<'de> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct(
                    "An alien Chrome field",
                    FIELD_JSON_NAMES,
                    StructVisitor,
                )
            }
        }
    } // deser
} // alien

#[derive(Default, Debug)]
pub struct ChromiumSesAddCap {}

// ChromiumCapa //

///
/// The Chromium-specific session capabilities.
pub type ChromiumCapa<'c> =
    comm::CommCapa<'c, ChromiumSesExtCap<'c>, ChromiumSesAddCap, alien::ChromiumSesAlnCap<'c>>;

impl<'r, 'c1, 'c2> ChromiumCapa<'r>
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

    fn from_other(other: &'c2 (impl w3c::W3cCapaGetter + chrom::ChromiumCapaGetter)) -> Self {
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

        // chrom
        if let Some(v) = other.binary() {
            newme.set_binary(v);
        }
        if let Some(v) = other.args() {
            newme.set_args(v);
        }
        if let Some(v) = other.extensions() {
            newme.set_extensions(v);
        }
        if let Some(v) = other.prefs() {
            newme.set_prefs(v);
        }
        if let Some(v) = other.detach() {
            newme.set_detach(v);
        }
        if let Some(v) = other.debugger_address() {
            newme.set_debugger_address(v);
        }
        if let Some(v) = other.exclude_switches() {
            newme.set_exclude_switches(v);
        }
        if let Some(v) = other.minidump_path() {
            newme.set_minidump_path(v);
        }
        if let Some(v) = other.window_types() {
            newme.set_window_types(v);
        }

        newme
    }

    fn is_conflict_with(
        &self,
        other: &(impl w3c::W3cCapaGetter + chrom::ChromiumCapaGetter),
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

impl chrom::ChromiumCapaGetter for ChromiumCapa<'_> {
    // getter
    fn args(&self) -> Option<Vec<&str>> {
        match self.ext.args.as_ref() {
            None => None,
            Some(v) => Some(v.iter().map(|x| x.as_ref()).collect()),
        }
    }
    fn binary(&self) -> Option<&str> {
        match self.ext.binary.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn extensions(&self) -> Option<Vec<&str>> {
        match self.ext.extensions.as_ref() {
            None => None,
            Some(v) => Some(v.iter().map(|x| x.as_ref()).collect()),
        }
    }
    fn prefs(&self) -> Option<BTreeMap<&str, &str>> {
        match self.ext.prefs.as_ref() {
            None => None,
            Some(v) => Some(
                v.map
                    .iter()
                    .map(|(k, v)| (k.as_ref(), v.as_ref()))
                    .collect(),
            ),
        }
    }
    fn detach(&self) -> Option<bool> {
        match self.ext.detach {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn debugger_address(&self) -> Option<&str> {
        match self.ext.debugger_address.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn minidump_path(&self) -> Option<&str> {
        match self.ext.minidump_path.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn exclude_switches(&self) -> Option<Vec<&str>> {
        match self.ext.exclude_switches.as_ref() {
            None => None,
            Some(v) => Some(v.iter().map(|x| x.as_ref()).collect()),
        }
    }
    fn window_types(&self) -> Option<Vec<&str>> {
        match self.ext.window_types.as_ref() {
            None => None,
            Some(v) => Some(v.iter().map(|x| x.as_ref()).collect()),
        }
    }
}

impl<'r, 'c> chrom::ChromiumCapaSetter<'c> for ChromiumCapa<'r>
where
    'c: 'r,
{
    // setter
    fn set_args(&mut self, arg: Vec<&'c str>) {
        self.ext.args = Some(arg.iter().map(|x| Cow::from(*x)).collect());
    }
    fn add_args(&mut self, arg: &'c str) {
        match self.ext.args.as_mut() {
            Some(v) => v.push(Cow::from(arg)),
            None => self.set_args(vec![arg]),
        }
    }
    fn set_binary(&mut self, arg: &'c str) {
        self.ext.binary = Some(Cow::from(arg));
    }
    fn set_extensions(&mut self, arg: Vec<&'c str>) {
        self.ext.extensions = Some(arg.iter().map(|x| Cow::from(*x)).collect());
    }
    fn add_extensions(&mut self, arg: &'c str) {
        match self.ext.extensions.as_mut() {
            Some(v) => v.push(Cow::from(arg)),
            None => self.set_args(vec![arg]),
        }
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
                let mut newone = BTreeMap::<&str, &str>::default();
                newone.insert(key, value);
                self.set_prefs(newone);
            }
        }
    }
    fn set_detach(&mut self, arg: bool) {
        self.ext.detach = Some(arg);
    }
    fn set_debugger_address(&mut self, arg: &'c str) {
        self.ext.debugger_address = Some(Cow::from(arg));
    }
    fn set_debugger_address_take(&mut self, arg: String) {
        self.ext.debugger_address = Some(Cow::from(arg));
    }
    fn set_exclude_switches(&mut self, arg: Vec<&'c str>) {
        self.ext.exclude_switches = Some(arg.iter().map(|x| Cow::from(*x)).collect());
    }
    fn add_exclude_switches(&mut self, arg: &'c str) {
        match self.ext.exclude_switches.as_mut() {
            Some(v) => v.push(Cow::from(arg)),
            None => self.set_exclude_switches(vec![arg]),
        }
    }
    fn set_minidump_path(&mut self, arg: &'c str) {
        self.ext.minidump_path = Some(Cow::from(arg));
    }
    fn set_window_types(&mut self, arg: Vec<&'c str>) {
        self.ext.window_types = Some(arg.iter().map(|x| Cow::from(*x)).collect());
    }
    fn add_window_types(&mut self, arg: &'c str) {
        match self.ext.window_types.as_mut() {
            Some(v) => v.push(Cow::from(arg)),
            None => self.set_window_types(vec![arg]),
        }
    }
}

impl<'c> chrom::ChromiumSesAlnCap<'c> for ChromiumCapa<'c> {
    // getter
    fn chromedriver_version_as_alien(&self) -> Option<&str> {
        match self.alien.chromedriver_version.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn user_data_dir_as_alien(&self) -> Option<&str> {
        match self.alien.user_data_dir.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn network_connection_enabled_as_alien(&self) -> Option<bool> {
        match self.alien.network_connection_enabled {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn webauthn_extension_cred_blob_as_alien(&self) -> Option<bool> {
        match self.alien.webauthn_extension_cred_blob {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn webauthn_extension_large_blob_as_alien(&self) -> Option<bool> {
        match self.alien.webauthn_extension_large_blob {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn webauthn_virtual_authenticators_as_alien(&self) -> Option<bool> {
        match self.alien.webauthn_virtual_authenticators {
            None => None,
            Some(v) => Some(v),
        }
    }

    // setter
    fn set_chromedriver_version_as_alien(&mut self, arg: &'c str) {
        self.alien.chromedriver_version = Some(Cow::from(arg));
    }
    fn set_chromedriver_version_take_as_alien(&mut self, arg: String) {
        self.alien.chromedriver_version = Some(Cow::from(arg));
    }
    fn set_user_data_dir_as_alien(&mut self, arg: &'c str) {
        self.alien.user_data_dir = Some(Cow::from(arg));
    }
    fn set_user_data_dir_take_as_alien(&mut self, arg: String) {
        self.alien.user_data_dir = Some(Cow::from(arg));
    }
    fn set_network_connection_enabled_as_alien(&mut self, arg: bool) {
        self.alien.network_connection_enabled = Some(arg);
    }
    fn set_webauthn_extension_cred_blob_as_alien(&mut self, arg: bool) {
        self.alien.webauthn_extension_cred_blob = Some(arg);
    }
    fn set_webauthn_extension_large_blob_as_alien(&mut self, arg: bool) {
        self.alien.webauthn_extension_large_blob = Some(arg);
    }
    fn set_webauthn_virtual_authenticators_as_alien(&mut self, arg: bool) {
        self.alien.webauthn_virtual_authenticators = Some(arg);
    }
}

// ChromCapRequ //

///
/// The ChromeDriver-specific session capabilities request.
pub type ChromCapRequ<'c> = comm::CommCapRequ<ChromiumCapa<'c>>;

impl<'r, 'c1, 'c2> w3c::W3cCapRequSetter<'c1, 'c2> for ChromCapRequ<'r>
where
    'c1: 'r,
    'c2: 'r,
{
    fn mandate_as_w3c(&mut self, other: &'c1 impl w3c::W3cCapaGetter) {
        let newone = ChromiumCapa::from_other_as_w3c(other);

        self.always_match = newone;

        self.first_match
            .retain(|x| !self.always_match.is_conflict_with_as_w3c(x));
    }

    fn allow_as_w3c(&mut self, other: &'c2 impl w3c::W3cCapaGetter) -> &mut Self {
        if !self.always_match.is_conflict_with_as_w3c(other) {
            let newone = ChromiumCapa::from_other_as_w3c(other);
            self.first_match.push(newone);
        }

        self
    }
}

impl<'r, 'c1, 'c2> chrom::ChromCapRequSetter<'c1, 'c2> for ChromCapRequ<'r>
where
    'c1: 'r,
    'c2: 'r,
{
    fn mandate(&mut self, other: &'c1 (impl w3c::W3cCapaGetter + chrom::ChromiumCapaGetter)) {
        let newone = ChromiumCapa::from_other(other);

        self.always_match = newone;

        self.first_match
            .retain(|x| !self.always_match.is_conflict_with(x));
    }

    fn allow(
        &mut self,
        other: &'c2 (impl w3c::W3cCapaGetter + chrom::ChromiumCapaGetter),
    ) -> &mut Self {
        if !self.always_match.is_conflict_with(other) {
            let newone = ChromiumCapa::from_other(other);
            self.first_match.push(newone);
        }
        self
    }
}

///
/// The ChromeDriver-specific session result.
pub type ChromSessResult<'c> = comm::CommSessResult<ChromiumCapa<'c>>;

mod ser {
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    const INSIG_SNAME: &str = "-";
    const INSIG_SFLEN: usize = 1;

    // make sure two categories are always in scope
    use super::*;

    impl Serialize for ChromCapRequ<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            state.serialize_field("alwaysMatch", &self.always_match)?;
            state.serialize_field("firstMatch", &self.first_match)?;

            state.end()
        }
    }

    impl Serialize for ChromiumCapa<'_> {
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
            if let Some(v) = &self.timeouts {
                state.serialize_field("timeouts", v)?;
            }
            if let Some(v) = &self.proxy {
                state.serialize_field("proxy", v)?;
            }
            if self.wsurl.is_some() {
                state.serialize_field("webSocketUrl", &true)?;
            }
            if !self.is_insig() {
                state.serialize_field("goog:chromeOptions", &self.ext)?;
            }

            state.end()
        }
    }

    impl Serialize for ChromiumSesExtCap<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            if let Some(v) = &self.args {
                state.serialize_field("args", &v)?;
            }
            if let Some(v) = &self.binary {
                state.serialize_field("binary", &v)?;
            }
            if let Some(v) = &self.extensions {
                state.serialize_field("extensions", &v)?;
            }
            if let Some(v) = &self.prefs {
                state.serialize_field("prefs", &v)?;
            }
            if let Some(v) = &self.detach {
                state.serialize_field("detach", &v)?;
            }
            if let Some(v) = &self.debugger_address {
                state.serialize_field("debuggerAddress", &v)?;
            }
            if let Some(v) = &self.exclude_switches {
                state.serialize_field("excludeSwitches", &v)?;
            }
            if let Some(v) = &self.minidump_path {
                state.serialize_field("minidumpPath", &v)?;
            }
            if let Some(v) = &self.window_types {
                state.serialize_field("windowTypes", &v)?;
            }

            state.end()
        }
    }
} // ser

mod deser {

    // Make sure the three significant  categories
    // are as transparent as possible in all child modules,
    // like here.
    use super::*;

    // Addition note: chromedriver response for _NewSession_ not
    // as flatten as others like geckodriver, thus need more nested
    // modules here.

    // for ChromiumCapa
    mod capa {
        use serde::de::Error as DeError;
        use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};

        use super::*;

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
            Chrome,
            GoogChromeOptions,
            NetworkConnectionEnable,
            WebauthnExtensionCredBlob,
            WebauthnExtensionLargeBlob,
            WebauthnVirtualAuthenticators,
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
            "chrome",
            "goog:chromeOptions",
            "networkConnectionEnabled",
            "webauthn:extension:credBlob",
            "webauthn:extension:largeBlob",
            "webauthn:virtualAuthenticators",
            //
            "webSocketUrl",
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
                    Fields::Chrome
                } else if s == FIELD_JSON_NAMES[11] {
                    Fields::GoogChromeOptions
                } else if s == FIELD_JSON_NAMES[12] {
                    Fields::NetworkConnectionEnable
                } else if s == FIELD_JSON_NAMES[13] {
                    Fields::WebauthnExtensionCredBlob
                } else if s == FIELD_JSON_NAMES[14] {
                    Fields::WebauthnExtensionLargeBlob
                } else if s == FIELD_JSON_NAMES[15] {
                    Fields::WebauthnVirtualAuthenticators
                } else if s == FIELD_JSON_NAMES[16] {
                    Fields::WebSocketUrl
                }
                //
                else {
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
                    Fields::Chrome => FIELD_JSON_NAMES[10],
                    Fields::GoogChromeOptions => FIELD_JSON_NAMES[11],
                    Fields::NetworkConnectionEnable => FIELD_JSON_NAMES[12],
                    Fields::WebauthnExtensionCredBlob => FIELD_JSON_NAMES[13],
                    Fields::WebauthnExtensionLargeBlob => FIELD_JSON_NAMES[14],
                    Fields::WebauthnVirtualAuthenticators => FIELD_JSON_NAMES[15],
                    //
                    Fields::WebSocketUrl => FIELD_JSON_NAMES[16],
                    // Fields::Reserved(name) => panic!("unsuppoted field: {}", name),
                    xxx => panic!("the string name of field `{:?}` is unknown", xxx),
                }
            }
        }

        struct FieldVisitor;

        struct StructVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected field",)
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
            type Value = ChromiumCapa<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
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

                decl!(f0, fid Fields::BrowserName, vtype &'de str);
                decl!(f1, fid Fields::BrowserVersion, vtype &'de str);
                decl!(f2, fid Fields::PlatformName, vtype &'de str);
                decl!(f3, fid Fields::AcceptInsecureCerts, vtype bool);
                decl!(f4, fid Fields::PageLoadStrategy, vtype &'de str);
                decl!(f5, fid Fields::Proxy, vtype comm::Proxy<'de>);
                decl!(f6, fid Fields::WindowRect, vtype bool);
                decl!(f7, fid Fields::Timeouts, vtype comm::Timeouts);
                decl!(f8, fid Fields::StrictFileInteractability, vtype bool);
                decl!(f9, fid Fields::UnhandledPromptBehavior, vtype &'de str);
                decl!(f99, fid Fields::WebSocketUrl, vtype &'de str);
                decl!(f10, fid Fields::Chrome, vtype alien::Chrome<'de>);
                decl!(f11, fid Fields::GoogChromeOptions, vtype ChromiumSesExtCap<'de>);
                decl!(f12, fid Fields::NetworkConnectionEnable, vtype alien::NetworkConnectionEnable);
                decl!(f13, fid Fields::WebauthnExtensionCredBlob, vtype alien::WebauthnExtensionCredBlob);
                decl!(f14, fid Fields::WebauthnExtensionLargeBlob, vtype alien::WebauthnExtensionLargeBlob);
                decl!(f15, fid Fields::WebauthnVirtualAuthenticators, vtype alien::WebauthnVirtualAuthenticators);

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                macro_rules! give_value_to {
                    ($field_obj:ident) => {
                        match $field_obj.0 {
                            None => $field_obj.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field($field_obj.1)),
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
                        Fields::Chrome => give_value_to!(f10),
                        Fields::GoogChromeOptions => give_value_to!(f11),
                        Fields::NetworkConnectionEnable => give_value_to!(f12),
                        Fields::WebauthnExtensionCredBlob => give_value_to!(f13),
                        Fields::WebauthnExtensionLargeBlob => give_value_to!(f14),
                        Fields::WebauthnVirtualAuthenticators => give_value_to!(f15),
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

                    unwrap_or_missing_error!(
                        f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15
                    );

                    let mut ret = ChromiumCapa::default();

                    ret.set_browser_name(f0);
                    ret.set_browser_version(f1);
                    ret.set_platform_name(f2);
                    ret.set_accept_insecure_certs(f3);
                    ret.set_page_load_strategy(f4);
                    ret.proxy = Some(f5);
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

                    // 'f11 != 'de, thus completely move
                    ret.ext = f11;

                    // 'f10 != 'de, thus partially move
                    ret.alien.chromedriver_version = Some(f10.chromedriver_version);
                    ret.alien.user_data_dir = Some(f10.user_data_dir);
                    ret.set_network_connection_enabled_as_alien(f12);
                    ret.set_webauthn_extension_cred_blob_as_alien(f13);
                    ret.set_webauthn_extension_large_blob_as_alien(f14);
                    ret.set_webauthn_virtual_authenticators_as_alien(f15);

                    Ok(ret)
                }
            }
        }

        #[cfg(target_family = "windows")]
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = ChromiumCapa<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
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
                decl!(f10, fid Fields::Chrome, vtype alien::Chrome<'de>);
                decl!(f11, fid Fields::GoogChromeOptions, vtype ChromiumSesExtCap<'de>);
                decl!(f12, fid Fields::NetworkConnectionEnable, vtype alien::NetworkConnectionEnable);
                decl!(f13, fid Fields::WebauthnExtensionCredBlob, vtype alien::WebauthnExtensionCredBlob);
                decl!(f14, fid Fields::WebauthnExtensionLargeBlob, vtype alien::WebauthnExtensionLargeBlob);
                decl!(f15, fid Fields::WebauthnVirtualAuthenticators, vtype alien::WebauthnVirtualAuthenticators);

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                macro_rules! give_value_to {
                    ($field_obj:ident) => {
                        match $field_obj.0 {
                            None => $field_obj.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field($field_obj.1)),
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
                        Fields::Chrome => give_value_to!(f10),
                        Fields::GoogChromeOptions => give_value_to!(f11),
                        Fields::NetworkConnectionEnable => give_value_to!(f12),
                        Fields::WebauthnExtensionCredBlob => give_value_to!(f13),
                        Fields::WebauthnExtensionLargeBlob => give_value_to!(f14),
                        Fields::WebauthnVirtualAuthenticators => give_value_to!(f15),
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

                    unwrap_or_missing_error!(
                        f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15
                    );

                    let mut ret = ChromiumCapa::default();

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

                    // 'f11 != 'de, thus completely move
                    ret.ext = f11;

                    // 'f10 != 'de, thus partially move
                    ret.alien.chromedriver_version = Some(f10.chromedriver_version);
                    ret.alien.user_data_dir = Some(f10.user_data_dir);
                    ret.set_network_connection_enabled_as_alien(f12);
                    ret.set_webauthn_extension_cred_blob_as_alien(f13);
                    ret.set_webauthn_extension_large_blob_as_alien(f14);
                    ret.set_webauthn_virtual_authenticators_as_alien(f15);

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
        impl<'de> Deserialize<'de> for ChromiumCapa<'de> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct("ChromiumCapa", FIELD_JSON_NAMES, StructVisitor)
            }
        }
    } // capa

    // for ChromiumSesExtCap
    mod exts {
        use serde::de::{
            Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor,
        };
        use std::borrow::Cow;

        use super::*;

        #[derive(Debug)]
        enum Fields {
            Args,
            Binary,
            Extensions,
            Detach,
            DebuggerAddress,
            ExcludeSwitches,
            MinidumpPath,
            WindowTypes,
            Reserved(String),
        }

        const FIELD_JSON_NAMES: &[&str] = &[
            "args",
            "binary",
            "extensions",
            "detach",
            "debuggerAddress",
            "excludeSwitches",
            "minidumpPath",
            "windowTypes",
        ];

        impl Fields {
            fn from_str(s: &str) -> Self {
                if s == FIELD_JSON_NAMES[0] {
                    Fields::Args
                } else if s == FIELD_JSON_NAMES[1] {
                    Fields::Binary
                } else if s == FIELD_JSON_NAMES[2] {
                    Fields::Extensions
                } else if s == FIELD_JSON_NAMES[3] {
                    Fields::Detach
                } else if s == FIELD_JSON_NAMES[4] {
                    Fields::DebuggerAddress
                } else if s == FIELD_JSON_NAMES[5] {
                    Fields::ExcludeSwitches
                } else if s == FIELD_JSON_NAMES[6] {
                    Fields::MinidumpPath
                } else if s == FIELD_JSON_NAMES[7] {
                    Fields::WindowTypes
                } else {
                    Fields::Reserved(s.to_string())
                }
            }
            fn to_sstr(&self) -> &'static str {
                match self {
                    Fields::Args => FIELD_JSON_NAMES[0],
                    Fields::Binary => FIELD_JSON_NAMES[1],
                    Fields::Extensions => FIELD_JSON_NAMES[2],
                    Fields::Detach => FIELD_JSON_NAMES[3],
                    Fields::DebuggerAddress => FIELD_JSON_NAMES[4],
                    Fields::ExcludeSwitches => FIELD_JSON_NAMES[5],
                    Fields::MinidumpPath => FIELD_JSON_NAMES[6],
                    Fields::WindowTypes => FIELD_JSON_NAMES[7],
                    _ => panic!("unsuppoted operation"),
                }
            }
        }

        struct FieldVisitor;

        struct StructVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected field")
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
            type Value = ChromiumSesExtCap<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
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
                decl!(f0, fid Fields::Args, vtype Vec<&'de str>);
                decl!(f1, fid Fields::Binary, vtype &'de str);
                decl!(f2, fid Fields::Extensions, vtype Vec<&'de str>);
                decl!(f3, fid Fields::Detach, vtype bool);
                decl!(f4, fid Fields::DebuggerAddress, vtype &'de str);
                decl!(f5, fid Fields::ExcludeSwitches, vtype Vec<&'de str>);
                decl!(f6, fid Fields::MinidumpPath, vtype &'de str);
                decl!(f7, fid Fields::WindowTypes, vtype Vec<&'de str>);
                // FIXME: prefs support absence

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                macro_rules! give_value_to {
                    ($field_obj:ident) => {
                        match $field_obj.0 {
                            None => $field_obj.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field($field_obj.1)),
                        }
                    };
                }

                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::Args => give_value_to!(f0),
                        Fields::Binary => give_value_to!(f1),
                        Fields::Extensions => give_value_to!(f2),
                        Fields::Detach => give_value_to!(f3),
                        Fields::DebuggerAddress => give_value_to!(f4),
                        Fields::ExcludeSwitches => give_value_to!(f5),
                        Fields::MinidumpPath => give_value_to!(f6),
                        Fields::WindowTypes => give_value_to!(f7),
                        _ => {
                            map.next_value::<IgnoredAny>()?; // advance 1 token

                            crate::dbgmsg!("SKIP...key {:?} and its value", key);
                        }
                    }
                }

                if let Some(e) = is_dup {
                    Err(e)
                } else {
                    // missing field is not an error
                    Ok(ChromiumSesExtCap {
                        args: f0.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        binary: f1.0.map(|v| Cow::from(v)),
                        extensions: f2.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        detach: f3.0,
                        debugger_address: f4.0.map(|v| Cow::from(v)),
                        exclude_switches: f5.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        minidump_path: f6.0.map(|v| Cow::from(v)),
                        window_types: f7.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        prefs: Default::default(),
                    })
                }
            }
        }

        #[cfg(target_family = "windows")]
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = ChromiumSesExtCap<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "unexpected struct")
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
                decl!(f0, fid Fields::Args, vtype Vec<&'de str>);
                decl!(f1, fid Fields::Binary, vtype &'de str);
                decl!(f2, fid Fields::Extensions, vtype Vec<&'de str>);
                decl!(f3, fid Fields::Detach, vtype bool);
                decl!(f4, fid Fields::DebuggerAddress, vtype String);
                decl!(f5, fid Fields::ExcludeSwitches, vtype Vec<&'de str>);
                decl!(f6, fid Fields::MinidumpPath, vtype &'de str);
                decl!(f7, fid Fields::WindowTypes, vtype Vec<&'de str>);
                // FIXME: prefs support absence

                // none or wrap a duplication error
                let mut is_dup: Option<A::Error> = None;

                macro_rules! give_value_to {
                    ($field_obj:ident) => {
                        match $field_obj.0 {
                            None => $field_obj.0 = Some(map.next_value()?),
                            _ => is_dup = Some(DeError::duplicate_field($field_obj.1)),
                        }
                    };
                }

                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::Args => give_value_to!(f0),
                        Fields::Binary => give_value_to!(f1),
                        Fields::Extensions => give_value_to!(f2),
                        Fields::Detach => give_value_to!(f3),
                        Fields::DebuggerAddress => give_value_to!(f4),
                        Fields::ExcludeSwitches => give_value_to!(f5),
                        Fields::MinidumpPath => give_value_to!(f6),
                        Fields::WindowTypes => give_value_to!(f7),
                        _ => {
                            map.next_value::<IgnoredAny>()?; // advance 1 token

                            crate::dbgmsg!("SKIP...key {:?} and its value", key);
                        }
                    }
                }

                if let Some(e) = is_dup {
                    Err(e)
                } else {
                    // missing field is not an error
                    Ok(ChromiumSesExtCap {
                        args: f0.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        binary: f1.0.map(|v| Cow::from(v)),
                        extensions: f2.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        detach: f3.0,
                        debugger_address: f4.0.map(|v| Cow::from(v)),
                        exclude_switches: f5.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        minidump_path: f6.0.map(|v| Cow::from(v)),
                        window_types: f7.0.map(|v| v.iter().map(|x| Cow::from(*x)).collect()),
                        prefs: Default::default(),
                    })
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

        //  trivial
        impl<'de> Deserialize<'de> for ChromiumSesExtCap<'de> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct(
                    "ChromiumSesExtCap",
                    FIELD_JSON_NAMES,
                    StructVisitor,
                )
            }
        }
    } // exts

    // tests
} // deser

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
            let requ = ChromCapRequ::default();

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _1() {
            // a tolerant match, this is the least condition for a
            // valid session, see #7.2-3-2.

            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

            requ.allow_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), true);
            assert_eq!(requ.first_match.len(), 1);
        }

        #[test]
        fn _11() {
            // a tolerant match, this is the least condition for a
            // valid session, see #7.2-3-2.

            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

            requ.allow_as_w3c(&capa);

            let capa = ChromiumCapa::default();

            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.always_match.platform_name().is_none(), true);
            assert_eq!(requ.always_match.is_insig_as_w3c(), true);
            assert_eq!(requ.first_match.len(), 1);
        }

        #[test]
        fn _2() {
            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            let mut capa = ChromiumCapa::default();
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
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            let mut capa1 = ChromiumCapa::default();
            let mut capa2 = ChromiumCapa::default();
            let mut capa3 = ChromiumCapa::default();
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
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);
            let mut capa = ChromiumCapa::default();
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
            let mut requ = ChromCapRequ::default();

            let mut capa = ChromiumCapa::default();
            capa.set_browser_name("edge");
            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            let mut capa = ChromiumCapa::default();
            capa.set_browser_name("edge");
            capa.set_platform_name("win");
            requ.allow_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_none(), true);
            assert_eq!(requ.first_match.len(), 2);

            let mut capa = ChromiumCapa::default();
            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.browser_name().is_some(), true);
            assert_eq!(requ.always_match.browser_name().unwrap(), "firefox");
            assert_eq!(requ.first_match.len(), 0);
        }
    } // w3c compliance

    #[cfg(test)]
    mod chrom_compl {
        use super::*;
        use chrom::{ChromCapRequSetter, ChromiumCapaGetter, ChromiumCapaSetter};

        #[test]
        fn _1() {
            // bare initialization

            let requ = ChromCapRequ::default();

            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.always_match.is_insig(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _2() {
            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

            requ.allow(&capa);

            assert_eq!(requ.first_match.len(), 1);
        }

        #[test]
        fn _3() {
            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

            requ.allow(&capa);
            requ.allow(&capa);

            assert_eq!(requ.first_match.len(), 2);
        }

        #[test]
        fn _4() {
            // `binary` field
            // exact one binary
            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();
            let mut capa1 = ChromiumCapa::default();
            let mut capa2 = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();
            let mut capa1 = ChromiumCapa::default();
            let mut capa2 = ChromiumCapa::default();
            let mut capa3 = ChromiumCapa::default();

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
            let mut requ = ChromCapRequ::default();

            let mut capa = ChromiumCapa::default();
            capa.set_binary("/path/to/binary_v1.0");
            requ.allow(&capa);

            let mut capa = ChromiumCapa::default();
            capa.set_binary("/path/to/binary_v2.0");
            requ.allow(&capa);

            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.first_match.len(), 2);

            let mut capa = ChromiumCapa::default();
            capa.set_binary("/path/to/binary_v3.0");
            requ.mandate(&capa);

            assert_eq!(requ.always_match.binary().is_some(), true);
            assert_eq!(requ.first_match.len(), 0);
        }
    } // chrom compliance

    #[cfg(test)]
    mod together_compl {
        use super::*;
        use chrom::{ChromCapRequSetter, ChromiumCapaGetter, ChromiumCapaSetter};
        use w3c::{W3cCapRequSetter, W3cCapaGetter, W3cCapaSetter};

        #[test]
        fn _0() {
            // use as w3c conf works just partly

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_browser_name("firefox");
            capa.set_browser_version("106.0.0");
            capa.set_binary("/path/to/firefox");
            capa.add_args("--headless");

            requ.mandate_as_w3c(&capa);

            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            // all chrom-* are insig:
            assert_eq!(requ.always_match.is_insig(), true);
            //  no bin:
            assert_eq!(requ.always_match.binary().is_none(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _1() {
            // use as chrom conf works fine

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_browser_name("firefox");
            capa.set_browser_version("106.0.0");
            capa.set_binary("/path/to/firefox");
            capa.add_args("--headless");

            requ.mandate(&capa); // use chrom own impl mandate

            assert_eq!(requ.always_match.is_insig_as_w3c(), false);
            // not all chrom-* are insig:
            assert_eq!(requ.always_match.is_insig(), false);
            // there is bin
            assert_eq!(requ.always_match.binary().is_some(), true);
            assert_eq!(requ.first_match.len(), 0);
        }

        #[test]
        fn _2() {
            // use as chrom conf works fine

            let mut requ = ChromCapRequ::default();
            let mut capa1 = ChromiumCapa::default();
            let mut capa2 = ChromiumCapa::default();
            let mut capa3 = ChromiumCapa::default();

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
        use chrom::ChromCapRequSetter;
        use serde_test::assert_ser_tokens;
        use serde_test::Token;
        use w3c::{W3cCapRequSetter, W3cCapaSetter};

        #[test]
        fn _1() {
            let requ = ChromCapRequ::default();

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
            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

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

            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

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

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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
        fn _44_2() {
            // set 1 field

            let mut requ = ChromCapRequ::default();
            let capa = ChromiumCapa::default();

            requ.allow_as_w3c(&capa);

            let mut capa = ChromiumCapa::default();

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
                    Token::Seq { len: Some(1) },
                    Token::Struct { name: "-", len: 1 },
                    Token::StructEnd,
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );

            // json str is correct?
            let actual = serde_json::to_string(&requ).unwrap();
            let expect = r#"{"alwaysMatch":{"timeouts":{"script":1234,"pageLoad":300000,"implicit":0}},"firstMatch":[{}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _444() {
            // set proxy field

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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
        fn _5() {
            // set two fields

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

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

            let mut requ = ChromCapRequ::default();

            let mut capa = ChromiumCapa::default();
            capa.set_browser_name("firefox");
            requ.mandate_as_w3c(&capa);

            let mut capa = ChromiumCapa::default();
            capa.set_platform_name("linux");
            requ.allow_as_w3c(&capa);

            let mut capa = ChromiumCapa::default();
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

            let mut requ = ChromCapRequ::default();

            let mut capa = ChromiumCapa::default();
            capa.set_browser_name("firefox");
            capa.add_args("--headless");
            requ.mandate(&capa);

            let mut capa = ChromiumCapa::default();
            capa.set_platform_name("linux");
            capa.set_binary("/usr/bin/firefox");
            requ.allow(&capa);

            let mut capa = ChromiumCapa::default();
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
                    Token::Str("goog:chromeOptions"),
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
                    Token::Str("goog:chromeOptions"),
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
                    Token::Str("goog:chromeOptions"),
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
            let expect = r#"{"alwaysMatch":{"browserName":"firefox","goog:chromeOptions":{"args":["--headless"]}},"firstMatch":[{"platformName":"linux","goog:chromeOptions":{"binary":"/usr/bin/firefox"}},{"platformName":"win","goog:chromeOptions":{"binary":"/c/program/firefox"}}]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _8() {
            // `prefs` field

            let mut requ = ChromCapRequ::default();
            let mut capa = ChromiumCapa::default();

            capa.set_browser_name("chrome");
            capa.set_binary("/path/to/chrome");
            capa.add_prefs("profile.name", "A Real Human");
            capa.add_prefs("profile.avatar_index", "16");

            requ.mandate(&capa);

            // token stream is correct?
            assert_ser_tokens(
                &requ,
                &[
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("alwaysMatch"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("browserName"),
                    Token::Str("chrome"),
                    Token::Str("goog:chromeOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("binary"),
                    Token::Str("/path/to/chrome"),
                    Token::Str("prefs"),
                    Token::Map { len: Some(2) },
                    Token::Str("profile.avatar_index"), // backing is type of order map
                    Token::U32(16),
                    Token::Str("profile.name"),
                    Token::Str("A Real Human"),
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
            let expect = r#"{"alwaysMatch":{"browserName":"chrome","goog:chromeOptions":{"binary":"/path/to/chrome","prefs":{"profile.avatar_index":16,"profile.name":"A Real Human"}}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }

        #[test]
        fn _88() {
            // `prefs` field, use non-std way

            let mut requ = ChromCapRequ::default();

            let mut capa = ChromiumCapa::default();
            capa.set_proxy_type("manual");
            capa.set_socks_version(5);
            capa.set_socks_proxy("127.0.0.1:1080");
            capa.add_prefs("browser.check_default_browser", "false");
            capa.add_prefs("dns_prefetching.enabled", "true");
            capa.add_prefs("profile.default_content_settings.geolocation", "0");
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
                    Token::Str("goog:chromeOptions"),
                    Token::Struct { name: "-", len: 1 },
                    Token::Str("prefs"),
                    Token::Map { len: Some(3) },
                    Token::Str("browser.check_default_browser"),
                    Token::Bool(false),
                    Token::Str("dns_prefetching.enabled"),
                    Token::Bool(true),
                    Token::Str("profile.default_content_settings.geolocation"),
                    Token::U32(0),
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
            let expect = r#"{"alwaysMatch":{"proxy":{"proxyType":"manual","socksProxy":"127.0.0.1:1080","socksVersion":5},"goog:chromeOptions":{"prefs":{"browser.check_default_browser":false,"dns_prefetching.enabled":true,"profile.default_content_settings.geolocation":0}}},"firstMatch":[]}"#;
            assert_eq!(actual, expect);
        }
    } // ser

    #[cfg(test)]
    mod deser {
        use super::*;
        use w3c::W3cSessResultGetter;

        #[test]
        fn _0() {
            // a response to default NewSession command, from chrome 106
            let resp_str = r#"{"value":{"capabilities":{"acceptInsecureCerts":false,"browserName":"chrome","browserVersion":"107.0.5304.0","chrome":{"chromedriverVersion":"107.0.5304.0 (5d7b1fc9cb7103d9c82eed647cf4be38cf09738b-refs/heads/main@{#1047731})","userDataDir":"/tmp/.org.chromium.Chromium.TS0afe"},"goog:chromeOptions":{"debuggerAddress":"localhost:43515"},"networkConnectionEnabled":false,"pageLoadStrategy":"normal","platformName":"linux","proxy":{},"setWindowRect":true,"strictFileInteractability":false,"timeouts":{"implicit":0,"pageLoad":300000,"script":30000},"unhandledPromptBehavior":"dismiss and notify","webauthn:extension:credBlob":true,"webauthn:extension:largeBlob":true,"webauthn:virtualAuthenticators":true},"sessionId":"61314ad6a23266bdd381cfb7b584e8d3"}}"#;

            let obj = serde_json::from_slice::<ChromSessResult>(resp_str.as_bytes()).unwrap();

            assert_eq!(obj.session_id(), "61314ad6a23266bdd381cfb7b584e8d3");
        }
    } // deser
} // utst
