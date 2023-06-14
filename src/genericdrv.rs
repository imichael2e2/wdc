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

use std::net::TcpStream;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;

use serde::Deserialize;
use serde::Serialize;

use crate::wdcmd::err::BadCmdResp;
use crate::wdcmd::find_elem::{
    FindElemFilter, FindElemFilterSetter, FindElemsResult, FindElemsResultGetter,
};
use crate::wdcmd::status::{DrvStatResult, DrvStatResultGetter};

use crate::wdcmd::session::{W3cCapRequSetter, W3cCapaGetter, W3cCapaSetter, W3cSessResultGetter};

use crate::wdcmd::actions::ActionGroup;

use crate::httpp;

use httpp::HttpRequestParts;
use httpp::HttpResponseParts;

use crate::WdcError;

pub(crate) fn check_fail_drvcmd(s: &[u8]) -> Result<(), WdcError> {
    match serde_json::from_slice::<BadCmdResp>(s) {
        Ok(eobj) => {
            if eobj.err() == "session not created" && eobj.msg() == "Session is already started" {
                Err(WdcError::BusyCreateSession)
            } else {
                Err(WdcError::BadDrvCmd(
                    eobj.err().to_string(),
                    eobj.msg().to_string(),
                ))
            }
        }
        Err(_e) => {
            dbgg!(_e);
            Err(WdcError::Buggy)
        }
    }
}

pub enum RendVendor {
    Mozilla,
    Google,
}

///
/// A trait for creating new WebDriver client instances.
pub trait CreateWebDrvClient
where
    Self: Sized,
{
    ///
    /// Create a new WebDriver client instance.
    ///
    /// Note that usually this method create a bare instance. While it provides
    /// fine-grained control over WebDriver, it is **not** ready for browser
    /// automation. Should [`init`] be used if one wants a
    /// ready-to-automation client instance.
    fn new(rhost: &str, rport: u16) -> WebDrvClient<Self>
    where
        for<'de, 'c1, 'c2> Self: CreateW3cSession<'de, 'c1, 'c2>;

    //

    fn rend_vendor() -> RendVendor;
}

///
/// Create standard-compliance WebDriver sessions.
pub trait CreateW3cSession<'de, 'c1, 'c2>
where
    Self: Sized,
{
    type CapRequ<'r>: Default + Serialize + W3cCapRequSetter<'c1, 'c2>
    where
        'c1: 'r,
        'c2: 'r;
    type Capa<'a>: Default + W3cCapaGetter + W3cCapaSetter<'a>;
    type SessResult: Deserialize<'de> + W3cSessResultGetter;
}

// WebDrvClient //

///
/// The client-side implementation of WebDriver.
///
/// It serves as the basic client implementation, with several assumptions:
///
/// 1. There is **one** active session at most.
/// 2. The session is standard-compliant.
///
/// See module-level docs [`crate`] for the usage.
#[derive(Debug, Default)]
pub struct WebDrvClient<D>
where
    D: CreateWebDrvClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    #[allow(dead_code)] // FIXME: use phantom?
    pub(crate) kind: D,
    pub(crate) rhost: String,
    pub(crate) rport: u16,
    pub(crate) rstream: Option<Arc<Mutex<TcpStream>>>,
    pub(crate) ssids: Vec<String>,
}

impl<D> Drop for WebDrvClient<D>
where
    D: CreateWebDrvClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    fn drop(&mut self) {
        for i in 0..self.ssids.len() {
            let ssid = &self.ssids[i];
            self.del_session(ssid).expect("delete driver session");
            self.ssids.remove(i);
        }
    }
}

///
/// The identical implementations shared by any standard-compliance WebDriver
/// client, regardless of driver type `D`.
///
/// WebDriver clients speak "commands", which form a large part of the client
/// implementation. W3C standard
/// [defines](https://w3c.github.io/webdriver/#dfn-endpoints) a
/// minimal set of commands that every standard-compliance server and client
/// should support. Here is a one-to-one(roughly) map for that set.
impl<D> WebDrvClient<D>
where
    D: CreateWebDrvClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    // commands

    pub fn is_ready(&self) -> Result<(), WdcError> {
        let mut req = HttpRequestParts::from_scratch();

        match &self.rstream {
            Some(rstream) => {
                let mut stream = rstream.lock().unwrap(); // -
                req.http1p1()
                    .get("/status")
                    .host(&self.raddr())
                    .send_through(&mut stream)
                    .unwrap();
                let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

                dbgg!(String::from_utf8_lossy(resp.msgbody()));

                if resp.is_ok() {
                    match serde_json::from_slice::<DrvStatResult>(resp.msgbody()) {
                        Ok(res) => {
                            if res.ready() {
                                Ok(())
                            } else if res.msg() == "Session already started" {
                                Err(WdcError::DriverNotReadyBusySession)
                            } else {
                                Err(WdcError::WebDriverNotReady)
                            }
                        }
                        _ => Err(WdcError::Buggy),
                    }
                } else {
                    check_fail_drvcmd(resp.msgbody())?;
                    Err(WdcError::Buggy) // unreachable
                }
            }
            None => Err(WdcError::WebDriverRemoteConnectionFailed),
        }
    }

    pub fn navi_to(&self, url: &str) -> Result<&Self, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();
        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"url":""#.as_bytes());
        mb.extend(url.bytes());
        mb.extend(r#""}"#.as_bytes());

        run_diag!("navi_to-send_req", {
            req.http1p1()
                .post(&format!("/session/{}/url", ssid))
                .host(&self.raddr())
                .msgbody_from_slice(&mb)
                .send_through(&mut stream)
                .unwrap();
        });

        let resp;

        run_diag!("navi_to-got_resp", {
            resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();
        });

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(self)
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn get_url(&self) -> Result<Vec<u8>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .get(&format!("/session/{}/url", ssid))
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 10, 2).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(resp.msgbody)
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn find_elem_css(&self, v: &str) -> Result<Vec<u8>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut conf = FindElemFilter::default();

        conf.set_using("css selector");
        conf.set_value(v);

        let mut mb = Vec::<u8>::new();
        mb.extend(serde_json::to_vec(&conf).unwrap());

        req.http1p1()
            .post(&format!("/session/{}/element", ssid))
            .host(&self.raddr())
            .msgbody_from_slice(&mb)
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 49, 3).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(resp.msgbody)
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn find_elems_css(&self, v: &str) -> Result<Vec<String>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut conf = FindElemFilter::default();

        conf.set_using("css selector");
        conf.set_value(v);

        let mut mb = Vec::<u8>::new();
        mb.extend(serde_json::to_vec(&conf).unwrap());

        req.http1p1()
            .post(&format!("/session/{}/elements", ssid))
            .host(&self.raddr())
            .msgbody_from_slice(&mb)
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            match serde_json::from_slice::<FindElemsResult>(resp.msgbody()) {
                Ok(resp) => Ok(resp.eleids().iter().map(|x| x.to_string()).collect()),
                _ => Err(WdcError::Buggy),
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn elem_send_keys(&self, eleid: &str, keys: &str) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"text":""#.as_bytes());
        mb.extend(keys.as_bytes());
        mb.extend(r#""}"#.as_bytes());

        req.http1p1()
            .post(&format!("/session/{}/element/{}/value", ssid, eleid))
            .host(&self.raddr())
            .content_type("application/json")
            .msgbody_from_slice(&mb)
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            // actually value:null needs to be checked, but not necessary
            Ok(())
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn screenshot(&self, save_path: &str) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .get(&format!("/session/{}/screenshot", ssid))
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, Some(save_path), 10, 2).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            if resp.msgbody_persist().is_some() {
                Ok(())
            } else {
                Err(WdcError::Buggy)
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    // FIXME: should not be &str
    pub fn screenshot_elem(&self, eleid: &str, save_path: &str) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .get(&format!("/session/{}/element/{}/screenshot", ssid, eleid))
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        dbgg!(&req);

        let resp = HttpResponseParts::from_stream(&mut stream, Some(save_path), 10, 2).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            if resp.msgbody_persist().is_some() {
                Ok(())
            } else {
                Err(WdcError::Buggy)
            }
        } else {
            dbgg!(&resp);
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn print_page(&self, save_path: &str) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"#.as_bytes());
        mb.extend(r#""background":true"#.as_bytes());
        mb.extend(r#","orientation":"portrait""#.as_bytes());
        mb.extend(r#"}"#.as_bytes());

        dbgg!(std::str::from_utf8(&mb).unwrap());

        req.http1p1()
            .post(&format!("/session/{}/print", ssid))
            .msgbody_from_slice(&mb)
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, Some(save_path), 10, 2).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            if resp.msgbody_persist().is_some() {
                Ok(())
            } else {
                Err(WdcError::Buggy)
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn page_src(&self, save_path: Option<&str>) -> Result<Option<Vec<u8>>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .get(&format!("/session/{}/source", ssid))
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, save_path, 10, 2).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            if resp.msgbody_persist().is_some() {
                Ok(None)
            } else if resp.msgbody().len() > 0 {
                Ok(Some(resp.msgbody))
            } else {
                Err(WdcError::Buggy)
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn exec_sync(&self, script: &str, args: Vec<&str>) -> Result<Vec<u8>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"script":""#.as_bytes());
        mb.extend(script.as_bytes());
        mb.extend(r#"","args":["#.as_bytes());
        for a in &args {
            mb.extend(a.as_bytes());
            mb.extend(",".as_bytes());
        }
        if args.len() > 0 {
            mb.truncate(mb.len() - 1);
        }
        mb.extend(r#"]}"#.as_bytes());

        run_diag!("exec-send_req", {
            req.http1p1()
                .post(&format!("/session/{}/execute/sync", ssid))
                .msgbody_from_slice(&mb)
                .host(&self.raddr())
                .send_through(&mut stream)
                .unwrap();
        });

        let resp;

        run_diag!("exec-got_resp", {
            resp = HttpResponseParts::from_stream(&mut stream, None, 9, 1).unwrap();
        });

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(resp.msgbody)
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn exec_async(&self, script: &str, args: Vec<&str>) -> Result<Vec<u8>, WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"script":""#.as_bytes());
        mb.extend(script.as_bytes());
        mb.extend(r#"","args":["#.as_bytes());
        for a in &args {
            mb.extend(a.as_bytes());
            mb.extend(",".as_bytes());
        }
        if args.len() > 0 {
            mb.truncate(mb.len() - 1);
        }
        mb.extend(r#"]}"#.as_bytes());

        req.http1p1()
            .post(&format!("/session/{}/execute/async", ssid))
            .msgbody_from_slice(&mb)
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 9, 1).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(resp.msgbody)
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    pub fn perform_actions(&self, actg: ActionGroup) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        if self.ssids.len() == 0 {
            return Err(WdcError::Buggy);
        }
        let ssid = &self.ssids[0];
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        let mut mb_s = serde_json::to_string(&actg).unwrap();
        mb_s = mb_s.replace("__U__", r"\u");
        let mb = mb_s.as_bytes();

        req.http1p1()
            .post(&format!("/session/{}/actions", ssid))
            .msgbody_from_slice(mb)
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 9, 1).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(())
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    // private

    pub(crate) fn add_ssid(&mut self, ssid: String) {
        self.ssids.push(ssid);
    }

    pub(crate) fn raddr(&self) -> String {
        format!("{}:{}", self.rhost, self.rport)
    }

    pub(crate) fn ensure_remote_connected(&mut self) -> Result<(), WdcError> {
        match TcpStream::connect(self.raddr()) {
            Ok(stream) => {
                // stream.set_nodelay(true).unwrap();
                self.rstream = Some(Arc::new(Mutex::new(stream)));
            }
            Err(_err) => {}
        }

        match self.rstream {
            Some(_) => Ok(()),
            None => Err(WdcError::WebDriverRemoteConnectionFailed),
        }
    }

    fn w3c_session_default(&mut self) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let rs = Arc::clone(self.rstream.as_ref().unwrap());

        let mut stream = rs.lock().unwrap();

        let anycapa = D::Capa::default();
        let mut requ = D::CapRequ::default();

        requ.allow_as_w3c(&anycapa); // tolerant match

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"capabilities":"#.as_bytes());
        mb.extend(serde_json::to_vec(&requ).expect("ser"));
        mb.extend(r#"}"#.as_bytes());

        dbgg!(String::from_utf8_lossy(&mb));

        req.http1p1()
            .post("/session")
            .host(&self.raddr())
            .msgbody_from_slice(&mb)
            .content_type("application/json")
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            let deser_result;

            run_diag!("deser_resp", {
                deser_result = serde_json::from_slice::<D::SessResult>(resp.msgbody())
            });

            match deser_result {
                Ok(sess) => {
                    self.add_ssid(sess.session_id().to_string());
                    Ok(())
                }
                Err(_e) => {
                    dbgg!(_e);
                    Err(WdcError::Buggy)
                }
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    fn w3c_session_singl(&mut self, capa: &impl W3cCapaGetter) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let rs = Arc::clone(self.rstream.as_ref().unwrap());

        let mut stream = rs.lock().unwrap();

        let anycapa = D::Capa::default();
        let mut requ = D::CapRequ::default();

        requ.allow_as_w3c(&anycapa); // tolerant match

        requ.mandate_as_w3c(capa);

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"capabilities":"#.as_bytes());
        mb.extend(serde_json::to_vec(&requ).expect("ser"));
        mb.extend(r#"}"#.as_bytes());

        dbgg!(String::from_utf8_lossy(&mb));

        req.http1p1()
            .post("/session")
            .host(&self.raddr())
            .msgbody_from_slice(&mb)
            .content_type("application/json")
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            let deser_result;

            run_diag!("deser_resp", {
                deser_result = serde_json::from_slice::<D::SessResult>(resp.msgbody())
            });

            match deser_result {
                Ok(sess) => {
                    self.add_ssid(sess.session_id().to_string());
                    Ok(())
                }
                _ => Err(WdcError::Buggy),
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    #[allow(dead_code)]
    fn w3c_session_multi(&mut self, capas: Vec<&impl W3cCapaGetter>) -> Result<(), WdcError> {
        if capas.len() == 0 {
            return self.w3c_session_default();
        }
        if capas.len() == 1 {
            return self.w3c_session_singl(capas[0]);
        }

        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let rs = Arc::clone(self.rstream.as_ref().unwrap());

        let mut stream = rs.lock().unwrap();

        let mut requ = D::CapRequ::default();

        requ.mandate_as_w3c(capas[0]);
        for c in &capas[1..] {
            requ.allow_as_w3c(*c);
        }

        let mut req = HttpRequestParts::from_scratch();

        let mut mb = Vec::<u8>::new();
        mb.extend(r#"{"capabilities":"#.as_bytes());
        mb.extend(serde_json::to_vec(&requ).expect("ser"));
        mb.extend(r#"}"#.as_bytes());

        dbgg!(String::from_utf8_lossy(&mb));

        req.http1p1()
            .post("/session")
            .host(&self.raddr())
            .msgbody_from_slice(&mb)
            .content_type("application/json")
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            let deser_result;

            run_diag!("deser_resp", {
                deser_result = serde_json::from_slice::<D::SessResult>(resp.msgbody())
            });

            match deser_result {
                Ok(sess) => {
                    self.add_ssid(sess.session_id().to_string());
                    Ok(())
                }
                _ => Err(WdcError::Buggy),
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }

    fn del_session(&self, ssid: &str) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        let mut stream = self.rstream.as_ref().unwrap().lock().unwrap();

        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .delete(&format!("/session/{}", ssid))
            .host(&self.raddr())
            .send_through(&mut stream)
            .unwrap();

        let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();

        dbgg!(String::from_utf8_lossy(resp.headers()));
        dbgg!(String::from_utf8_lossy(resp.msgbody()));

        if resp.is_ok() {
            Ok(())
        } else {
            Err(WdcError::Buggy)
        }
    }
}

///
/// Initialize a WebDriver client instance.
///
/// It initializes the WebDriver client with default settings. By this, the
/// client shall be **completely** ready for browser automation. This process
/// consists of:
///
/// 1. Ensure the connection to WebDriver server.
/// 2. Ensure WebDriver server is ready for command processing.
///
/// See [`crate`] for more examples.
pub fn init<D>(rhost: &str, rport: u16, ready_timeout: u32) -> Result<WebDrvClient<D>, WdcError>
where
    D: Sized + CreateWebDrvClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    let mut wdc = D::new(rhost, rport);

    wdc.ensure_remote_connected()?;

    let ready_timeout_in_micros = (ready_timeout * 1000000) as u64;
    let mut already_wait = 0u64;
    let wait_each_round = 100u64;
    let mut ready_or_not = false;

    while already_wait < ready_timeout_in_micros {
        match wdc.is_ready() {
            Ok(_) => {
                break;
            }
            Err(WdcError::DriverNotReadyBusySession) => {
                std::thread::sleep(std::time::Duration::from_micros(wait_each_round));
                already_wait += wait_each_round;
                continue;
            }
            Err(_e) => {
                dbgg!(_e);
                break;
            }
        }
    }

    while already_wait < ready_timeout_in_micros {
        match wdc.w3c_session_default() {
            Ok(_) => {
                ready_or_not = true;
                break;
            }
            Err(WdcError::BusyCreateSession) => {
                std::thread::sleep(std::time::Duration::from_micros(wait_each_round));
                already_wait += wait_each_round;
                continue;
            }
            Err(_e) => {
                dbgg!(_e);
                break;
            }
        }
    }

    dbgg!(already_wait);

    if ready_or_not {
        Ok(wdc)
    } else {
        Err(WdcError::WebDriverNotReady)
    }
}

///
/// Initialize a WebDriver client instance.
///
/// It differs [`init`] in the underlying assumption of the number of driver
/// servers, here assumes only **one** server. Hence the
/// capabilities configurations can accurately apply to the server.
pub fn init_singl<D>(
    rhost: &str,
    rport: u16,
    capa: &impl W3cCapaGetter,
    ready_timeout: u32,
) -> Result<WebDrvClient<D>, WdcError>
where
    D: Sized + CreateWebDrvClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    let mut wdc = D::new(rhost, rport);

    wdc.ensure_remote_connected()?;

    let ready_timeout_in_micros = (ready_timeout * 1000000) as u64;
    let mut already_wait = 0u64;
    let wait_each_round = 100u64;
    let mut ready_or_not = false;

    while already_wait < ready_timeout_in_micros {
        match wdc.is_ready() {
            Ok(_) => {
                break;
            }
            Err(WdcError::DriverNotReadyBusySession) => {
                std::thread::sleep(std::time::Duration::from_micros(wait_each_round));
                already_wait += wait_each_round;
                continue;
            }
            Err(_e) => {
                dbgg!(_e);
                break;
            }
        }
    }

    while already_wait < ready_timeout_in_micros {
        match wdc.w3c_session_singl(capa) {
            Ok(_) => {
                ready_or_not = true;
                break;
            }
            Err(WdcError::BusyCreateSession) => {
                std::thread::sleep(std::time::Duration::from_micros(wait_each_round));
                already_wait += wait_each_round;
                continue;
            }
            Err(_e) => {
                dbgg!(_e);
                break;
            }
        }
    }

    dbgg!(already_wait);

    if ready_or_not {
        Ok(wdc)
    } else {
        Err(WdcError::WebDriverNotReady)
    }
}

// UNIT TEST //

#[cfg(test)]
mod utst {
    use super::*;
    use std::net::TcpStream; // port probe.

    #[cfg(feature = "firefox")]
    mod gecko {
        use super::*;
        use crate::GeckoDriver;
        type RendKind = GeckoDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 4444;

        #[test]
        fn init_and_deinit1() {
            // correctly init and deinit, concurrently(1~4 are same-task clients)

            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_err(), true);
            if let Err(e) = wdc.is_ready() {
                assert_eq!(e, WdcError::DriverNotReadyBusySession);
            }
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit2() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_err(), true);
            if let Err(e) = wdc.is_ready() {
                assert_eq!(e, WdcError::DriverNotReadyBusySession);
            }
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit3() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_err(), true);
            if let Err(e) = wdc.is_ready() {
                assert_eq!(e, WdcError::DriverNotReadyBusySession);
            }
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit4() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_err(), true);
            if let Err(e) = wdc.is_ready() {
                assert_eq!(e, WdcError::DriverNotReadyBusySession);
            }
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn session_id1() {
            let wdc = init::<GeckoDriver>("127.0.0.1", 4444, 10).expect("init wdc");
            assert!(wdc.ssids.len() == 1);
            assert!(is_uuid(&wdc.ssids[0]));
        }
    }

    #[cfg(feature = "chromium")]
    mod chrom {
        use super::*;
        use crate::ChromeDriver;

        type RendKind = ChromeDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 9515;

        #[test]
        fn init_and_deinit1() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_ok(), true);
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit2() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_ok(), true);
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit3() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_ok(), true);
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn init_and_deinit4() {
            let raddr = format!("{}:{}", REND_HOST, REND_PORT);

            let wdc = init::<RendKind>(REND_HOST, REND_PORT, 10).expect("failed to init wdc");
            if let Err(_err) = TcpStream::connect(&raddr) {
                assert!(false, "rend is down");
            }
            assert_eq!(wdc.rstream.is_some(), true);
            assert_eq!(wdc.ssids.len() > 0, true);
            assert_eq!(wdc.is_ready().is_ok(), true);
            let ssid = wdc.ssids[0].clone();

            drop(wdc);

            match TcpStream::connect(&raddr) {
                Ok(mut s) => {
                    let mut req = HttpRequestParts::from_scratch();
                    req.http1p1()
                        .get(&format!("/session/{}/url", ssid))
                        .host(&raddr)
                        .send_through(&mut s)
                        .unwrap();
                    let resp = HttpResponseParts::from_stream(&mut s, None, 0, 0).unwrap();
                    assert_eq!(
                        String::from_utf8_lossy(resp.msgbody()).contains("invalid session id"),
                        true
                    );
                }
                Err(_e) => {
                    assert!(false, "server is down");
                }
            }
        }

        #[test]
        fn session_id1() {
            let wdc = init::<ChromeDriver>("127.0.0.1", 9515, 10).expect("init wdc");
            assert!(wdc.ssids.len() == 1);
            assert!(is_uuid_nodash(&wdc.ssids[0]));
        }
    }

    // Auxiliary Functions //

    #[allow(unused)]
    fn is_uuid(s: &str) -> bool {
        let re = regex::Regex::new(
            r"^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}$",
        )
        .unwrap();

        re.is_match(s)
    }

    #[allow(unused)]
    fn is_uuid_nodash(s: &str) -> bool {
        let re = regex::Regex::new(
            r"^[0-9A-Fa-f]{8}[0-9A-Fa-f]{4}[0-9A-Fa-f]{4}[0-9A-Fa-f]{4}[0-9A-Fa-f]{12}$",
        )
        .unwrap();

        re.is_match(s)
    }
}
