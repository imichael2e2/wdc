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

use crate::httpp::HttpRequestParts;
use crate::httpp::HttpResponseParts;

use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum WspSett {
    Mask,
    Ping,
    Pong,
    TextMsg,
    BinaryMsg,
    MaxFrameLen(usize),
}

#[derive(Debug, PartialEq)]
pub enum WspError {
    Buggy,
    InvalidMaxFrameLen,
    InvalidDataLen,
    InsufficientSize,
    SizeKindNotFound,
    HandshakeFail1,
    HandshakeFail2,
}

pub struct WebSocketHandshaker;
impl WebSocketHandshaker {
    pub fn try_as_client(stream: &mut TcpStream, uri: &str, host: &str) -> Result<(), WspError> {
        let mut req = HttpRequestParts::from_scratch();
        req.get(uri)
            .http1p1()
            .connection("Upgrade")
            .upgrade("websocket")
            .host(host)
            .ws_key("aG93LXRvLWhhbmRzaGFrZQ==")
            .ws_ver("13");
        req.send_through(stream).unwrap();

        let resp = HttpResponseParts::from_stream(stream, None, 0, 0).unwrap();

        match resp.get_status() {
            Ok(status_reason) => {
                if !(status_reason == b"101 Switching Protocols"
                    || status_reason == b"101 WebSocket Protocol Handshake")
                {
                    dbgg!(String::from_utf8_lossy(status_reason));
                    return Err(WspError::HandshakeFail1);
                }
            }
            Err(_e) => {
                dbgg!(_e);
                return Err(WspError::HandshakeFail2);
            }
        }

        match resp.get_ws_accept() {
            Ok(acc) => {
                if acc != b"GsCYk86TcY3D9uBDLZuG5FmeV3Y=" {
                    dbgg!(String::from_utf8_lossy(acc));
                    return Err(WspError::HandshakeFail1);
                }
            }
            Err(_e) => {
                dbgg!(_e);
                return Err(WspError::HandshakeFail2);
            }
        }

        Ok(())
    }
}

// ------------------------------------------------- WebSocketMessage ------ //

#[derive(Debug)]
pub struct WebSocketMessage {
    size_kinds: [bool; 3],
    frames: Vec<WebSocketFrame>,
}

impl WebSocketMessage {
    pub fn new() -> Self {
        WebSocketMessage {
            size_kinds: [false, false, false],
            frames: vec![],
        }
    }

    pub fn allow_small(&mut self) -> &mut Self {
        self.size_kinds[0] = true;
        self
    }

    pub fn allow_medium(&mut self) -> &mut Self {
        self.size_kinds[1] = true;
        self
    }

    pub fn allow_large(&mut self) -> &mut Self {
        self.size_kinds[2] = true;
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut big_buf = Vec::<u8>::new();
        for f in &self.frames {
            big_buf.extend(f.to_vec());
        }

        big_buf
    }

    fn get_default_max_flen(&self) -> usize {
        let allow_s = self.size_kinds[0];
        let allow_m = self.size_kinds[1];
        let allow_l = self.size_kinds[2];

        // pick smaller
        if allow_s {
            125
        } else if allow_m {
            u16::MAX as usize
        } else if allow_l {
            u64::MAX as usize
        } else {
            0
        }
    }

    pub fn get_message_data(&self) -> Result<Vec<u8>, WspError> {
        let mut ret = Vec::<u8>::new();

        for f in &self.frames {
            match f.size_kind {
                SizeKind::S => {
                    if f.plen > 125 {
                        return Err(WspError::InvalidDataLen);
                    }
                }
                SizeKind::M => {
                    if f.plen > u16::MAX as u64 {
                        return Err(WspError::InvalidDataLen);
                    }
                }
                _ => {}
            }

            ret.extend(&f.payload);
        }

        Ok(ret)
    }

    #[allow(unused_assignments)]
    pub fn set_message_data(
        &mut self,
        bin_data: &[u8],
        setts: Vec<WspSett>,
    ) -> Result<(), WspError> {
        let all_bytes = bin_data;
        let total_len = all_bytes.len();

        let mut is_text_msg = false; // 1:text 2:bin
        let mut max_flen = self.get_default_max_flen();
        let mut need_mask = false;
        let mut is_ping = false;
        let mut is_pong = false;

        for sett in setts {
            match sett {
                WspSett::TextMsg => {
                    is_text_msg = true;
                }
                WspSett::BinaryMsg => {
                    is_text_msg = false;
                }
                WspSett::Ping => {
                    is_ping = true;
                }
                WspSett::Pong => {
                    is_pong = true;
                }
                WspSett::Mask => {
                    need_mask = true;
                }
                WspSett::MaxFrameLen(ml) => {
                    max_flen = ml;
                } // _ => {}
            }
        }

        if max_flen == 0 {
            return Err(WspError::InvalidMaxFrameLen);
        }

        if total_len == 0 {
            return Err(WspError::InvalidDataLen);
        }

        let mut framed_len = 0;
        let mut nexf_starti = 0;
        let mut curf_starti = 0;
        while framed_len != total_len {
            let rest_len = total_len - framed_len;

            let actual_flen = if rest_len > max_flen {
                max_flen
            } else {
                rest_len
            };

            let mut f = WebSocketFrame::with_size_kinds(self.size_kinds, actual_flen)?;
            // dbgg!(&f);

            curf_starti = nexf_starti;
            nexf_starti += actual_flen;
            if curf_starti == 0 {
                // 1st frame
                if is_text_msg {
                    f.set_text_frame();
                } else {
                    f.set_binary_frame();
                }

                if is_ping {
                    f.set_ping();
                }
                if is_pong {
                    f.set_pong();
                }
            }

            f.set_payload(&all_bytes[curf_starti..nexf_starti])?;
            if need_mask {
                f.set_mask();
            }

            f.set_plen(actual_flen)?;

            framed_len += actual_flen;
            if framed_len == total_len {
                f.set_fin();
            }

            self.frames.push(f); // into message
        }

        Ok(())
    }

    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, WspError> {
        let mut newmsg = WebSocketMessage::new();
        newmsg.allow_small().allow_medium().allow_large();

        // let mut rbuf = [0u8; 1024];
        let mut rbuf = Vec::<u8>::new();
        rbuf.resize(1024, 0);
        let mut is_all_frames_read = false;

        while !is_all_frames_read {
            let mut newframe = WebSocketFrame::new();

            stream.read_exact(&mut rbuf[0..2]).unwrap();

            let opcode = rbuf[0];
            let plen_s = rbuf[1];

            let is_fin = (opcode & 0b1000_0000) == 0b1000_0000;
            let is_mask = (plen_s & 0b1000_0000) == 0b1000_0000;
            let plen_hint = plen_s & 0b0111_1111;

            dbgg!(is_fin, is_mask, plen_hint);

            let plen: usize = if plen_hint < 126 {
                newframe.size_kind = SizeKind::S; // !
                plen_hint as usize
            } else if plen_hint == 126 {
                stream.read_exact(&mut rbuf[0..2]).unwrap();
                newframe.size_kind = SizeKind::M; // !
                u16::from_be_bytes([rbuf[0], rbuf[1]]) as usize
            } else if plen_hint == 127 {
                stream.read_exact(&mut rbuf[0..8]).unwrap();
                newframe.size_kind = SizeKind::L; // !
                u64::from_be_bytes([
                    rbuf[0], rbuf[1], rbuf[2], rbuf[3], rbuf[4], rbuf[5], rbuf[6], rbuf[7],
                ]) as usize
            } else {
                panic!("corrupt frame")
            };

            dbgg!(plen);
            newframe.plen = plen as u64; // !

            let mut mask_key = 0u32;
            if is_mask {
                stream.read_exact(&mut rbuf[0..4]).unwrap();
                mask_key = u32::from_be_bytes([rbuf[0], rbuf[1], rbuf[2], rbuf[3]]);
                newframe.mkey = Some(mask_key); // !
            }

            if plen > rbuf.len() {
                rbuf.resize(plen, 0);
            }
            stream.read_exact(&mut rbuf[0..plen]).unwrap();

            let mut pl_data = Vec::<u8>::new();
            pl_data.extend(&rbuf[0..plen]);
            if is_mask {
                let mask_key_bytes = mask_key.to_be_bytes();
                for i in 0..pl_data.len() {
                    let j = i % 4;
                    let nb = pl_data[i] ^ mask_key_bytes[j];
                    pl_data[i] = nb;
                }
            }

            // dbgg!(String::from_utf8_lossy(&pl_data));

            newframe.payload.extend(&pl_data); // !

            newmsg.frames.push(newframe);

            if is_fin {
                is_all_frames_read = true;
            }
        }

        Ok(newmsg)
    }

    pub fn send_through(&self, stream: &mut TcpStream) -> Result<(), WspError> {
        let bytes = self.to_vec();
        stream.write_all(&bytes[..]).unwrap();
        Ok(())
    }
}

// --------------------------------------------------- WebSocketFrame ------ //

#[derive(Debug)]
struct WebSocketFrame {
    size_kind: SizeKind,
    //
    ops: u8,
    plen: u64,
    mkey: Option<u32>,
    payload: Vec<u8>, // always unmasked
}

#[derive(Debug)]
enum SizeKind {
    S,
    M,
    L,
}

#[allow(unused)]
impl WebSocketFrame {
    fn new() -> Self {
        WebSocketFrame {
            size_kind: SizeKind::L,
            //
            ops: 0,
            plen: 0,
            mkey: None,
            payload: vec![],
        }
    }

    pub fn with_size_kinds(
        kinds: [bool; 3],
        actual_flen: usize,
    ) -> Result<WebSocketFrame, WspError> {
        let allow_s = kinds[0];
        let allow_m = kinds[1];
        let allow_l = kinds[2];

        let mut newone = WebSocketFrame {
            ops: 0,
            size_kind: SizeKind::S,
            plen: 0,
            mkey: None,
            payload: vec![],
        };

        // always use the smallest kind
        if actual_flen <= 125 {
            // [0,125]
            if allow_s {
                newone.size_kind = SizeKind::S;
            } else if allow_m {
                newone.size_kind = SizeKind::M;
            } else if allow_l {
                newone.size_kind = SizeKind::L;
            } else {
                return Err(WspError::SizeKindNotFound);
            }
        } else if actual_flen <= u16::MAX as usize {
            // [126,65535]
            if allow_m {
                newone.size_kind = SizeKind::M;
            } else if allow_l {
                newone.size_kind = SizeKind::L;
            } else {
                return Err(WspError::SizeKindNotFound);
            }
        } else if actual_flen <= u64::MAX as usize {
            // [65536,1<<65-1]
            if allow_l {
                newone.size_kind = SizeKind::L;
            } else {
                return Err(WspError::SizeKindNotFound);
            }
        } else {
            return Err(WspError::Buggy);
        }

        Ok(newone)
    }

    pub fn set_fin(&mut self) -> &mut Self {
        self.ops |= 0b10000000;
        self
    }

    pub fn set_close(&mut self) -> &mut Self {
        self.ops &= 0b11111000;
        self
    }

    pub fn set_ping(&mut self) -> &mut Self {
        self.ops &= 0b1111_0000;
        self.ops |= 0b0000_1001;
        self
    }

    pub fn set_pong(&mut self) -> &mut Self {
        self.ops &= 0b1111_0000;
        self.ops |= 0b0000_1010;
        self
    }

    pub fn set_text_frame(&mut self) -> &mut Self {
        self.ops &= 0b1111_0000;
        self.ops |= 0b0000_0001;
        self
    }

    pub fn set_binary_frame(&mut self) -> &mut Self {
        self.ops &= 0b1111_0000;
        self.ops |= 0b0000_0010;
        self
    }

    fn is_mask(&self) -> bool {
        self.mkey.is_some()
    }

    pub fn set_mask(&mut self) -> &mut Self {
        // pre-define, is wrong!
        self.mkey = Some(u32::from_be_bytes([0x37, 0xfa, 0x21, 0x3d]));

        self
    }

    pub fn set_plen(&mut self, len: usize) -> Result<&mut Self, WspError> {
        match self.size_kind {
            SizeKind::S => {
                if len > 125usize {
                    return Err(WspError::InsufficientSize);
                }
            }
            SizeKind::M => {
                if len > u16::MAX as usize {
                    return Err(WspError::InsufficientSize);
                }
            }
            SizeKind::L => {}
        }

        self.plen = len as u64;

        Ok(self)
    }

    pub fn set_payload(&mut self, bytes: &[u8]) -> Result<&mut Self, WspError> {
        let len = bytes.len();
        match self.size_kind {
            SizeKind::S => {
                if len > 125 {
                    return Err(WspError::InsufficientSize);
                }
            }
            SizeKind::M => {
                if len > u16::MAX as usize {
                    return Err(WspError::InsufficientSize);
                }
            }
            SizeKind::L => {}
        }

        self.payload.extend(bytes);

        Ok(self)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut ret = Vec::<u8>::new();

        // opcodes
        ret.push(self.ops);

        // payload length
        match self.size_kind {
            SizeKind::S => {
                let mut byte2 = self.plen as u8;
                if self.is_mask() {
                    byte2 |= 0b10000000;
                }
                ret.push(byte2);
            }
            SizeKind::M => {
                let mut byte2 = 126u8;
                if self.is_mask() {
                    byte2 |= 0b10000000;
                }
                ret.push(byte2);
                ret.extend((self.plen as u16).to_be_bytes());
            }
            SizeKind::L => {
                let mut byte2 = 127u8;
                if self.is_mask() {
                    byte2 |= 0b10000000;
                }
                ret.push(byte2);
                ret.extend((self.plen).to_be_bytes());
            }
        }

        // payload append
        let orig = &self.payload;
        if self.is_mask() {
            let mut masked = vec![];
            let mkey = self.mkey.unwrap();
            let mkey_bytes = mkey.to_be_bytes();
            let len = orig.len();
            for i in 0..len {
                let j = i % 4;
                let ob = orig[i];
                let nb: u8 = ob ^ mkey_bytes[j];
                masked.push(nb);
            }
            ret.extend(mkey.to_be_bytes());
            ret.extend(&masked);
        } else {
            ret.extend(orig);
        }

        ret
    }
}

#[cfg(test)]
mod utst {
    use super::*;

    #[test]
    fn _1() {
        // framing

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(bytes, vec![0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn _11() {
        // framing

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg, WspSett::Ping])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(bytes, vec![0x89, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn _111() {
        // framing, masked

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(
                b"Hello",
                vec![WspSett::TextMsg, WspSett::Pong, WspSett::Mask],
            )
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(
            bytes,
            vec![0x8a, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58]
        );
    }

    #[test]
    fn _1111() {
        // get message data

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg])
            .expect("set payload data");

        assert_eq!(wsmsg.frames.len(), 1);
        assert_eq!(&wsmsg.get_message_data().unwrap(), b"Hello");
    }

    #[test]
    fn _11111() {
        // get message data

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg, WspSett::MaxFrameLen(1)])
            .expect("set payload data");

        assert_eq!(wsmsg.frames.len(), 5);
        assert_eq!(&wsmsg.get_message_data().unwrap(), b"Hello");
    }

    #[test]
    fn _111111() {
        // get message data, data already unmasked

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(
                b"Hello",
                vec![WspSett::TextMsg, WspSett::MaxFrameLen(2), WspSett::Mask],
            )
            .expect("set payload data");

        assert_eq!(wsmsg.frames.len(), 3);
        assert_eq!(&wsmsg.get_message_data().unwrap(), b"Hello");
    }

    #[test]
    fn _1111111() {
        // frame data, accuracy

        let mut data_buf = vec![0xceu8; 65535];

        data_buf[100] = 0xec;
        data_buf[500] = 0xec;
        data_buf[1000] = 0xec;
        data_buf[5000] = 0xec;
        data_buf[65534] = 0xec;

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(&data_buf, vec![WspSett::TextMsg])
            .expect("set payload data");

        assert_eq!(wsmsg.frames.len(), 525); // 65535/125+1
        let bytes = wsmsg.to_vec(); // each frame is 2+125=127
        assert!(bytes[101] == bytes[103]); // 2+101=103->i is 102
        assert!(bytes[102] != bytes[103]);
        assert!(bytes[101] == 0xce);
        assert!(bytes[102] == 0xec);
    }

    #[test]
    fn _11111111() {
        // get message data, long msg

        let mut data_buf = vec![0xceu8; 65535];

        data_buf[100] = 0xec;
        data_buf[500] = 0xec;
        data_buf[1000] = 0xec;
        data_buf[5000] = 0xec;
        data_buf[65534] = 0xec;

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(&data_buf, vec![WspSett::TextMsg, WspSett::Mask])
            .expect("set payload data");

        assert_eq!(wsmsg.frames.len(), 525);
        let data_got = wsmsg.get_message_data().unwrap();
        assert_eq!(data_got.len(), 65535);
        for i in 0..data_got.len() {
            if i == 100 || i == 500 || i == 1000 || i == 5000 || i == 65534 {
                assert_eq!(data_got[i], 0xec);
            } else {
                assert_eq!(data_got[i], 0xce);
            }
        }
    }

    #[test]
    fn _2() {
        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg, WspSett::Mask])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(
            bytes,
            vec![0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58]
        );
    }

    #[test]
    fn _3() {
        // customize frame length
        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        if let Err(err) = wsmsg.set_message_data(b"Hello", vec![WspSett::MaxFrameLen(0)]) {
            assert_eq!(err, WspError::InvalidMaxFrameLen);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn _33() {
        // customize frame length
        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg, WspSett::MaxFrameLen(3)])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(
            bytes,
            vec![
                0x01, 0x03, 0x48, 0x65, 0x6c, //
                0x80, 0x02, 0x6c, 0x6f
            ]
        );
    }

    #[test]
    fn _333() {
        // customize frame length
        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg
            .set_message_data(b"Hello", vec![WspSett::TextMsg, WspSett::MaxFrameLen(1)])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(
            bytes,
            vec![
                0x01, 0x01, b'H', //
                0x00, 0x01, b'e', //
                0x00, 0x01, b'l', //
                0x00, 0x01, b'l', //
                0x80, 0x01, b'o', //
            ]
        );
    }

    #[test]
    fn _4() {
        // long binary

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(256, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_medium();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(&bytes[0..4], vec![0x82, 0x7E, 0x01, 0x00]);
        for b in &bytes[4..] {
            if *b != 0xfa {
                assert!(false);
            }
        }
    }

    #[test]
    fn _44() {
        // long long binary

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(65536, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_large();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        let bytes = wsmsg.to_vec();
        assert_eq!(
            &bytes[0..10],
            vec![0x82, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]
        );
        for b in &bytes[10..] {
            if *b != 0xfa {
                assert!(false);
            }
        }
    }

    #[test]
    fn _5() {
        // always smallest size

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(256, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 3);
    }

    #[test]
    fn _55() {
        // always smallest size

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(256, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        // wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 1);
    }

    #[test]
    fn _555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(256, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg
            .set_message_data(&bin_buf, vec![WspSett::MaxFrameLen(256)])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 1);
    }

    #[test]
    fn _5555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(256, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg
            .set_message_data(&bin_buf, vec![WspSett::MaxFrameLen(128)])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 2);
    }

    #[test]
    fn _55555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(65536, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_large();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 1);
    }

    #[test]
    fn _555555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(65536, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_medium();
        wsmsg.allow_large();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 2); // 65536/65535
    }

    #[test]
    fn _5555555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(65536, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg.allow_large();
        wsmsg
            .set_message_data(&bin_buf, vec![])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 525); // 65536/125
    }

    #[test]
    fn _55555555() {
        // defaut is always smallest size, but with user's will

        let mut bin_buf = Vec::<u8>::new();
        bin_buf.resize(65536, 0);
        bin_buf.fill(0xfa);

        let mut wsmsg = WebSocketMessage::new();
        wsmsg.allow_small();
        wsmsg.allow_medium();
        wsmsg.allow_large();
        wsmsg
            .set_message_data(&bin_buf, vec![WspSett::MaxFrameLen(100)])
            .expect("set payload data");
        dbgg!(&wsmsg);
        assert_eq!(wsmsg.frames.len(), 656); // 65536/100
    }

    use std::io::Write;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn _6() {
        // client parse server's message

        let (rhost, rport) = ("127.0.0.1", "33457");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let msg = vec![0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
                        stream.write_all(&msg).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 1);
                assert_eq!(wsmsg.frames[0].payload, b"Hello");
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _66() {
        let (rhost, rport) = ("127.0.0.1", "33458");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let msg = vec![
                            0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58,
                        ];
                        stream.write_all(&msg).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 1);
                assert_eq!(wsmsg.frames[0].payload, b"Hello");
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _666() {
        let (rhost, rport) = ("127.0.0.1", "33459");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let msg = vec![0x01, 0x03, 0x48, 0x65, 0x6c, 0x80, 0x02, 0x6c, 0x6f];
                        stream.write_all(&msg).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 2);
                assert_eq!(wsmsg.frames[0].payload, b"Hel");
                assert_eq!(wsmsg.frames[1].payload, b"lo");
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _6666() {
        let (rhost, rport) = ("127.0.0.1", "33460");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let mut msg = vec![0x82, 0x7E, 0x01, 0x00];
                        msg.resize(4 + 256, 0xce);
                        stream.write_all(&msg).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 1);
                assert_eq!(wsmsg.frames[0].plen, 256);
                assert_eq!(wsmsg.frames[0].payload, vec![0xce; 256]);
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _66666() {
        let (rhost, rport) = ("127.0.0.1", "33461");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let mut msg =
                            vec![0x82, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00];
                        msg.resize(10 + 65536, 0xce);
                        stream.write_all(&msg).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 1);
                assert_eq!(wsmsg.frames[0].plen, 65536);
                assert_eq!(wsmsg.frames[0].payload, vec![0xce; 65536]);
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _666666() {
        let (rhost, rport) = ("127.0.0.1", "33462");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let mut msg = WebSocketMessage::new();
                        msg.allow_small();
                        msg.set_message_data(&vec![0xab; 65536], vec![])
                            .expect("set payload data");

                        stream.write_all(&msg.to_vec()).unwrap();
                    }
                    Err(e) => {
                        dbgg!(e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client
        match TcpStream::connect(&raddr) {
            Ok(mut stream) => {
                let wsmsg = WebSocketMessage::from_stream(&mut stream).unwrap();
                assert_eq!(wsmsg.frames.len(), 525);
                for i in 0..524 {
                    assert_eq!(wsmsg.frames[i].plen, 125);
                    for b in &wsmsg.frames[i].payload {
                        assert_eq!(*b, 0xab);
                    }
                }
                assert_eq!(wsmsg.frames[524].plen, 36);
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }

    #[test]
    fn _7() {
        // handshake

        let (rhost, rport) = ("127.0.0.1", "19222");
        let raddr = format!("{}:{}", rhost, rport);
        let listener = TcpListener::bind(&raddr).expect("tcp listener");

        // server
        let _join_h = thread::spawn(move || {
            for conn in listener.incoming() {
                match conn {
                    Ok(mut stream) => {
                        let req = HttpRequestParts::from_stream(&stream).unwrap();
                        if req.get_req_uri().unwrap() == b"/ws-resource"
                            && req.get_connection().unwrap() == b"Upgrade"
                            && req.get_upgrade().unwrap() == b"websocket"
                            && req.get_host().unwrap() == b"127.0.0.1:19222"
                            && req.get_ws_key().unwrap() == b"aG93LXRvLWhhbmRzaGFrZQ=="
                            && req.get_ws_ver().unwrap() == b"13"
                        {
                            let mut resp = HttpResponseParts::from_scratch();
                            resp.http1p1()
                                .status(b"101 Switching Protocols")
                                .upgrade("websocket")
                                .connection("Upgrade")
                                .ws_accept("GsCYk86TcY3D9uBDLZuG5FmeV3Y=");

                            resp.send_through(&mut stream).unwrap();
                        } else {
                            dbgg!(String::from_utf8_lossy(req.get_req_uri().unwrap()));
                            dbgg!(String::from_utf8_lossy(req.get_connection().unwrap()));
                            dbgg!(String::from_utf8_lossy(req.get_upgrade().unwrap()));
                            dbgg!(String::from_utf8_lossy(req.get_host().unwrap()));
                            dbgg!(String::from_utf8_lossy(req.get_ws_key().unwrap()));
                            dbgg!(String::from_utf8_lossy(req.get_ws_ver().unwrap()));
                            dbgg!(&req);
                        }
                    }
                    Err(_e) => {
                        dbgg!(_e);
                    }
                }
            }
        });

        sleep(Duration::from_secs(1));

        // client

        match TcpStream::connect(&raddr) {
            Ok(mut s) => {
                WebSocketHandshaker::try_as_client(&mut s, "/ws-resource", &raddr)
                    .expect("handshake");
            }
            Err(e) => {
                dbgg!(e);
            }
        }
    }
}
