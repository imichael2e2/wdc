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

use std::borrow::Cow;

///
/// WebDriver response for failed commands.
#[derive(PartialEq, Debug)]
pub struct BadCmdResp<'a> {
    value: BadCmdRespDetail<'a>,
}

///
/// The detail of command failure.
// "stack trace" occupy 400+ bytes, about 2~4 times more than
// "message" field; plus, most current webdriver server implementations use
// non-fully-machine-readable format for it, which makes it impossible to do
// zero-copy deserialization, and introduces extra heap allocation and allows
// non-trivial copy operation, all these are relatively expensive; most
// importantly, its detail helps little unless for webdriver server developer;
// so turn it off by default.
#[derive(PartialEq, Debug)]
pub struct BadCmdRespDetail<'a> {
    error: Cow<'a, str>,
    message: Cow<'a, str>,
    #[cfg(feature = "err_strace")]
    stacktrace: Cow<'a, str>,
}

impl BadCmdResp<'_> {
    pub fn err(&self) -> &str {
        &self.value.error
    }
    pub fn msg(&self) -> &str {
        &self.value.message
    }
    #[cfg(feature = "err_strace")]
    pub fn strace(&self) -> &str {
        &self.value.stacktrace
    }

    // no setters
}

mod deser {
    use super::*;
    use serde::de::{Deserialize, Deserializer, Error as DeError, MapAccess, Visitor};

    mod _1 {
        use super::*;

        #[derive(Debug)]
        enum Fields {
            Value,
            Reserved(String),
        }

        const FIELD_JSON_NAMES: &[&str] = &["value"];

        impl Fields {
            fn from_str(s: &str) -> Self {
                if s == FIELD_JSON_NAMES[0] {
                    Fields::Value
                } else {
                    Fields::Reserved(s.to_string())
                }
            }
            fn to_sstr(&self) -> &'static str {
                match self {
                    Fields::Value => FIELD_JSON_NAMES[0],
                    _ => panic!("unsuppoted operation"),
                }
            }
        }

        struct FieldVisitor;

        struct StructVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expecting complete struct")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Fields::from_str(v))
            }
        }

        impl<'de> Visitor<'de> for StructVisitor {
            type Value = super::BadCmdResp<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expecting complete struct")
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
                decl!(f0, fid Fields::Value, vtype BadCmdRespDetail<'de>);

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
                        Fields::Value => give_value_to!(f0),
                        _ => {
                            // skip unknown fields, rather than error
                            use serde::de::IgnoredAny;
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

                    unwrap_or_missing_error!(f0);

                    let ret = super::BadCmdResp { value: f0 };

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
        impl<'de> Deserialize<'de> for BadCmdResp<'de> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct("BadCmdResp", FIELD_JSON_NAMES, StructVisitor)
            }
        }
    }

    mod _2 {
        use super::*;

        #[derive(Debug)]
        enum Fields {
            Error,
            Message,
            StackTrace,
            Reserved(String),
        }

        const FIELD_JSON_NAMES: &[&str] = &["error", "message", "stacktrace"];

        impl Fields {
            fn from_str(s: &str) -> Self {
                if s == FIELD_JSON_NAMES[0] {
                    Fields::Error
                } else if s == FIELD_JSON_NAMES[1] {
                    Fields::Message
                } else if s == FIELD_JSON_NAMES[2] {
                    Fields::StackTrace
                } else {
                    Fields::Reserved(s.to_string())
                }
            }
            fn to_sstr(&self) -> &'static str {
                match self {
                    Fields::Error => FIELD_JSON_NAMES[0],
                    Fields::Message => FIELD_JSON_NAMES[1],
                    Fields::StackTrace => FIELD_JSON_NAMES[2],
                    _ => panic!("unsuppoted operation"),
                }
            }
        }

        struct FieldVisitor;

        struct StructVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expecting complete struct")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Fields::from_str(v))
            }
        }

        impl<'de> Visitor<'de> for StructVisitor {
            type Value = super::BadCmdRespDetail<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expecting complete struct `CmdRespChrom106`")
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

                decl!(f0, fid Fields::Error, vtype &'de str);
                decl!(f1, fid Fields::Message, vtype String);
                // decl!(f1, fid Fields::Message, vtype &'de str);
                #[cfg(feature = "err_strace")]
                decl!(f2, fid Fields::StackTrace, vtype String);

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
                        Fields::Error => give_value_to!(f0),
                        Fields::Message => give_value_to!(f1),
                        #[cfg(feature = "err_strace")]
                        Fields::StackTrace => give_value_to!(f2),
                        _ => {
                            // skip unknown fields, rather than error
                            use serde::de::IgnoredAny;
                            map.next_value::<IgnoredAny>()?; // advance 1 token
                            crate::dbgmsg!("SKIP...key {:?} and its value", key);
                        }
                    }
                }

                if let Some(e) = is_dup {
                    Err(e)
                } else {
                    let f0 = f0.0.ok_or_else(|| DeError::missing_field(f0.1))?;
                    let f1 = f1.0.ok_or_else(|| DeError::missing_field(f1.1))?;
                    #[cfg(feature = "err_strace")]
                    let mut f2 = f2.0.ok_or_else(|| DeError::missing_field(f2.1))?;

                    #[cfg(feature = "err_strace")]
                    let f2_deser_safe = f2.replace(r#"\n"#, "_NL_");

                    let ret = super::BadCmdRespDetail {
                        error: Cow::from(f0),
                        message: Cow::from(f1),
                        #[cfg(feature = "err_strace")]
                        stacktrace: Cow::from(f2_deser_safe),
                    };

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
        impl<'de> Deserialize<'de> for BadCmdRespDetail<'de> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct("BadCmdRespDetail", FIELD_JSON_NAMES, StructVisitor)
            }
        }
    }

    #[cfg(test)]
    mod tst {
        use super::*;

        use serde_test::{assert_de_tokens, Token};

        #[cfg(not(feature = "err_strace"))]
        #[test]
        fn _1() {
            let eobj = BadCmdResp {
                value: BadCmdRespDetail {
                    error: Cow::from("a"),
                    message: Cow::from("b"),
                },
            };

            assert_de_tokens(
                &eobj,
                &[
                    Token::Struct {
                        name: "BadCmdResp",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Struct {
                        name: "BadCmdRespDetail",
                        len: 3,
                    },
                    Token::Str("error"),
                    Token::BorrowedStr("a"),
                    Token::Str("message"),
                    Token::BorrowedStr("b"),
                    Token::StructEnd,
                    Token::StructEnd,
                ],
            );

            let estr = r#"{"value":{"error":"a","message":"b","stacktrace":"WebDriverError@chrome://remote/content/shared/webdriver/Errors.jsm:186:5\\nUnknownError@chrome://remote/content/shared/webdriver/Errors.jsm:513:5\\ncheckReadyState@chrome://remote/content/marionette/navigate.js:65:24\\nonNavigation@chrome://remote/content/marionette/navigate.js:333:39\\nemit@resource://gre/modules/EventEmitter.jsm:160:20\\nreceiveMessage@chrome://remote/content/marionette/actors/MarionetteEventsParent.jsm:44:25\\n"}}"#;

            let deobj = serde_json::from_slice::<BadCmdResp>(&estr.as_bytes()).unwrap();

            assert_eq!(deobj.value.error, "a");
            assert_eq!(deobj.value.message, "b");
        }

        #[cfg(feature = "err_strace")]
        #[test]
        fn _1() {
            let eobj = BadCmdResp {
                value: BadCmdRespDetail {
                    error: Cow::from("a"),
                    message: Cow::from("b"),
                    stacktrace: Cow::from("c"),
                },
            };

            assert_de_tokens(
                &eobj,
                &[
                    Token::Struct {
                        name: "BadCmdResp",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Struct {
                        name: "BadCmdRespDetail",
                        len: 3,
                    },
                    Token::Str("error"),
                    Token::BorrowedStr("a"),
                    Token::Str("message"),
                    Token::BorrowedStr("b"),
                    Token::Str("stacktrace"),
                    Token::Str("c"),
                    Token::StructEnd,
                    Token::StructEnd,
                ],
            );

            let estr = r#"{"value":{"error":"a","message":"b","stacktrace":"WebDriverError@chrome://remote/content/shared/webdriver/Errors.jsm:186:5\\nUnknownError@chrome://remote/content/shared/webdriver/Errors.jsm:513:5\\ncheckReadyState@chrome://remote/content/marionette/navigate.js:65:24\\nonNavigation@chrome://remote/content/marionette/navigate.js:333:39\\nemit@resource://gre/modules/EventEmitter.jsm:160:20\\nreceiveMessage@chrome://remote/content/marionette/actors/MarionetteEventsParent.jsm:44:25\\n"}}"#;

            let deobj = serde_json::from_slice::<BadCmdResp>(&estr.as_bytes()).unwrap();

            assert_eq!(deobj.value.error, "a");
            assert_eq!(deobj.value.message, "b");
            assert_eq!(
                deobj.value.stacktrace,
                r#"WebDriverError@chrome://remote/content/shared/webdriver/Errors.jsm:186:5_NL_UnknownError@chrome://remote/content/shared/webdriver/Errors.jsm:513:5_NL_checkReadyState@chrome://remote/content/marionette/navigate.js:65:24_NL_onNavigation@chrome://remote/content/marionette/navigate.js:333:39_NL_emit@resource://gre/modules/EventEmitter.jsm:160:20_NL_receiveMessage@chrome://remote/content/marionette/actors/MarionetteEventsParent.jsm:44:25_NL_"#
            );
        }
    }
}
