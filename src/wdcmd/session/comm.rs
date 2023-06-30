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

use std::borrow::Cow;

use super::w3c;

#[derive(Default, Debug)]
pub(crate) struct Proxy<'c> {
    pub(crate) proxy_type: Cow<'c, str>,
    pub(crate) proxy_autoconfig_url: Cow<'c, str>,
    pub(crate) ftp_proxy: Cow<'c, str>,
    pub(crate) http_proxy: Cow<'c, str>,
    pub(crate) no_proxy: Vec<Cow<'c, str>>,
    pub(crate) ssl_proxy: Cow<'c, str>,
    pub(crate) socks_proxy: Cow<'c, str>,
    pub(crate) socks_version: u8,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Timeouts {
    pub(crate) script: u32,
    #[serde(rename = "pageLoad")]
    pub(crate) page_load: u32,
    pub(crate) implicit: u32,
}

#[derive(Default, Debug)]
pub struct NoAlien;

///
/// The [capabilities](https://w3c.github.io/webdriver/#capabilities)
/// that any common WebDriver session would have.
#[derive(Default, Debug)]
pub struct CommCapa<'capa, X, A, L = NoAlien>
where
    X: Default,
    A: Default,
    L: Default,
{
    pub(crate) browser_name: Option<Cow<'capa, str>>,
    pub(crate) browser_version: Option<Cow<'capa, str>>,
    pub(crate) platform_name: Option<Cow<'capa, str>>,
    pub(crate) accept_insecure_certs: Option<bool>,
    pub(crate) page_load_strategy: Option<Cow<'capa, str>>,
    pub(crate) proxy: Option<Proxy<'capa>>, // maybe flatten in the future
    pub(crate) window_rect: Option<bool>,
    pub(crate) timeouts: Option<Timeouts>,
    pub(crate) strict_file_interactability: Option<bool>,
    pub(crate) unhandled_prompt_behavior: Option<Cow<'capa, str>>,
    pub(crate) wsurl: Option<Cow<'capa, str>>, // bidi spec additional
    pub(crate) ext: X,
    #[allow(dead_code)] // FIXME: should not exist
    pub(crate) add: A,
    pub(crate) alien: L,
}

impl<X: Default, A: Default, L: Default> w3c::W3cCapaGetter for CommCapa<'_, X, A, L> {
    fn browser_name(&self) -> Option<&str> {
        match self.browser_name.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn browser_version(&self) -> Option<&str> {
        match self.browser_version.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn platform_name(&self) -> Option<&str> {
        match self.platform_name.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn accept_insecure_certs(&self) -> Option<bool> {
        match self.accept_insecure_certs {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn page_load_strategy(&self) -> Option<&str> {
        match self.page_load_strategy.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    //
    fn proxy_type(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.proxy_type.as_ref()),
        }
    }
    fn proxy_autoconfig_url(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.proxy_autoconfig_url.as_ref()),
        }
    }
    fn ftp_proxy(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.ftp_proxy.as_ref()),
        }
    }
    fn http_proxy(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.http_proxy.as_ref()),
        }
    }
    fn no_proxy(&self) -> Option<Vec<&str>> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.no_proxy.iter().map(|x| x.as_ref()).collect()),
        }
    }
    fn ssl_proxy(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.ssl_proxy.as_ref()),
        }
    }
    fn socks_proxy(&self) -> Option<&str> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.socks_proxy.as_ref()),
        }
    }
    fn socks_version(&self) -> Option<u8> {
        match self.proxy.as_ref() {
            None => None,
            Some(v) => Some(v.socks_version),
        }
    }
    //
    fn window_rect(&self) -> Option<bool> {
        match self.window_rect {
            None => None,
            Some(v) => Some(v),
        }
    }
    //
    fn timeouts_script(&self) -> Option<u32> {
        match self.timeouts.as_ref() {
            None => None,
            Some(v) => Some(v.script),
        }
    }
    fn timeouts_page_load(&self) -> Option<u32> {
        match self.timeouts.as_ref() {
            None => None,
            Some(v) => Some(v.page_load),
        }
    }
    fn timeouts_implicit(&self) -> Option<u32> {
        match self.timeouts.as_ref() {
            None => None,
            Some(v) => Some(v.implicit),
        }
    }
    //
    fn strict_file_interactability(&self) -> Option<bool> {
        match self.strict_file_interactability {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn unhandled_prompt_behavior(&self) -> Option<&str> {
        match self.unhandled_prompt_behavior.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn wsurl(&self) -> Option<&str> {
        match self.wsurl.as_ref() {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn is_insig_as_w3c(&self) -> bool {
        self.browser_name().is_none()
            && self.browser_version().is_none()
            && self.platform_name().is_none()
            && self.accept_insecure_certs().is_none()
            && self.page_load_strategy().is_none()
            && self.proxy_type().is_none()
            && self.timeouts_script().is_none()
            && self.strict_file_interactability().is_none()
            && self.unhandled_prompt_behavior().is_none()
            && self.wsurl().is_none()
    }
}

impl<'r, 'c, X: Default, A: Default, L: Default> w3c::W3cCapaSetter<'c> for CommCapa<'r, X, A, L>
where
    'c: 'r,
{
    // setters

    fn set_browser_name(&mut self, arg: &'c str) {
        self.browser_name = Some(Cow::from(arg))
    }
    fn set_browser_name_owned(&mut self, arg: &str) {
        self.browser_name = Some(Cow::from(arg.to_string()))
    }
    fn set_browser_name_take(&mut self, arg: String) {
        self.browser_name = Some(Cow::from(arg))
    }

    fn set_browser_version(&mut self, arg: &'c str) {
        self.browser_version = Some(Cow::from(arg))
    }
    fn set_browser_version_take(&mut self, arg: String) {
        self.browser_version = Some(Cow::from(arg))
    }
    fn set_browser_version_owned(&mut self, arg: &str) {
        self.browser_version = Some(Cow::from(arg.to_string()))
    }

    fn set_platform_name(&mut self, arg: &'c str) {
        self.platform_name = Some(Cow::from(arg))
    }
    fn set_platform_name_take(&mut self, arg: String) {
        self.platform_name = Some(Cow::from(arg))
    }
    fn set_platform_name_owned(&mut self, arg: &str) {
        self.platform_name = Some(Cow::from(arg.to_string()))
    }

    fn set_accept_insecure_certs(&mut self, arg: bool) {
        self.accept_insecure_certs = Some(arg)
    }
    fn set_page_load_strategy(&mut self, arg: &'c str) {
        self.page_load_strategy = Some(Cow::from(arg))
    }
    fn set_page_load_strategy_take(&mut self, arg: String) {
        self.page_load_strategy = Some(Cow::from(arg))
    }
    fn set_page_load_strategy_owned(&mut self, arg: &str) {
        self.page_load_strategy = Some(Cow::from(arg.to_string()))
    }
    //
    fn set_proxy_type(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    proxy_type: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.proxy_type = Cow::from(arg),
        }
    }
    fn set_proxy_autoconfig_url(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    proxy_autoconfig_url: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.proxy_autoconfig_url = Cow::from(arg),
        }
    }
    fn set_ftp_proxy(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    ftp_proxy: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.ftp_proxy = Cow::from(arg),
        }
    }
    fn set_http_proxy(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    http_proxy: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.http_proxy = Cow::from(arg),
        }
    }
    fn set_no_proxy(&mut self, arg: Vec<&'c str>) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    no_proxy: arg.iter().map(|x| Cow::from(*x)).collect(),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.no_proxy = arg.iter().map(|x| Cow::from(*x)).collect(),
        }
    }
    fn add_no_proxy(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    no_proxy: vec![Cow::from(arg)],
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.no_proxy.push(Cow::from(arg)),
        }
    }
    fn set_ssl_proxy(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    ssl_proxy: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.ssl_proxy = Cow::from(arg),
        }
    }
    fn set_socks_proxy(&mut self, arg: &'c str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    socks_proxy: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.socks_proxy = Cow::from(arg),
        }
    }
    fn set_socks_proxy_owned(&mut self, arg: &str) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'_> {
                    socks_proxy: Cow::from(arg.to_string()),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.socks_proxy = Cow::from(arg.to_string()),
        }
    }
    fn set_socks_proxy_take(&mut self, arg: String) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'_> {
                    socks_proxy: Cow::from(arg),
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.socks_proxy = Cow::from(arg),
        }
    }
    fn set_socks_version(&mut self, arg: u8) {
        match self.proxy.as_mut() {
            None => {
                let newone = Proxy::<'c> {
                    socks_version: arg,
                    ..Default::default()
                };
                self.proxy = Some(newone);
            }
            Some(v) => v.socks_version = arg,
        }
    }
    //
    fn set_window_rect(&mut self, arg: bool) {
        self.window_rect = Some(arg)
    }
    //
    fn set_timeouts_script(&mut self, arg: u32) {
        match self.timeouts.as_mut() {
            None => {
                let newone = Timeouts {
                    script: arg,
                    ..Default::default()
                };
                self.timeouts = Some(newone);
            }
            Some(v) => v.script = arg,
        }
    }
    fn set_timeouts_page_load(&mut self, arg: u32) {
        match self.timeouts.as_mut() {
            None => {
                let newone = Timeouts {
                    page_load: arg,
                    ..Default::default()
                };
                self.timeouts = Some(newone);
            }
            Some(v) => v.page_load = arg,
        }
    }
    fn set_timeouts_implicit(&mut self, arg: u32) {
        match self.timeouts.as_mut() {
            None => {
                let newone = Timeouts {
                    implicit: arg,
                    ..Default::default()
                };
                self.timeouts = Some(newone);
            }
            Some(v) => v.implicit = arg,
        }
    }
    //
    fn set_strict_file_interactability(&mut self, arg: bool) {
        self.strict_file_interactability = Some(arg)
    }
    fn set_unhandled_prompt_behavior(&mut self, arg: &'c str) {
        self.unhandled_prompt_behavior = Some(Cow::from(arg))
    }
    fn set_unhandled_prompt_behavior_take(&mut self, arg: String) {
        self.unhandled_prompt_behavior = Some(Cow::from(arg))
    }
    fn set_unhandled_prompt_behavior_owned(&mut self, arg: &str) {
        self.unhandled_prompt_behavior = Some(Cow::from(arg.to_string()))
    }
    //
    fn set_wsurl(&mut self, arg: &'c str) {
        self.wsurl = Some(Cow::from(arg))
    }
    fn set_wsurl_take(&mut self, arg: String) {
        self.wsurl = Some(Cow::from(arg))
    }
    fn enable_bidi(&mut self) {
        self.wsurl = Some(Cow::from(""))
    }
}

///
/// The [request](https://w3c.github.io/webdriver/#dfn-capabilities-processing)
/// that any common WebDriver's _NewSession_ command would ask for.
///
/// #dfn-capabilities-processing,2
/// #dfn-capabilities-processing,3
#[derive(Default, Debug)]
pub struct CommCapRequ<C>
where
    C: Default,
{
    pub(crate) always_match: C,
    pub(crate) first_match: Vec<C>,
}

///
/// The [response](https://w3c.github.io/webdriver/#dfn-send-a-response)
/// that any common WebDriver would send after creating a session.
#[derive(Default, Debug, serde::Deserialize)]
pub struct CommSessResult<C>
where
    C: w3c::W3cCapaGetter,
{
    pub(crate) value: CommSessResultVal<C>,
}

#[derive(Default, Debug)]
pub(crate) struct CommSessResultVal<C>
where
    C: w3c::W3cCapaGetter,
{
    session_id: String,

    //
    // Why not more specific, like `capabilities: CommCapa<X,A,L>?`
    // Rationale: due to serde's map-based deserialization mechanism, if
    // use CommCapa, structure is fixed for all drivers, which means
    // the map we got when deserializing is fixed, for CommCapa<X,A,L>, this
    // means all standard-defined capabilities can be deserialize immediately,
    // but as for its X,A,L, deserialization should be delegated to themselves.
    // this works theoretically but not practically at all: gecko's
    // session result is a nearly 100% flatten map, chrome's one is the
    // combination of flatten map and enclosed map, and nearly 50%-50%
    // for each of them, it is impossible to unify these characteristics, and
    // even some of chrome capabilities' placement are more unpredictable than
    // gecko's ones. Hence we strictly follow WebDriver standard's rules
    // (#dfn-new-sessions - 11): all details leave to implementation, which
    // means generic <C>.
    // Note that, this workaround has one major drawback: all standard-defined
    // capabilities like browserName, browserVersion are hidden here. Although
    // this is considered "drawback", it could be fixed by the future
    // WebDriver standard.
    pub(crate) capabilities: C,
}

impl Default for Timeouts {
    fn default() -> Self {
        Timeouts {
            script: 30_000,
            page_load: 300_000,
            implicit: 0,
        }
    }
}

impl<C> w3c::W3cSessResultGetter for CommSessResult<C>
where
    C: w3c::W3cCapaGetter,
{
    fn session_id(&self) -> &str {
        &self.value.session_id
    }

    fn wsurl(&self) -> Option<&str> {
        self.value.capabilities.wsurl()
    }
}

use std::collections::BTreeMap;

// a BTreeMap wrapper which supports customizable serialization for Map's value.
#[derive(Default, Debug)]
pub(crate) struct VarValTypeMap<K, V> {
    pub(crate) map: BTreeMap<K, V>,
}

mod ser {
    use super::*;
    use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};
    use std::borrow::Cow;

    impl Serialize for Proxy<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Proxy", 8)?; // FIXME: prefer accurate len

            if self.proxy_type.len() != 0 {
                state.serialize_field("proxyType", &self.proxy_type)?;
            }
            if self.proxy_autoconfig_url.len() != 0 {
                state.serialize_field("proxyAutoconfigUrl", &self.proxy_autoconfig_url)?;
            }
            if self.ftp_proxy.len() != 0 {
                state.serialize_field("ftpProxy", &self.ftp_proxy)?;
            }
            if self.http_proxy.len() != 0 {
                state.serialize_field("httpProxy", &self.http_proxy)?;
            }
            if self.no_proxy.len() != 0 {
                state.serialize_field("noProxy", &self.no_proxy)?;
            }
            if self.ssl_proxy.len() != 0 {
                state.serialize_field("sslProxy", &self.ssl_proxy)?;
            }
            if self.socks_proxy.len() != 0 {
                state.serialize_field("socksProxy", &self.socks_proxy)?;
            }
            if self.socks_version != 0 {
                state.serialize_field("socksVersion", &self.socks_version)?;
            }

            state.end()
        }
    }

    impl Serialize for VarValTypeMap<Cow<'_, str>, Cow<'_, str>> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_map(Some(self.map.len()))?;

            for (key, value) in self.map.iter() {
                if value == "true" {
                    state.serialize_entry(key, &true)?;
                } else if value == "false" {
                    state.serialize_entry(key, &false)?;
                } else if value.parse::<u32>().is_ok() {
                    state.serialize_entry(key, &value.parse::<u32>().unwrap())?;
                } else {
                    state.serialize_entry(key, value)?;
                }
            }

            state.end()
        }
    }
} // ser

mod deser_proxyref {
    use super::*;
    use serde::de::{Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor};

    struct FieldVisitor;
    struct StructVisitor;

    const FIELD_JSON_NAMES: &[&str] = &[
        "proxyType",
        "proxyAutoconfigUrl",
        "ftpProxy",
        "httpProxy",
        "noProxy",
        "sslProxy",
        "socksProxy",
        "socksVersion",
    ];

    #[derive(Debug)]
    enum Fields {
        ProxyType,
        ProxyAutoconfigUrl,
        FtpProxy,
        HttpProxy,
        NoProxy,
        SslProxy,
        SocksProxy,
        SocksVersion,
        Reserved(String),
    }

    impl Fields {
        fn from_str(s: &str) -> Self {
            if s == FIELD_JSON_NAMES[0] {
                Fields::ProxyType
            } else if s == FIELD_JSON_NAMES[1] {
                Fields::ProxyAutoconfigUrl
            } else if s == FIELD_JSON_NAMES[2] {
                Fields::FtpProxy
            } else if s == FIELD_JSON_NAMES[3] {
                Fields::HttpProxy
            } else if s == FIELD_JSON_NAMES[4] {
                Fields::NoProxy
            } else if s == FIELD_JSON_NAMES[5] {
                Fields::SslProxy
            } else if s == FIELD_JSON_NAMES[6] {
                Fields::SocksProxy
            } else if s == FIELD_JSON_NAMES[7] {
                Fields::SocksVersion
            } else {
                Fields::Reserved(s.to_string())
            }
        }
        fn to_sstr(&self) -> &'static str {
            match self {
                Fields::ProxyType => FIELD_JSON_NAMES[0],
                Fields::ProxyAutoconfigUrl => FIELD_JSON_NAMES[1],
                Fields::FtpProxy => FIELD_JSON_NAMES[2],
                Fields::HttpProxy => FIELD_JSON_NAMES[3],
                Fields::NoProxy => FIELD_JSON_NAMES[4],
                Fields::SslProxy => FIELD_JSON_NAMES[5],
                Fields::SocksProxy => FIELD_JSON_NAMES[6],
                Fields::SocksVersion => FIELD_JSON_NAMES[7],
                _ => panic!("unsuppoted operation"),
            }
        }
    }

    impl<'de> Visitor<'de> for FieldVisitor {
        type Value = Fields;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "proxy fields")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Fields::from_str(v))
        }
    }

    impl<'de> Visitor<'de> for StructVisitor {
        type Value = Proxy<'de>;
        // type Value = (&'de str, &'de str);
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "expecting a complete struct `Proxy`")
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut ptype: Option<&'de str> = None;
            let mut pauto: Option<&'de str> = None;
            let mut ftpp: Option<&'de str> = None;
            let mut httpp: Option<&'de str> = None;
            let mut npxy: Option<Vec<&'de str>> = None;
            let mut sslp: Option<&'de str> = None;
            let mut socksp: Option<&'de str> = None;
            let mut socksv: Option<u8> = None;

            let mut is_dup: Option<A::Error> = None; // none or wrap a duplication error

            while let Some(key) = map.next_key()? {
                match key {
                    Fields::ProxyType => match ptype {
                        None => ptype = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::ProxyAutoconfigUrl => match pauto {
                        None => pauto = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::FtpProxy => match ftpp {
                        None => ftpp = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::HttpProxy => match npxy {
                        None => httpp = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::NoProxy => match npxy {
                        None => npxy = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::SslProxy => match npxy {
                        None => sslp = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::SocksProxy => match npxy {
                        None => socksp = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::SocksVersion => match npxy {
                        None => socksv = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    _ => {
                        map.next_value::<IgnoredAny>()?; // advance 1 token
                        dbgmsg!("SKIP...key {:?} and its value", key);
                    }
                }
            }

            if let Some(e) = is_dup {
                Err(e)
            } else {
                // missing is not an error
                let ptype = ptype.unwrap_or_default();
                let pauto = pauto.unwrap_or_default();
                let ftpp = ftpp.unwrap_or_default();
                let httpp = httpp.unwrap_or_default();
                let npxy = npxy.unwrap_or_default();
                let sslp = sslp.unwrap_or_default();
                let socksp = socksp.unwrap_or_default();
                let socksv = socksv.unwrap_or_default();

                // Ok((ptype, pauto))
                Ok(Proxy {
                    proxy_type: Cow::from(ptype),
                    proxy_autoconfig_url: Cow::from(pauto),
                    ftp_proxy: Cow::from(ftpp),
                    http_proxy: Cow::from(httpp),
                    no_proxy: npxy.iter().map(|x| Cow::from(*x)).collect(),
                    ssl_proxy: Cow::from(sslp),
                    socks_proxy: Cow::from(socksp),
                    socks_version: socksv,
                })
            }
        }
    }

    impl<'de> Deserialize<'de> for Fields {
        fn deserialize<D>(deser: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deser.deserialize_identifier(FieldVisitor)
        }
    }

    impl<'de> Deserialize<'de> for Proxy<'de> {
        fn deserialize<D>(deser: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deser.deserialize_struct("Proxy", FIELD_JSON_NAMES, StructVisitor)
        }
    }
} // deser_proxyref

mod deser_timeouts {
    use serde::de::{Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor};

    struct FieldVisitor;
    struct StructVisitor;

    const FIELD_JSON_NAMES: &[&str] = &["script", "pageLoad", "implicit"];

    #[derive(Debug)]
    enum Fields {
        Script,
        PageLoad,
        Implicit,
        Reserved(String),
    }

    impl Fields {
        fn from_str(s: &str) -> Self {
            if s == FIELD_JSON_NAMES[0] {
                Fields::Script
            } else if s == FIELD_JSON_NAMES[1] {
                Fields::PageLoad
            } else if s == FIELD_JSON_NAMES[2] {
                Fields::Implicit
            } else {
                Fields::Reserved(s.to_string())
            }
        }
        fn to_sstr(&self) -> &'static str {
            match self {
                Fields::Script => FIELD_JSON_NAMES[0],
                Fields::PageLoad => FIELD_JSON_NAMES[1],
                Fields::Implicit => FIELD_JSON_NAMES[2],
                _ => panic!("unsuppoted operation"),
            }
        }
    }

    impl<'de> Visitor<'de> for FieldVisitor {
        type Value = Fields;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "proxy fields")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Fields::from_str(v))
        }
    }

    impl<'de> Visitor<'de> for StructVisitor {
        type Value = super::Timeouts;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "expecting a complete `Timetouts`")
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut script: Option<u32> = None;
            let mut page_load: Option<u32> = None;
            let mut implicit: Option<u32> = None;

            let mut is_dup: Option<A::Error> = None; // none or wrap a duplication error

            while let Some(key) = map.next_key()? {
                match key {
                    Fields::Script => match script {
                        None => script = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::PageLoad => match page_load {
                        None => page_load = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    Fields::Implicit => match implicit {
                        None => implicit = Some(map.next_value()?),
                        _ => is_dup = Some(DeError::duplicate_field(key.to_sstr())),
                    },
                    _ => {
                        map.next_value::<IgnoredAny>()?; // advance 1 token
                        dbgmsg!("SKIP...key {:?} and its value", key);
                    }
                }
            }

            if let Some(e) = is_dup {
                // duplicate is an error
                Err(e)
            } else {
                // missing is not an error
                let script = script.unwrap_or_default();
                let page_load = page_load.unwrap_or_default();
                let implicit = implicit.unwrap_or_default();

                Ok(super::Timeouts {
                    script,
                    page_load,
                    implicit,
                })
            }
        }
    }

    // trivial
    impl<'de> Deserialize<'de> for Fields {
        fn deserialize<D>(deser: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deser.deserialize_identifier(FieldVisitor)
        }
    }

    // trivial
    impl<'de> Deserialize<'de> for super::Timeouts {
        fn deserialize<D>(deser: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deser.deserialize_struct("Proxy", FIELD_JSON_NAMES, StructVisitor)
        }
    }
} // deser_timeouts

mod deser_sessresval {
    use super::*;
    use serde::de::{Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor};

    #[derive(Debug)]
    enum Fields {
        MatchedCapa,
        SessionId,
        Reserved(String),
    }

    const FIELD_JSON_NAMES: &[&str] = &["capabilities", "sessionId"];

    impl Fields {
        fn from_str(s: &str) -> Self {
            if s == FIELD_JSON_NAMES[0] {
                Fields::MatchedCapa
            } else if s == FIELD_JSON_NAMES[1] {
                Fields::SessionId
            } else {
                Fields::Reserved(s.to_string())
            }
        }
        fn to_sstr(&self) -> &'static str {
            match self {
                Fields::MatchedCapa => FIELD_JSON_NAMES[0],
                Fields::SessionId => FIELD_JSON_NAMES[1],
                xxx => panic!("the string name of field `{:?}` is unknown", xxx),
            }
        }
    }

    struct FieldVisitor;

    struct StructVisitor<T> {
        phantom: std::marker::PhantomData<T>,
    }

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

    impl<'de, T: Deserialize<'de>> Visitor<'de> for StructVisitor<T>
    where
        T: w3c::W3cCapaGetter,
    {
        type Value = super::CommSessResultVal<T>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "unexpected field")
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

            // T must implement Deserialize
            decl!(f0, fid Fields::MatchedCapa, vtype T);
            decl!(f1, fid Fields::SessionId, vtype String);

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
                    Fields::MatchedCapa => give_value_to!(f0),
                    Fields::SessionId => give_value_to!(f1),

                    _ => {
                        map.next_value::<IgnoredAny>()?; // advance 1 token
                        dbgmsg!("SKIP...key {:?} and its value", key);
                    }
                }
            }

            if let Some(e) = is_dup {
                Err(e)
            } else {
                // missing field is an error
                let f0 = f0.0.ok_or_else(|| DeError::missing_field(f0.1))?;
                let f1 = f1.0.ok_or_else(|| DeError::missing_field(f1.1))?;

                Ok(super::CommSessResultVal {
                    capabilities: f0,
                    session_id: f1,
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

    // trivial
    impl<'de, T: Deserialize<'de>> Deserialize<'de> for CommSessResultVal<T>
    where
        T: w3c::W3cCapaGetter,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_struct(
                "A W3c response value",
                FIELD_JSON_NAMES,
                StructVisitor {
                    phantom: std::marker::PhantomData,
                },
            )
        }
    }
} // deser_sessresval
