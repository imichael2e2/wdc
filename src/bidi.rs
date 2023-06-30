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

//!
//! The module dedicated to WebDriver BiDi standard.
//!
//! WebDriver BiDi is a new standard for browser automation, it has several
//! advantages compared to it predecessor.
//!
//! Note that the standard is still
//! work-in-progress, all functionalities provided here are experimental,

use std::net::TcpStream;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;

// use crate::wdcmd::err::{InspectDrvCmdFail};
use crate::wdcmd::session::W3cCapRequSetter;
use crate::wdcmd::session::W3cCapaSetter;
use crate::wdcmd::session::W3cSessResultGetter;
use crate::wdcmd::status::{DrvStatResult, DrvStatResultGetter};

use crate::httpp::HttpRequestParts;
use crate::httpp::HttpResponseParts;
use crate::wsp::WebSocketHandshaker;
use crate::wsp::WebSocketMessage;
use crate::wsp::WspSett;

use crate::genericdrv::SessionMeta;

use crate::genericdrv::check_fail_drvcmd;
#[cfg(feature = "chromium")]
use crate::ChromeDriver;
use crate::CreateW3cSession;
#[cfg(feature = "firefox")]
use crate::GeckoDriver;
use crate::WdcError;

// pub trait BidiCommand {
//     fn gen_ctx(&mut self, ctx_type: u8) -> Result<(), WdcError>;
//     fn navi(&self, ctx_id: &str, url: &str) -> Result<&Self, WdcError>;
// }

pub trait CreateBidiClient
where
    Self: Sized,
{
    fn new_bdc(rhost: &str, rport: u16) -> BidiClient<Self>
    where
        for<'de, 'c1, 'c2> Self: CreateW3cSession<'de, 'c1, 'c2>;
}

#[cfg(feature = "firefox")]
impl CreateBidiClient for GeckoDriver {
    fn new_bdc(rhost: &str, rport: u16) -> BidiClient<Self> {
        BidiClient {
            kind: GeckoDriver,
            rhost: rhost.to_string(),
            rport,
            http_stream: None,
            ws_stream: None,
            ssmetas: vec![],
            ctxlist: vec![],
        }
    }
}

#[cfg(feature = "chromium")]
impl CreateBidiClient for ChromeDriver {
    fn new_bdc(rhost: &str, rport: u16) -> BidiClient<Self> {
        BidiClient {
            kind: ChromeDriver,
            rhost: rhost.to_string(),
            rport,
            http_stream: None,
            ws_stream: None,
            ssmetas: vec![],
            ctxlist: vec![],
        }
    }
}

// BidiClient //

#[derive(Debug, Default)]
pub struct BidiClient<D>
where
    D: CreateBidiClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    #[allow(dead_code)] // FIXME: use phantom?
    pub(crate) kind: D,
    pub(crate) rhost: String,
    pub(crate) rport: u16,
    pub(crate) http_stream: Option<Arc<Mutex<TcpStream>>>,
    pub(crate) ws_stream: Option<Arc<Mutex<TcpStream>>>,
    pub(crate) ssmetas: Vec<SessionMeta>,
    pub(crate) ctxlist: Vec<String>,
}

impl<D> Drop for BidiClient<D>
where
    D: CreateBidiClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    fn drop(&mut self) {
        for i in 0..self.ssmetas.len() {
            let ssmeta = &self.ssmetas[i];
            self.del_session(&ssmeta.ssid)
                .expect("delete driver session");
            self.ssmetas.remove(i);
        }
    }
}

pub fn init<D>(rhost: &str, rport: u16, ready_timeout: u32) -> Result<BidiClient<D>, WdcError>
where
    D: Sized + CreateBidiClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    let mut wdc = D::new_bdc(rhost, rport);

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
        match wdc.wstd_session_default() {
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

impl<D> BidiClient<D>
where
    D: CreateBidiClient + for<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2>,
{
    fn is_ready(&self) -> Result<(), WdcError> {
        let mut req = HttpRequestParts::from_scratch();

        match &self.http_stream {
            Some(rstream) => {
                let mut stream = rstream.lock().unwrap(); // LOCK
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

    pub fn gen_ctx(&mut self, ctx_type: u8) -> Result<(), WdcError> {
        if self.ws_stream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        #[allow(unused_mut)]
        let mut stream = self.ws_stream.as_ref().unwrap().lock().unwrap();

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small().allow_medium().allow_large();
        wsmsg
            .set_message_data(
                if ctx_type == 1 {
                    br#"{"id":123,"method":"browsingContext.create","params":{"type":"tab"}}"#
                } else {
                    br#"{"id":123,"method":"browsingContext.create","params":{"type":"window"}}"#
                },
                vec![WspSett::TextMsg, WspSett::Mask],
            )
            .unwrap();
        wsmsg.send_through(&mut stream).unwrap();

        let resp = WebSocketMessage::from_stream(&mut stream).unwrap();
        let respdata = resp.get_message_data().unwrap();

        dbgg!(String::from_utf8_lossy(&respdata));

        #[derive(serde::Deserialize)]
        #[allow(unused)]
        struct CommandResponse {
            id: u32,
            result: ResultData,
        }
        #[derive(serde::Deserialize)]
        struct ResultData {
            context: String,
        }

        let cmdresp = serde_json::from_slice::<CommandResponse>(&respdata).unwrap();

        self.ctxlist.push(cmdresp.result.context);
        // self.ctxlist
        // .push(String::from_utf8_lossy(&respdata[31..respdata.len() -
        // 3]).to_string());

        Ok(())
    }

    pub fn navi(&self, ctx_id: &str, url: &str) -> Result<&Self, WdcError> {
        if self.ws_stream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let mut stream = self.ws_stream.as_ref().unwrap().lock().unwrap();

        // {"id":123,"method":"browsingContext.navigate","params":{"url":"https://www.w3.org","context":"67BAB34FF3FD05FF8366DAD6A34E181D"}}
        let mb_string = format!(
            r#"{{"id":123,"method":"browsingContext.navigate","params":{{"url":"{}","context":"{}"}}}}"#,
            url, ctx_id
        );
        dbgg!(&mb_string);
        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small().allow_medium().allow_large();
        wsmsg
            .set_message_data(mb_string.as_bytes(), vec![WspSett::TextMsg, WspSett::Mask])
            .unwrap();
        wsmsg.send_through(&mut stream).unwrap();

        let resp = WebSocketMessage::from_stream(&mut stream).unwrap();
        let respdata = resp.get_message_data().unwrap();
        dbgg!(String::from_utf8_lossy(&respdata));

        // check_fail_drvcmd(&respdata)?;

        // CommandResponse = {
        //     id: js-uint,
        //     result: ResultData,
        //     Extensible
        // }

        // ErrorResponse = {
        //     id: js-uint / null,
        //     error: ErrorCode,
        //     message: text,
        //     ?stacktrace: text,
        //     Extensible
        // }
        let respdata_as_string = String::from_utf8_lossy(&respdata).to_string();
        if respdata_as_string.contains("result") {
            Ok(self)
        } else {
            Err(WdcError::Buggy)
        }
    }

    pub fn ctx_tree(&self) -> Result<&Self, WdcError> {
        if self.ws_stream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let mut stream = self.ws_stream.as_ref().unwrap().lock().unwrap();

        // {"id":123,"method":"browsingContext.navigate","params":{"url":"https://www.w3.org","context":"67BAB34FF3FD05FF8366DAD6A34E181D"}}
        let mb_string = format!(
            r#"{{"id":123,"method":"browsingContext.getTree","params":{{}}}}"#,
            /* ctx_id */
        );

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small().allow_medium().allow_large();
        wsmsg
            .set_message_data(mb_string.as_bytes(), vec![WspSett::TextMsg, WspSett::Mask])
            .unwrap();
        wsmsg.send_through(&mut stream).unwrap();

        let resp = WebSocketMessage::from_stream(&mut stream).unwrap();
        let respdata = resp.get_message_data().unwrap();
        dbgg!(String::from_utf8_lossy(&respdata));

        // check_fail_drvcmd(&respdata)?;

        // CommandResponse = {
        //     id: js-uint,
        //     result: ResultData,
        //     Extensible
        // }

        // ErrorResponse = {
        //     id: js-uint / null,
        //     error: ErrorCode,
        //     message: text,
        //     ?stacktrace: text,
        //     Extensible
        // }
        let respdata_as_string = String::from_utf8_lossy(&respdata).to_string();
        if respdata_as_string.contains("result") {
            // the ctxtree result from server currently is not much
            // self-described, maybe due to active development of new std,
            // thus leave this as trivial implementation
            Ok(self)
        } else {
            Err(WdcError::Buggy)
        }
    }

    pub fn ctxlist(&self) -> Vec<&str> {
        self.ctxlist.iter().map(|x| x.as_ref()).collect()
    }

    // private

    pub(crate) fn add_ssmeta(&mut self, ssid: String, profile: Option<String>) {
        self.ssmetas.push(SessionMeta { ssid, profile });
    }

    fn raddr(&self) -> String {
        format!("{}:{}", self.rhost, self.rport)
    }

    fn ensure_remote_connected(&mut self) -> Result<(), WdcError> {
        match TcpStream::connect(self.raddr()) {
            Ok(stream) => {
                stream.set_nodelay(true).unwrap();
                self.http_stream = Some(Arc::new(Mutex::new(stream)));
            }
            Err(_err) => {}
        }

        match self.http_stream {
            Some(_) => Ok(()),
            None => Err(WdcError::WebDriverRemoteConnectionFailed),
        }
    }

    fn del_session(&self, ssid: &str) -> Result<(), WdcError> {
        if self.http_stream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };
        let mut stream = self.http_stream.as_ref().unwrap().lock().unwrap();

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

    fn wstd_session_default(&mut self) -> Result<(), WdcError> {
        if self.http_stream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let rs = Arc::clone(self.http_stream.as_ref().unwrap());

        let mut stream = rs.lock().unwrap();

        let mut ws_capa = D::Capa::default();
        ws_capa.enable_bidi();
        let anycapa = D::Capa::default(); // must before requ!!!
        let mut requ = D::CapRequ::default();

        requ.allow_as_w3c(&anycapa); // tolerant match
        requ.mandate_as_w3c(&ws_capa);

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
                    self.add_ssmeta(sess.session_id().to_string(), None);

                    // FIXME: maybe RE is overkill?
                    let re = regex::Regex::new(r"ws://(.*)/session/(.*)").unwrap();
                    let cap = re.captures(sess.wsurl().unwrap()).unwrap();
                    let raddr = cap.get(1).unwrap().as_str();
                    let newssid = cap.get(2).unwrap().as_str();
                    let ws_uri = format!("/session/{}", newssid);
                    dbgg!(raddr, newssid);

                    if let Ok(mut wsstream) = TcpStream::connect(&raddr) {
                        WebSocketHandshaker::try_as_client(&mut wsstream, &ws_uri, &raddr)
                            .expect("handshake");
                        self.ws_stream = Some(Arc::new(Mutex::new(wsstream)));
                        dbgmsg!("yes!");
                    }

                    Ok(())
                }
                _ => Err(WdcError::Buggy),
            }
        } else {
            check_fail_drvcmd(resp.msgbody())?;
            Err(WdcError::Buggy) // unreachable
        }
    }
}
