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

use std::sync::Arc;

use crate::wdcmd::session::{FirefoxCapa, GeckoCapRequ, GeckoSessResult};
use crate::wdcmd::session::{FirefoxCapaGetter, GeckoCapRequSetter};
use crate::wdcmd::session::{W3cCapaGetter, W3cSessResultGetter};

use crate::CreateW3cSession;
use crate::CreateWebDrvClient;
use crate::WdcError;

use crate::genericdrv::RendVendor;
use crate::genericdrv::WebDrvClient;

use crate::httpp::HttpRequestParts;
use crate::httpp::HttpResponseParts;

use crate::genericdrv::check_fail_drvcmd;

// GeckoDriver //

/// A placeholder for Mozilla GeckoDriver.
#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
#[derive(Debug, Default)]
pub struct GeckoDriver;

///
/// Create a GeckoDriver-specific WebDriver client.
impl CreateWebDrvClient for GeckoDriver {
    fn new(rhost: &str, rport: u16) -> WebDrvClient<Self> {
        WebDrvClient {
            kind: GeckoDriver,
            rhost: rhost.to_string(),
            rport,
            rstream: None,
            ssids: vec![],
        }
    }

    fn rend_vendor() -> RendVendor {
        RendVendor::Mozilla
    }
}

impl<'de, 'c1, 'c2> CreateW3cSession<'de, 'c1, 'c2> for GeckoDriver {
    type CapRequ<'r> = GeckoCapRequ<'r>where 'c1: 'r, 'c2: 'r;
    type Capa<'a> = FirefoxCapa<'a>;
    type SessResult = GeckoSessResult<'de>;
}

///
/// Initialize a WebDriver client instance.
///
/// It differs [`init_singl`] in the underlying assumption of session's backing
/// web browser, here assumes [Firefox](https://www.mozilla.org/en-US/firefox/).
#[cfg(feature = "firefox")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "firefox")))]
pub fn init_singl_ff(
    rhost: &str,
    rport: u16,
    capa: &(impl W3cCapaGetter + FirefoxCapaGetter),
    ready_timeout: u32,
) -> Result<WebDrvClient<GeckoDriver>, WdcError> {
    let mut wdc = GeckoDriver::new(rhost, rport);

    wdc.ensure_remote_connected()?;

    let ready_timeout_in_micros = (ready_timeout * 1000000) as u64;
    let mut already_wait = 0u64;
    let wait_each_round = 100u64;
    let mut ready_or_not = false;
    let mut sess_err: Option<WdcError> = None;
    let mut rdy_err: Option<WdcError> = None;

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
                rdy_err = Some(_e);
                break;
            }
        }
    }

    while already_wait < ready_timeout_in_micros {
        match wdc.ff_session_singl(capa) {
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
                sess_err = Some(_e);
                break;
            }
        }
    }

    dbgg!(already_wait);

    if ready_or_not {
        Ok(wdc)
    } else {
        if rdy_err.is_some() {
            let rdy_err = rdy_err.unwrap();
            Err(rdy_err)
        } else if sess_err.is_some() {
            let sess_err = sess_err.unwrap();
            Err(sess_err)
        } else {
            Err(WdcError::WebDriverNotReady)
        }
    }
}

impl WebDrvClient<GeckoDriver> {
    fn ff_session_singl(
        &mut self,
        capa: &(impl W3cCapaGetter + FirefoxCapaGetter),
    ) -> Result<(), WdcError> {
        if self.rstream.is_none() {
            return Err(WdcError::WebDriverRemoteConnectionFailed);
        };

        let rs = Arc::clone(self.rstream.as_ref().unwrap());
        let mut stream = rs.lock().unwrap();

        let anycapa = FirefoxCapa::default(); // before requ
        let mut requ = GeckoCapRequ::default();

        requ.allow(&anycapa); // tolerant match

        requ.mandate(capa);

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
                deser_result = serde_json::from_slice::<GeckoSessResult>(resp.msgbody())
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
}
