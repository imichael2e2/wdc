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

use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum HttpError {
    Buggy,
    InvalidHttpData,
    InvalidHttpVersion,
    InvalidContentLength,
    HeaderNotExistContentLength,
    HeaderNotExistContentType,
    HeaderNotExistReqUri,
    HeaderNotExistConnection,
    HeaderNotExistUpgrade,
    HeaderNotExistHost,
    HeaderNotExistWsKey,
    HeaderNotExistWsVer,
    HeaderNotExistWsAccept,
    PersistBodyPathAbsent,
    PersistBodyPathNotFound,
    PersistBodyWrite,
    IncompleteFinish,
}

// HttpRequestParts //

pub(crate) struct HttpRequestParts {
    method: Vec<u8>,
    requri: Vec<u8>,
    httpver: Vec<u8>,
    headers: Vec<u8>, // no order
    msgbody: Vec<u8>,
}

#[allow(unused)]
impl HttpRequestParts {
    pub fn from_scratch() -> Self {
        HttpRequestParts {
            method: vec![],
            requri: vec![],
            httpver: vec![],
            headers: vec![],
            msgbody: vec![],
        }
    }

    pub fn http1p1(&mut self) -> &mut Self {
        self.set_http_version("HTTP/1.1");
        self
    }

    pub fn get(&mut self, uri: &str) -> &mut Self {
        self.set_method("GET").set_request_uri(uri);
        self
    }

    pub fn post(&mut self, uri: &str) -> &mut Self {
        self.set_method("POST").set_request_uri(uri);
        self
    }

    pub fn delete(&mut self, uri: &str) -> &mut Self {
        self.set_method("DELETE").set_request_uri(uri);
        self
    }

    pub fn host(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Host: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn content_type(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Content-Type: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn connection(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Connection: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn upgrade(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Upgrade: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn ws_key(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Sec-WebSocket-Key: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn ws_ver(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Sec-WebSocket-Version: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    // getter

    pub fn get_req_uri(&self) -> Result<&[u8], HttpError> {
        if self.requri.len() > 0 {
            Ok(&self.requri[..])
        } else {
            Err(HttpError::HeaderNotExistContentType)
        }
    }

    pub fn get_content_length(&self) -> Result<usize, HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"C" || &buf[curi..curi + 1] == b"c" {
                let h_name = b"Content-Length:";
                let h_name_alt = b"content-length:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok((String::from_utf8_lossy(&buf[hdl_val_begi..hdl_val_endi]))
                .trim()
                .parse::<usize>()
                .unwrap())
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_connection(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"C" || &buf[curi..curi + 1] == b"c" {
                let h_name = b"Connection:";
                let h_name_alt = b"connection:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_upgrade(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"U" || &buf[curi..curi + 1] == b"u" {
                let h_name = b"Upgrade:";
                let h_name_alt = b"upgrade:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_host(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"H" || &buf[curi..curi + 1] == b"h" {
                let h_name = b"Host:";
                let h_name_alt = b"host:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_ws_key(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"S" || &buf[curi..curi + 1] == b"s" {
                let h_name = b"Sec-WebSocket-Key:";
                let h_name_alt = b"sec-webSocket-key:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_ws_ver(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"S" || &buf[curi..curi + 1] == b"s" {
                let h_name = b"Sec-WebSocket-Version:";
                let h_name_alt = b"sec-websocket-version:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut req: Vec<u8> = vec![];
        req.extend(&self.method);
        req.extend(b" ");
        req.extend(&self.requri);
        req.extend(b" ");
        req.extend(&self.httpver);
        req.extend(b"\r\n");
        req.extend(&self.headers);
        req.extend(b"\r\n");
        req.extend(&self.msgbody);

        req
    }

    #[allow(dead_code)]
    pub fn get_msgbody(&self) -> Vec<u8> {
        self.msgbody.clone()
    }

    // non-public //

    fn set_method(&mut self, m: &str) -> &mut Self {
        self.method.extend(m.as_bytes());
        self
    }

    // RFC2616-5.1.2
    fn set_request_uri(&mut self, uri: &str) -> &mut Self {
        self.requri.extend(uri.as_bytes());
        self
    }

    fn set_http_version(&mut self, v: &str) -> &mut Self {
        self.httpver.extend(v.as_bytes());
        self
    }

    #[allow(dead_code)]
    fn ua(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("User-Agent: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn msgbody_from_slice(&mut self, b: &[u8]) -> &mut Self {
        self.msgbody.extend(b);
        let hdrline = format!("Content-Length: {}\r\n", b.len()).into_bytes();
        dbgg!(std::str::from_utf8(b).unwrap());
        self.headers.extend(hdrline);
        self
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut req = vec![];

        req.extend(&self.method);
        req.extend(b" ");
        req.extend(&self.requri);
        req.extend(b" ");
        req.extend(&self.httpver);
        req.extend(b"\r\n");
        req.extend(&self.headers);
        req.extend(b"\r\n");
        req.extend(&self.msgbody);

        req
    }

    pub fn send_through(&self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let wbuf = self.to_vec();
        dbgg!(wbuf.len());
        stream.write_all(&wbuf).unwrap();

        Ok(())
    }

    pub fn from_stream(mut stream: &TcpStream) -> Result<Self, HttpError> {
        // pub(crate) struct HttpRequestParts {
        //     method: Vec<u8>,
        //     requri: Vec<u8>,
        //     httpver: Vec<u8>,
        //     headers: Vec<u8>, // unorder
        //     msgbody: Vec<u8>,
        // }

        let mut newone = HttpRequestParts::from_scratch();
        const SZ_MAX_RBUF: usize = 8192;
        let mut rbuf = [0u8; SZ_MAX_RBUF]; // Read Buffer
        let mut curi = 0usize;

        let mut req_line_starti = 0usize;
        let mut req_uri_starti = 0usize;
        let mut http_ver_starti = 0usize;
        let mut headers_starti = 0usize;

        // Request-Line   = Method SP Request-URI SP HTTP-Version CRLF
        loop {
            stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap(); // !
            if rbuf[curi] == b' ' {
                if req_uri_starti == 0 {
                    req_uri_starti = curi + 1;
                } else if http_ver_starti == 0 {
                    http_ver_starti = curi + 1;
                }
            }
            if rbuf[curi] == b'\r' {
                curi += 1;
                stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap(); // !
                if rbuf[curi] == b'\n' {
                    headers_starti = curi + 1;
                    curi += 1;
                    break;
                } else {
                    dbgg!(1233);
                    return Err(HttpError::InvalidHttpData);
                }
            } else {
                curi += 1;
            }
        }

        newone
            .method
            .extend(&rbuf[req_line_starti..req_uri_starti - 1]);
        newone
            .requri
            .extend(&rbuf[req_uri_starti..http_ver_starti - 1]);
        newone
            .httpver
            .extend(&rbuf[http_ver_starti..headers_starti - 2]);

        // Headers  =  ...CRLFCRLF
        let mut mb_starti = 0usize;
        loop {
            if curi >= rbuf.len() {
                dbgg!(rbuf.len());
                return Err(HttpError::InvalidHttpData);
            }
            stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap(); // !
            if rbuf[curi] == b'\r' {
                curi += 1;
                stream.read_exact(&mut rbuf[curi..curi + 3]).unwrap(); // !
                if &rbuf[curi..curi + 3] == b"\n\r\n" {
                    mb_starti = curi + 3;
                    curi += 3;
                    break;
                } else {
                    curi += 3;
                }
            } else {
                curi += 1;
            }
        }

        newone.headers.extend(&rbuf[headers_starti..mb_starti]);

        let mut content_len = 0usize;
        match newone.get_content_length() {
            Ok(l) => content_len = l,
            Err(e) => match e {
                HttpError::HeaderNotExistContentLength => return Ok(newone),
                _ => return Err(e),
            },
        }

        if content_len > 0 && content_len < SZ_MAX_RBUF {
            // FIXME: SZ_MAX_RBUF is inaccurate
            stream.read_exact(&mut rbuf[0..content_len]).unwrap();
            newone.headers.extend(&rbuf[0..content_len]);

            Ok(newone)
        } else {
            Err(HttpError::Buggy) // FIXME: long body
        }
    }
}

// HttpResponseParts //

pub(crate) struct HttpResponseParts {
    httpver: Vec<u8>,
    status: Vec<u8>,
    headers: Vec<u8>,
    pub(crate) msgbody: Vec<u8>,
    msgbody_persist: Option<String>,
}

#[allow(unused)]
impl HttpResponseParts {
    // getter

    pub fn get_status(&self) -> Result<&[u8], HttpError> {
        Ok(&self.status)
    }

    pub fn get_content_length(&self) -> Result<usize, HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"C" || &buf[curi..curi + 1] == b"c" {
                let h_name = b"Content-Length:";
                let h_name_alt = b"content-length:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok((String::from_utf8_lossy(&buf[hdl_val_begi..hdl_val_endi]))
                .trim()
                .parse::<usize>()
                .unwrap())

            // Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_connection(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"C" || &buf[curi..curi + 1] == b"c" {
                let h_name = b"Connection:";
                let h_name_alt = b"connection:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_upgrade(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"U" || &buf[curi..curi + 1] == b"u" {
                let h_name = b"Upgrade:";
                let h_name_alt = b"upgrade:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    pub fn get_ws_accept(&self) -> Result<&[u8], HttpError> {
        let buf = &self.headers;

        let mut curi = 0usize;
        let mut hdl_val_begi: usize;
        let mut hdl_val_endi: usize;

        loop {
            if curi >= self.headers.len() {
                return Err(HttpError::HeaderNotExistContentLength);
            }
            if &buf[curi..curi + 1] == b"S" || &buf[curi..curi + 1] == b"s" {
                let h_name = b"Sec-WebSocket-Accept:";
                let h_name_alt = b"sec-webSocket-accept:";
                let h_name_len = h_name.len();
                if &buf[curi..curi + h_name_len] == h_name
                    || &buf[curi..curi + h_name_len] == h_name_alt
                {
                    hdl_val_begi = curi + h_name_len;
                    // rfc2616-4.2:
                    // "The field-content does not include any leading or trailing LWS"
                    while self.headers[hdl_val_begi] == b' ' {
                        hdl_val_begi += 1;
                    }
                    curi = hdl_val_begi;
                    break;
                }
            }
            curi += 1;
        }

        loop {
            if buf[curi] == b'\r' {
                if buf[curi + 1] == b'\n' {
                    hdl_val_endi = curi;
                    // rfc2616-4.2
                    while self.headers[hdl_val_endi] == b' ' {
                        hdl_val_endi -= 1;
                    }
                    break;
                } else {
                    panic!("bug");
                }
            }
            curi += 1;
        }

        if hdl_val_endi - hdl_val_begi > 0 {
            Ok(&buf[hdl_val_begi..hdl_val_endi])
        } else {
            Err(HttpError::HeaderNotExistContentLength)
        }
    }

    // setter

    pub fn http1p1(&mut self) -> &mut Self {
        self.httpver.extend(b"HTTP/1.1");
        self
    }

    pub fn status(&mut self, status_reason: &[u8]) -> &mut Self {
        self.status.extend(status_reason);
        self
    }

    pub fn connection(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Connection: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn upgrade(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Upgrade: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    pub fn ws_accept(&mut self, n: &str) -> &mut Self {
        let hdrline = format!("Sec-WebSocket-Accept: {}\r\n", n).into_bytes();
        self.headers.extend(hdrline);
        self
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut req = vec![];

        req.extend(&self.httpver);
        req.extend(b" ");
        req.extend(&self.status);
        req.extend(b"\r\n");
        req.extend(&self.headers);
        req.extend(b"\r\n");
        req.extend(&self.msgbody);

        req
    }

    // old

    #[allow(dead_code)]
    pub fn headers(&self) -> &Vec<u8> {
        // self.headers.iter().map(|x| x).collect()
        &self.headers
    }

    pub fn msgbody(&self) -> &Vec<u8> {
        &self.msgbody
    }

    pub fn msgbody_persist(&self) -> Option<&str> {
        match &self.msgbody_persist {
            None => None,
            Some(m) => Some(m),
        }
    }

    ///
    /// Check response status code is 200.
    pub fn is_ok(&self) -> bool {
        &self.status == b"200 OK"
    }

    pub fn from_scratch() -> Self {
        HttpResponseParts {
            httpver: vec![],
            status: vec![],
            headers: vec![],
            msgbody: vec![],
            msgbody_persist: None,
        }
    }

    ///
    /// Contruct an response instance from TcpStream.
    ///
    /// The second parameter stands for "persistent body path", which is
    /// a path to a file whose content is the response body got from `stream`.
    /// The `write_after` controls the number of bytes from start of original
    /// response body should be ignored, the `insig_tail` indicates the number
    /// of bytes from the end of original response body should be ignored, these
    /// two are accomplish differently to each other, the former is used
    /// while consuming the stream, the latter is used just before closing the
    /// persistent response body file.
    pub fn from_stream(
        stream: &mut TcpStream,
        pbody_path: Option<&str>,
        insig_head: usize,
        insig_tail: usize,
    ) -> Result<Self, HttpError> {
        let mut template = Self::from_scratch();
        let (mut curi, mut nexi) = (0, 0);
        const SZ_MAX_RBUF: usize = 8192;
        let mut rbuf = [0u8; SZ_MAX_RBUF]; // Read Buffer

        // Status Line

        // dbgg!(stream.nodelay().unwrap());

        // // http ident ver
        // run_diag!("http_ident", {
        //     (curi, nexi) = (0, 5);
        //     stream.read_exact(&mut rbuf[curi..nexi]).unwrap();
        //     if &rbuf[curi..nexi] != b"HTTP/" {
        //         return Err(HttpError::InvalidHttpData);
        //     }
        // });

        // http ver
        run_diag!("http_ver", {
            stream.read_exact(&mut rbuf[0..9]).unwrap(); // rfc2616-6.1, mandated SP
            match &rbuf[0..9] {
                b"HTTP/0.9 " | b"HTTP/1.0 " | b"HTTP/1.1 " | b"HTTP/2.0 " | b"HTTP/3.0 " => {
                    template.httpver.extend(&rbuf[0..8]);
                }
                _ => {
                    return Err(HttpError::InvalidHttpVersion);
                }
            };
        });

        // status code
        // run_diag!("status_code", {
        stream.read_exact(&mut rbuf[0..4]).unwrap(); // rfc2616-6.1, mandated SP

        // FIXME: skip verify content?
        template.status.extend(&rbuf[0..4]);
        // template.status.extend(match &rbuf[curi..nexi] {
        //     b"200 " => b"200",
        //     b"201 " => b"201",
        //     b"202 " => b"202",
        //     b"302 " => b"302",
        //     b"404 " => b"404",
        //     b"405 " => b"405",
        //     b"400 " => b"400",
        //     b"500 " => b"500",
        //     unknown => {
        //         dbgmsg!("unknown status code: {}", unknown);
        //         unknown
        //     }
        // });
        // });

        let (mut insig_head_tmp, insig_tail_tmp); // head maybe mutated
        if &template.status == b"200 " {
            (insig_head_tmp, insig_tail_tmp) = (insig_head, insig_tail);
        } else {
            (insig_head_tmp, insig_tail_tmp) = (0, 0);
        }

        // reason phase and crlf
        // let _reason_phase_begi = nexi;
        // let mut _reason_phase_endi = 0;
        // run_diag!("rea_phase", {
        let mut curi = 0usize;
        let reason_endi;
        loop {
            stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap();
            if &rbuf[curi..curi + 1] == b"\r" {
                curi += 1;
                stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap();
                if &rbuf[curi..curi + 1] == b"\n" {
                    reason_endi = curi - 1; // rfc2616-6.1, mandated SPC
                    break;
                } else {
                    dbgg!(&rbuf[curi..curi + 1], &template);
                    return Err(HttpError::InvalidHttpData);
                }
            } else {
                curi += 1;
            }
        }

        template.status.extend(&rbuf[0..reason_endi]);
        // });

        // dbgg!(_reason_phase_begi, _reason_phase_endi);

        // let _header_begi = nexi;
        // let mut _header_endi = 0;
        // run_diag!("headers", {
        let mut curi = 0usize;
        let headers_endi; // excluded
        loop {
            stream.read_exact(&mut rbuf[curi..curi + 1]).unwrap();
            if &rbuf[curi..curi + 1] == b"\r" {
                curi += 1;
                stream.read_exact(&mut rbuf[curi..curi + 3]).unwrap();
                if &rbuf[curi..curi + 3] == b"\n\r\n" {
                    curi += 3;
                    headers_endi = curi;
                    break;
                } else {
                    curi += 3;
                }
            } else {
                curi += 1;
            }
        }
        template.headers.extend(&rbuf[0..headers_endi]);

        // run_diag!("ctn_type", {
        //     match get_content_type(&template.headers) {
        //         Ok(_content_type) => {
        //             // fixme: use enum to save things
        //             dbgg!(_content_type);
        //         }
        //         Err(_err) => {
        //             dbgg!(_err);
        //         }
        //     }
        // });

        match template.get_content_length() {
            Ok(msgbody_len) => {
                dbgg!(msgbody_len);
                if pbody_path.is_none() {
                    // a trivial-size body
                    let msgbody_begi = 0;
                    let msgbody_endi = msgbody_len;

                    if msgbody_len <= SZ_MAX_RBUF {
                        stream
                            .read_exact(&mut rbuf[msgbody_begi..msgbody_endi])
                            .unwrap();
                        template.msgbody.extend(
                            &rbuf[msgbody_begi + insig_head_tmp..msgbody_endi - insig_tail_tmp],
                        );
                    } else {
                        let mut big_rbuf = Vec::<u8>::with_capacity(msgbody_len);
                        big_rbuf.resize(msgbody_len, 0);
                        if big_rbuf.len() < msgbody_len {
                            panic!("buggy");
                        }
                        stream
                            .read_exact(&mut big_rbuf[msgbody_begi..msgbody_endi])
                            .unwrap();
                        template.msgbody.extend(
                            &big_rbuf[msgbody_begi + insig_head_tmp..msgbody_endi - insig_tail_tmp],
                        );
                    }
                } else {
                    // a non-trivial body
                    let pbody_path = pbody_path.unwrap();
                    let pbody_file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        // .append(true)
                        .open(pbody_path);
                    if let Err(_e) = pbody_file {
                        return Err(HttpError::PersistBodyPathNotFound);
                    }
                    let mut pbody_file = pbody_file.unwrap();

                    let mut nleft = msgbody_len;
                    // read from socket, then write to local storage
                    let mut read_buf = [0u8; 1024 * 1024 * 1];

                    // diagnostic only variables
                    let mut _n_sys_socket_read = 0usize;
                    let mut _n_sys_disk_write = 0usize;

                    loop {
                        if nleft == 0 {
                            break;
                        }
                        dbgg!(nleft);
                        match stream.read(&mut read_buf) {
                            Ok(nread) => {
                                dbgg!(nread);
                                if nread > 0 {
                                    {
                                        _n_sys_socket_read += 1;
                                    }

                                    // dbgg!(nleft, nread, insig_head_tmp);
                                    if nleft < nread {
                                        return Err(HttpError::InvalidContentLength);
                                    }
                                    nleft -= nread;

                                    if insig_head_tmp > nread {
                                        // dbgg!("skip persist write", insig_head_tmp,
                                        // nread);
                                        insig_head_tmp -= nread;
                                        continue;
                                    }
                                    if let Err(_e) =
                                        pbody_file.write_all(&read_buf[insig_head_tmp..nread])
                                    {
                                        dbgg!(_e);
                                        return Err(HttpError::PersistBodyWrite);
                                    }
                                    insig_head_tmp = 0;
                                    {
                                        _n_sys_disk_write += 1;
                                    }
                                } else {
                                    // ==0 maybe remote close conn correctly, fixme to handle
                                    // this
                                    dbgg!("mabybe remote close conn");
                                    break;
                                }
                            }
                            Err(_e) => {
                                dbgg!("stream read failed", _e);
                                break;
                            }
                        }
                    } // loop

                    if nleft == 0 {
                        dbgg!("all data read done", _n_sys_socket_read, _n_sys_disk_write);
                        template.msgbody_persist = Some(pbody_path.to_string());
                        // start to truncate from file tail
                        let final_fsize = msgbody_len - insig_head - insig_tail;
                        dbgg!(msgbody_len, insig_head_tmp, final_fsize);
                        pbody_file
                            .set_len(final_fsize as u64)
                            .expect("failed to truncate file");
                    } else {
                        dbgg!(nleft);
                        return Err(HttpError::IncompleteFinish);
                    }
                }
            }
            Err(_err) => {
                dbgg!(_err);
            }
        }
        // });

        Ok(template)
    } // from_stream

    pub fn send_through(&self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let wbuf = self.to_vec();
        stream.write_all(&wbuf).unwrap();
        Ok(())
    }
}

impl std::fmt::Debug for HttpResponseParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "HttpResponseParts ( httpver:{} , status:{} , headers:{} , msgbody:{} )",
            String::from_utf8_lossy(&self.httpver),
            String::from_utf8_lossy(&self.status),
            String::from_utf8_lossy(&self.headers),
            String::from_utf8_lossy(&self.msgbody),
        )
    }
}

impl std::fmt::Debug for HttpRequestParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "HttpRequestParts (method:{},requri:{},httpver:{},headers:{},msgbody:{})",
            String::from_utf8_lossy(&self.method),
            String::from_utf8_lossy(&self.requri),
            String::from_utf8_lossy(&self.httpver),
            String::from_utf8_lossy(&self.headers),
            String::from_utf8_lossy(&self.msgbody),
        )
    }
}

// UNIT TEST //

#[cfg(test)]
mod utst {

    // use super::*; // not strictly unit tests, but along with integrated ones.

    use super::HttpRequestParts;
    use super::HttpResponseParts;

    use std::fs::OpenOptions;
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread;

    #[test]
    fn _1() {
        let mut req = HttpRequestParts::from_scratch();

        req.http1p1()
            .get("/statusss")
            .host("127.0.0.1:4444")
            .ua("fakecurl/1.1.0")
            .msgbody_from_slice(b"{ capalibities: {} }");

        let reqstr = req.to_vec();

        assert_eq!(
            &reqstr,
            b"GET /statusss HTTP/1.1\r\n\
	     Host: 127.0.0.1:4444\r\n\
	     User-Agent: fakecurl/1.1.0\r\n\
	     Content-Length: 20\r\n\
	     \r\n\
	     { capalibities: {} }"
        );
    }

    #[test]
    fn _11() {
        let mut req = HttpRequestParts::from_scratch();
        req.http1p1()
            .get("/statusss")
            .ua("fakecurl/1.1.0")
            .host("127.0.0.1:4444")
            .msgbody_from_slice(b"{ capalibities: { } }");

        let reqstr = req.to_vec();

        assert_eq!(
            &reqstr,
            b"GET /statusss HTTP/1.1\r\n\
	     User-Agent: fakecurl/1.1.0\r\n\
	     Host: 127.0.0.1:4444\r\n\
	     Content-Length: 21\r\n\
	     \r\n\
	     { capalibities: { } }"
        );
    }

    #[test]
    fn _4() {
        // send_through

        let (remote_host, remote_port) = ("127.0.0.1", "23457");
        let remote_sockaddr = format!("{}:{}", remote_host, remote_port);

        // Server
        let listener = TcpListener::bind(&remote_sockaddr).expect("Server Up");

        // Client
        match TcpStream::connect(&remote_sockaddr) {
            Ok(mut stream) => {
                let mut req = HttpRequestParts::from_scratch();
                req.http1p1()
                    .get("/statuss")
                    .ua("wda/1.1.1234")
                    .host("127.0.0.1:4444")
                    .msgbody_from_slice(b"{ capalibities: { } }");
                req.send_through(&mut stream).unwrap();
            }
            Err(_) => {
                panic!("Server Not Up")
            }
        }

        // Server
        let conn = listener.incoming().next().unwrap();
        match conn {
            Ok(mut stream) => {
                let expected_comming_request = "GET /statuss HTTP/1.1\r\n\
						User-Agent: wda/1.1.1234\r\n\
						Host: 127.0.0.1:4444\r\n\
						Content-Length: 21\r\n\
						\r\n\
						{ capalibities: { } }";
                let mut rbuf = vec![0u8; expected_comming_request.len()];
                stream.read_exact(&mut rbuf).unwrap();
                // dbgg!(&rbuf);
                assert_eq!(
                    &std::str::from_utf8(&rbuf).unwrap(),
                    &expected_comming_request
                );
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn _5() {
        // from_stream

        let (remote_host, remote_port) = ("127.0.0.1", "23456");
        let remote_sockaddr = format!("{}:{}", remote_host, remote_port);

        // Server
        let listener = TcpListener::bind(&remote_sockaddr).expect("server up failed");
        let server_thread = thread::spawn(move || {
            let a_http_resp = b"HTTP/1.1 200 OK\r\n\
			    content-type: application/json; charset=utf-8\r\n\
			    cache-control: no-cache\r\n\
			    content-length: 37\r\n\
			    date: Wed, 01 Mar 2023 09:29:00 GMT\r\n\
			    \r\n\
			    {\"value\":{\"message\":\"\",\"ready\":true}}";

            let conn = listener
                .incoming()
                .next()
                .expect("Should Have One Incoming Conn");

            match conn {
                Ok(mut stream) => {
                    stream.write_all(a_http_resp).unwrap();
                }
                Err(_) => {
                    panic!("Connection to Server Failed");
                }
            }
        });

        // Client
        match TcpStream::connect(&remote_sockaddr) {
            Ok(mut stream) => {
                let resp = HttpResponseParts::from_stream(&mut stream, None, 0, 0).unwrap();
                assert_eq!(resp.httpver, b"HTTP/1.1");
                assert_eq!(&resp.status, b"200 OK");
                assert_eq!(
                    &resp.headers,
                    b"content-type: application/json; charset=utf-8\r\n\
		      cache-control: no-cache\r\n\
		      content-length: 37\r\n\
		      date: Wed, 01 Mar 2023 09:29:00 GMT\r\n\
		      \r\n"
                );
                assert_eq!(
                    std::str::from_utf8(&resp.msgbody).unwrap(),
                    "{\"value\":{\"message\":\"\",\"ready\":true}}"
                );

                //
                // Direct byte-wide comparison has little benefit to diagnostic.
                //
                //     assert_eq!(
                //         &resp.msgbody,
                //         b"{\"value\":{\"message\":\"\",\"ready\":true}}"
                //     );
            }
            Err(err) => {
                assert!(false, "{:?}", err);
            }
        }

        server_thread.join().unwrap();
    }

    #[test]
    fn _55() {
        // start offset is not 0, but 10, simulating '{"value":{' string

        const RANDOM_FILE: &str = "wdctmp/sample2-bigrandom";
        const RESP_FILE: &str = "wdctmp/sample2-bigresp";
        const BODYGOT_FILE: &str = "wdctmp/sample2-bodygot";
        const SZ_RANDOM: usize = 50 * 1024 * 1024;
        const SZ_BODY_START: usize = 10;
        let (remote_host, remote_port) = ("127.0.0.1", "23458");
        let remote_sockaddr = format!("{}:{}", remote_host, remote_port);

        // prepare local cache files. FIXME: this could be a seperate fn
        {
            let mut resp_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RESP_FILE)
                .unwrap();

            let mut rand_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RANDOM_FILE)
                .unwrap();

            // append http resp header to resp file
            resp_file.write_all(format!("HTTP/1.1 200 OK\r\ncontent-type: application/json; charset=utf-8\r\ncontent-length: {}\r\n\r\n", SZ_BODY_START + SZ_RANDOM
            ).as_bytes()).unwrap();

            // append body start noisy characters to resp file
            resp_file.write_all(&[0u8; SZ_BODY_START]).unwrap();

            // generate random data file
            rand_file.write_all(&[33u8; SZ_RANDOM]).unwrap();

            drop(resp_file);
            drop(rand_file);

            let mut rand_file = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
            let mut resp_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(RESP_FILE)
                .unwrap();

            // append random data to http resp header
            let mut nread = 1;
            let mut rbuf = [0u8; 4096];
            while nread > 0 {
                nread = rand_file.read(&mut rbuf).unwrap();
                resp_file.write_all(&rbuf[0..nread]).unwrap();
            }
        }

        // Server
        let listener = TcpListener::bind(&remote_sockaddr).expect("server up failed");
        let server_thread = thread::spawn(move || {
            let mut read_buf = [0u8; 1024];
            let mut read_cache = Vec::<u8>::with_capacity(10 * 1024 * 1024);
            let mut tread = 0usize;

            let mut f = OpenOptions::new().read(true).open(RESP_FILE).unwrap();
            loop {
                match f.read(&mut read_buf) {
                    Ok(nread) => {
                        if nread > 0 {
                            tread += nread;
                            read_cache.extend(&read_buf);
                        } else {
                            break;
                        }
                    }
                    Err(_e) => {
                        dbgg!(_e);
                        break;
                    }
                }
            }

            dbgg!(tread);

            let conn = listener
                .incoming()
                .next()
                .expect("fail to get incoming connection");

            dbgg!("a conn!!!");

            match conn {
                Ok(mut stream) => {
                    stream.write_all(&read_cache[0..tread]).unwrap();
                }
                Err(_) => {
                    panic!("Connection to Server Failed");
                }
            }
        });

        // no need to wait inside same parent process
        // std::thread::sleep(std::time::Duration::from_secs(2));

        // Client
        match TcpStream::connect(&remote_sockaddr) {
            Ok(mut stream) => {
                let resp = HttpResponseParts::from_stream(
                    &mut stream,
                    Some(BODYGOT_FILE),
                    SZ_BODY_START,
                    0,
                )
                .unwrap();
                assert_eq!(resp.httpver, b"HTTP/1.1");

                assert_eq!(&resp.status, b"200 OK");
                assert_eq!(
                    &resp.headers,
                    &format!(
                        "content-type: application/json; charset=utf-8\r\n\
        		 content-length: {}\r\n\
        		 \r\n",
                        SZ_RANDOM + SZ_BODY_START
                    )
                    .as_bytes()
                );
                assert_eq!(resp.msgbody_persist().is_some(), true);
                assert_eq!(resp.msgbody_persist().unwrap(), BODYGOT_FILE);

                let mut bodydata = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
                let mut bodygot = OpenOptions::new().read(true).open(BODYGOT_FILE).unwrap();
                let mut buf1 = [0u8; 1024];
                let mut buf2 = [0u8; 1024];
                loop {
                    if let Err(e) = bodydata.read_exact(&mut buf1) {
                        assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        break;
                    }
                    if let Err(e) = bodygot.read_exact(&mut buf2) {
                        assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        // break;
                    }
                    assert_eq!(buf1, buf2);
                }
            }
            Err(err) => {
                assert!(false, "{:?}", err);
            }
        }

        server_thread.join().unwrap();
    }

    #[test]
    fn _555() {
        // start offset is larger than one socket read buffer, that is,
        // insig_head_tmp>>nread; not likely happen in practice, but not
        // impossible

        const RANDOM_FILE: &str = "wdctmp/sample3-bigrandom";
        const RESP_FILE: &str = "wdctmp/sample3-bigresp";
        const BODYGOT_FILE: &str = "wdctmp/sample3-bodygot";
        const SZ_RANDOM: usize = 1 * 1024;
        const SZ_BODY_START: usize = 1024 * 1024;
        let (remote_host, remote_port) = ("127.0.0.1", "23459");
        let remote_sockaddr = format!("{}:{}", remote_host, remote_port);

        // prepare local cache files
        {
            let mut resp_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RESP_FILE)
                .unwrap();

            let mut rand_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RANDOM_FILE)
                .unwrap();

            // append http resp header to resp file
            resp_file.write_all(format!("HTTP/1.1 200 OK\r\ncontent-type: application/json; charset=utf-8\r\ncontent-length: {}\r\n\r\n", SZ_BODY_START + SZ_RANDOM
            ).as_bytes()).unwrap();

            // append body start noisy characters to resp file
            resp_file.write_all(&[0u8; SZ_BODY_START]).unwrap();

            // generate random data file
            rand_file.write_all(&[33u8; SZ_RANDOM]).unwrap();

            drop(resp_file);
            drop(rand_file);

            let mut rand_file = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
            let mut resp_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(RESP_FILE)
                .unwrap();

            // append random data to http resp header
            let mut nread = 1;
            let mut rbuf = [0u8; 4096];
            while nread > 0 {
                nread = rand_file.read(&mut rbuf).unwrap();
                resp_file.write_all(&rbuf[0..nread]).unwrap();
            }
        }

        // Server
        let listener = TcpListener::bind(&remote_sockaddr).expect("server up failed");
        let server_thread = thread::spawn(move || {
            let mut read_buf = [0u8; 1024];
            let mut read_cache = Vec::<u8>::with_capacity(10 * 1024 * 1024);
            let mut tread = 0usize;

            let mut f = OpenOptions::new().read(true).open(RESP_FILE).unwrap();
            loop {
                match f.read(&mut read_buf) {
                    Ok(nread) => {
                        if nread > 0 {
                            tread += nread;
                            read_cache.extend(&read_buf);
                        } else {
                            break;
                        }
                    }
                    Err(_e) => {
                        dbgg!(_e);
                        break;
                    }
                }
            }

            dbgg!(tread);

            let conn = listener
                .incoming()
                .next()
                .expect("fail to get incoming connection");

            dbgg!("a conn!!!");

            match conn {
                Ok(mut stream) => {
                    stream.write_all(&read_cache[0..tread]).unwrap();
                }
                Err(_) => {
                    panic!("Connection to Server Failed");
                }
            }
        });

        // no need to wait inside same parent process
        // std::thread::sleep(std::time::Duration::from_secs(2));

        // Client
        match TcpStream::connect(&remote_sockaddr) {
            Ok(mut stream) => {
                let resp = HttpResponseParts::from_stream(
                    &mut stream,
                    Some(BODYGOT_FILE),
                    SZ_BODY_START,
                    0,
                )
                .unwrap();
                assert_eq!(resp.httpver, b"HTTP/1.1");

                assert_eq!(&resp.status, b"200 OK");
                assert_eq!(
                    &resp.headers,
                    &format!(
                        "content-type: application/json; charset=utf-8\r\n\
        		 content-length: {}\r\n\
        		 \r\n",
                        SZ_RANDOM + SZ_BODY_START
                    )
                    .as_bytes()
                );
                assert_eq!(resp.msgbody_persist().is_some(), true);
                assert_eq!(resp.msgbody_persist().unwrap(), BODYGOT_FILE);

                let mut bodydata = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
                let mut bodygot = OpenOptions::new().read(true).open(BODYGOT_FILE).unwrap();
                let mut buf1 = [0u8; 1024];
                let mut buf2 = [0u8; 1024];
                loop {
                    if let Err(e) = bodydata.read_exact(&mut buf1) {
                        assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        break;
                    }
                    if let Err(e) = bodygot.read_exact(&mut buf2) {
                        assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        // break;
                    }
                    assert_eq!(buf1, buf2);
                }
            }
            Err(err) => {
                assert!(false, "{:?}", err);
            }
        }

        server_thread.join().unwrap();
    }

    #[test]
    fn _5555() {
        // start offset is not 0, but 10, simulating '{"value":{' string
        // the tail 10 bytes are dismissed.

        const RANDOM_FILE: &str = "wdctmp/sample4-bigrandom"; //  random data,
                                                              // would be eventual
                                                              // resp body content
        const RESP_FILE: &str = "wdctmp/sample4-bigresp"; //  response headers
                                                          // file
        const BODYGOT_FILE: &str = "wdctmp/sample4-bodygot"; // response got by client,
                                                             // through connection from server
        const SZ_RANDOM: usize = 1024 * 10; // the random data size in byte
        const SZ_BODY_START: usize = 10; // response body start size in byte
        const SZ_DISMISS_TAIL: usize = 10; // the useless tail size in byte
        let (remote_host, remote_port) = ("127.0.0.1", "23460");
        let remote_sockaddr = format!("{}:{}", remote_host, remote_port);

        // prepare local cache files
        {
            let mut resp_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RESP_FILE)
                .unwrap();

            let mut rand_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(RANDOM_FILE)
                .unwrap();

            // append http resp header to resp file
            resp_file.write_all(format!("HTTP/1.1 200 OK\r\ncontent-type: application/json; charset=utf-8\r\ncontent-length: {}\r\n\r\n", SZ_BODY_START + SZ_RANDOM
            ).as_bytes()).unwrap();

            // append body start noisy characters to resp file
            resp_file.write_all(&[0u8; SZ_BODY_START]).unwrap();

            // generate random data file
            rand_file.write_all(&[33u8; SZ_RANDOM]).unwrap();

            drop(resp_file);
            drop(rand_file);

            let mut rand_file = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
            let mut resp_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(RESP_FILE)
                .unwrap();

            // append random data to http resp header
            let mut nread = 1;
            let mut rbuf = [0u8; 4096];
            while nread > 0 {
                nread = rand_file.read(&mut rbuf).unwrap();
                resp_file.write_all(&rbuf[0..nread]).unwrap();
            }
        }

        // Server
        let listener = TcpListener::bind(&remote_sockaddr).expect("server up failed");
        let server_thread = thread::spawn(move || {
            let mut read_buf = [0u8; 1024];
            let mut read_cache = Vec::<u8>::with_capacity(10 * 1024 * 1024);
            let mut tread = 0usize;

            let mut f = OpenOptions::new().read(true).open(RESP_FILE).unwrap();
            loop {
                match f.read(&mut read_buf) {
                    Ok(nread) => {
                        if nread > 0 {
                            tread += nread;
                            read_cache.extend(&read_buf);
                        } else {
                            break;
                        }
                    }
                    Err(_e) => {
                        dbgg!(_e);
                        break;
                    }
                }
            }

            dbgg!(tread);

            let conn = listener
                .incoming()
                .next()
                .expect("fail to get incoming connection");

            dbgg!("a conn!!!");

            match conn {
                Ok(mut stream) => {
                    stream.write_all(&read_cache[0..tread]).unwrap();
                }
                Err(_) => {
                    panic!("Connection to Server Failed");
                }
            }
        });

        // Client
        match TcpStream::connect(&remote_sockaddr) {
            Ok(mut stream) => {
                let resp = HttpResponseParts::from_stream(
                    &mut stream,
                    Some(BODYGOT_FILE),
                    SZ_BODY_START,
                    SZ_DISMISS_TAIL,
                )
                .unwrap();
                assert_eq!(resp.httpver, b"HTTP/1.1");

                assert_eq!(&resp.status, b"200 OK");
                assert_eq!(
                    &resp.headers,
                    &format!(
                        "content-type: application/json; charset=utf-8\r\n\
        		 content-length: {}\r\n\
        		 \r\n",
                        SZ_RANDOM + SZ_BODY_START
                    )
                    .as_bytes()
                );
                assert_eq!(resp.msgbody_persist().is_some(), true);
                assert_eq!(resp.msgbody_persist().unwrap(), BODYGOT_FILE);

                const SZ_FRAG: usize = 256;
                const FRAG_COUNT: usize = SZ_RANDOM / SZ_FRAG; // careful, should x%y==0
                let mut bodydata = OpenOptions::new().read(true).open(RANDOM_FILE).unwrap();
                let mut bodygot = OpenOptions::new().read(true).open(BODYGOT_FILE).unwrap();
                // compare fragment by fragment
                for i in 0..FRAG_COUNT {
                    let mut buf1 = [0u8; SZ_FRAG]; // for body data
                    let mut buf2 = [0u8; SZ_FRAG]; // for body got
                    if let Err(_e) = bodydata.read_exact(&mut buf1) {
                        // assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        assert!(false); // body data is longer, reading should always be error-free
                        break;
                    }
                    if let Err(e) = bodygot.read_exact(&mut buf2) {
                        assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                        // break;
                    }

                    if i != FRAG_COUNT - 1 {
                        // compare byte by byte
                        assert_eq!(buf1, buf2, "fragment number:{}", i);
                    } else {
                        // compare byte by byte
                        assert_eq!(
                            buf1[0..SZ_FRAG - SZ_DISMISS_TAIL],
                            buf2[0..SZ_FRAG - SZ_DISMISS_TAIL],
                            "fragment number:{}",
                            i
                        );
                    }
                }
            }
            Err(err) => {
                assert!(false, "{:?}", err);
            }
        }

        server_thread.join().unwrap();
    }
}
