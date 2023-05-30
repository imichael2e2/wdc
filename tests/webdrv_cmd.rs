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

mod webdrv_cmds {

    use wdc::WdcError::BadDrvCmd;

    use wdc::wdcmd::session::W3cCapaSetter;

    use std::fs::OpenOptions;
    use std::io::Read;
    use std::thread::sleep;
    use std::time::Duration;

    #[cfg(feature = "firefox")]
    mod gecko {
        use super::*;
        use wdc::wdcmd::session::{FirefoxCapa, FirefoxCapaSetter};
        use wdc::GeckoDriver;

        type RendKind = GeckoDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 4444;
        const PBODY_FILENAME_PREFIX: &str = "gecko";

        mod w3c_compliance {
            use super::*;

            #[test]
            fn navi_to1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                let cur_url = wdc.get_url().expect("get url");
                assert_eq!(cur_url, b"about:rights");
            }

            #[test]
            fn find_elem1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                let eleid = wdc.find_elem_css("#your-rights").expect("find elem");
                let eleid = String::from_utf8_lossy(&eleid);
                assert!(is_uuid(&eleid));
            }

            #[test]
            fn find_elem2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                let eleids = wdc.find_elems_css("#your-rights").expect("find elem");
                assert!(eleids.len() > 0);
                for id in eleids {
                    assert!(is_uuid(&id));
                }
            }

            #[test]
            fn screenshot1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                wdc.screenshot(&pbody_file(PBODY_FILENAME_PREFIX, "sshot"))
                    .expect("screenshot failed");
                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "sshot"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn screenshot_elem1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                let eleid = wdc.find_elem_css("#your-rights").expect("find elem");
                let eleid = String::from_utf8_lossy(&eleid);
                wdc.screenshot_elem(&eleid, &pbody_file(PBODY_FILENAME_PREFIX, "sshot-elem"))
                    .expect("screenshot failed");
                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "sshot-elem"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn print_page1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("about:rights").expect("navi to");
                wdc.print_page(&pbody_file(PBODY_FILENAME_PREFIX, "ppdf"))
                    .expect("print page");
                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "ppdf"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn page_src1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                wdc.navi_to("about:rights").expect("navi to");
                wdc.page_src(Some(&pbody_file(PBODY_FILENAME_PREFIX, "pagesrc")))
                    .expect("page src");

                // check file data
                let data_got =
                    std::fs::read_to_string(&pbody_file(PBODY_FILENAME_PREFIX, "pagesrc"))
                        .expect("pbody file");
                assert!(data_got.contains("<html"));
                assert!(data_got.contains("</html>"));
            }

            #[test]
            fn exec_sync1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc.exec_sync("return 0xcafe", vec![]).expect("exec_sync");
                assert_eq!(eval_ret, b"51966");

                let eval_ret = wdc.exec_sync("return '0xcafe'", vec![]).expect("exec_sync");
                assert_eq!(eval_ret, br#""0xcafe""#);
            }

            #[test]
            fn exec_sync2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc.exec_sync("throw 123456789", vec![]);
                assert_eq!(eval_ret.is_ok(), false);

                if let Err(msg) = eval_ret {
                    assert_eq!(
                        msg,
                        BadDrvCmd("javascript error".to_string(), "123456789".to_string())
                    );
                } else {
                    assert!(false);
                }
            }

            #[test]
            fn exec_sync3() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc
                    .exec_sync("return arguments[0] + arguments[1];", vec!["3", "4"])
                    .expect("exec_sync");
                assert_eq!(eval_ret, b"7");

                let eval_ret = wdc
                    .exec_sync(
                        "return arguments[0] + arguments[1];",
                        vec!["\"3\"", "\"4\""],
                    )
                    .expect("exec_sync");
                assert_eq!(eval_ret, b"\"34\"");
            }

            #[test]
            fn exec_async1() {
                let mut capa = FirefoxCapa::default();
                capa.set_timeouts_script(2000);

                let wdc =
                    wdc::init_singl::<RendKind>(REND_HOST, REND_PORT, &capa, 10).expect("init wdc");

                let eval_ret = wdc.exec_async("console.log(123)", vec![]);

                match eval_ret {
                    Ok(_) => assert!(false),
                    Err(e) => match e {
                        BadDrvCmd(_, _) => {
                            assert!(true);
                        }
                        _e => {
                            dbg!(_e);
                            assert!(false);
                        }
                    },
                }
                // the following is true on unix, but not win
                // Err(e) => assert_eq!(
                //     e,
                //     BadDrvCmd(
                //         "script timeout".to_string(),
                //         "Timed out after 2000 ms".to_string()
                //     )
                // ),
            }

            #[test]
            fn exec_async2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc
                    .exec_async(
                        "var cb = arguments[arguments.length-1]; cb(0xcafe);",
                        vec![],
                    )
                    .expect("exec_sync");

                assert_eq!(eval_ret, b"51966");
            }
        }

        mod non_w3c {
            use super::*;

            #[test]
            fn _1() {
                let mut capa = FirefoxCapa::default();
                // capa.set_timeouts_page_load(2000);
                capa.set_proxy_type("manual");
                capa.set_socks_version(5);
                capa.set_socks_proxy("127.0.0.1:1080");
                capa.add_prefs("network.proxy.socks_remote_dns", "true"); // non-std

                let wdc = wdc::init_singl_ff(REND_HOST, REND_PORT, &capa, 10).expect("init wdc");

                wdc.navi_to("about:config").unwrap();

                let eleid = wdc.find_elem_css("#warningButton").expect("find elem");
                let eleid = String::from_utf8_lossy(&eleid);

                wdc.elem_send_keys(&eleid, r"\uE007").expect("send keys");

                let eleid = wdc.find_elem_css("#about-config-search").unwrap();
                let eleid = String::from_utf8_lossy(&eleid);

                wdc.elem_send_keys(&eleid, "network.proxy.")
                    .expect("send keys");

                // GUI event is slow
                sleep(Duration::from_millis(100));

                let table_content = wdc
                    .exec_sync("return document.getElementById('prefs').innerText", vec![])
                    .expect("exec_sync");

                dbg!(&table_content);

                // innerText tricks
                assert!(String::from_utf8_lossy(&table_content)
                    .contains(r"network.proxy.socks\t127.0.0.1"));
                assert!(String::from_utf8_lossy(&table_content)
                    .contains(r"network.proxy.socks_port\t1080"));
                assert!(String::from_utf8_lossy(&table_content)
                    .contains(r"network.proxy.socks_remote_dns\ttrue"));
            }
        }
    }

    #[cfg(feature = "chromium")]
    mod chrom {
        use super::*;
        use wdc::wdcmd::session::ChromiumCapa;
        use wdc::ChromeDriver;

        use wdc::wdcmd::actions::ActionGroup;

        type RendKind = ChromeDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 9515;
        const PBODY_FILENAME_PREFIX: &str = "chrom";

        mod w3c_compliance {
            use super::*;

            #[test]
            fn navi_to1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");
                let cur_url = wdc.get_url().expect("get url");
                assert_eq!(cur_url, b"chrome://version/");
            }

            #[test]
            fn find_elem1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");
                let eleid = wdc.find_elem_css("#outer").expect("find elem");
                let eleid = String::from_utf8_lossy(&eleid);
                assert!(eleid.len() >= 32);
                dbg!(&eleid);
                // assert!(is_uuid(&eleid)); // chrome@fedora/macos fail
            }

            #[test]
            fn find_elem2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");
                let eleids = wdc.find_elems_css("#outer").expect("find elem");
                assert!(eleids.len() > 0);
                for id in eleids {
                    dbg!(&id);
                    assert!(id.len() >= 32);
                    // assert!(is_uuid(&id)); // chrome@fedora/macos fail
                }
            }

            #[test]
            fn screenshot1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");
                wdc.screenshot(&pbody_file(PBODY_FILENAME_PREFIX, "sshot"))
                    .expect("screenshot");
                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "sshot"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn screenshot_elem1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");

                let eleid = wdc.find_elem_css("#outer").expect("find elem");
                let eleid = String::from_utf8_lossy(&eleid);
                wdc.screenshot_elem(&eleid, &pbody_file(PBODY_FILENAME_PREFIX, "sshot-elem"))
                    .expect("screenshot elem");

                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "sshot-elem"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn print_page1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.navi_to("chrome://version").expect("navi to");
                wdc.print_page(&pbody_file(PBODY_FILENAME_PREFIX, "ppdf"))
                    .expect("print page");
                // check b64 data
                let mut buf = [0u8; 4096];
                let mut b64png = OpenOptions::new()
                    .read(true)
                    .open(&pbody_file(PBODY_FILENAME_PREFIX, "ppdf"))
                    .expect("pbody file");
                loop {
                    let nread = b64png.read(&mut buf).unwrap();
                    if nread == 0 {
                        break;
                    }
                    for ele in &buf[0..nread] {
                        if *ele == b'}' || *ele == b'}' || *ele == b'"' {
                            assert!(false, "invalid base64 data");
                        }
                    }
                }
            }

            #[test]
            fn page_src1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                wdc.navi_to("chrome://version").expect("navi to");
                wdc.page_src(Some(&pbody_file(PBODY_FILENAME_PREFIX, "pagesrc")))
                    .expect("page src");

                // sleep(Duration::from_secs(100));

                // check file data
                let data_got =
                    std::fs::read_to_string(&pbody_file(PBODY_FILENAME_PREFIX, "pagesrc"))
                        .expect("pbody file");

                // FIXTHEM: the resulting html is corrupted with octal
                assert!(!data_got.contains("<html"));
                assert!(data_got.contains("html"));
                assert!(data_got.contains("body"));
            }

            #[test]
            fn exec_sync1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc.exec_sync("return 0xcafe", vec![]).expect("exec_sync");
                assert_eq!(eval_ret, b"51966");

                let eval_ret = wdc.exec_sync("return '0xcafe'", vec![]).expect("exec_sync");
                assert_eq!(eval_ret, br#""0xcafe""#);
            }

            #[test]
            fn exec_sync2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc.exec_sync("throw 123456789", vec![]);
                assert_eq!(eval_ret.is_ok(), false);

                if let Err(eobj) = eval_ret {
                    if let BadDrvCmd(err, msg) = eobj {
                        assert_eq!(err, "javascript error");
                        assert_eq!(msg.contains("javascript error"), true);
                    } else {
                        assert!(false);
                    }
                } else {
                    assert!(false);
                }
            }

            #[test]
            fn exec_sync3() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc
                    .exec_sync("return arguments[0] + arguments[1];", vec!["3", "4"])
                    .expect("exec_sync");
                assert_eq!(eval_ret, b"7");

                let eval_ret = wdc
                    .exec_sync(
                        "return arguments[0] + arguments[1];",
                        vec!["\"3\"", "\"4\""],
                    )
                    .expect("exec_sync");
                assert_eq!(eval_ret, b"\"34\"");
            }

            #[test]
            fn exec_async1() {
                let mut capa = ChromiumCapa::default();
                capa.set_timeouts_script(2000);

                let wdc =
                    wdc::init_singl::<RendKind>(REND_HOST, REND_PORT, &capa, 10).expect("init wdc");

                let eval_ret = wdc.exec_async("console.log(123)", vec![]);

                match eval_ret {
                    Ok(_) => assert!(false),
                    Err(e) => match e {
                        BadDrvCmd(cmderr, _cmdemsg) => {
                            assert_eq!(cmderr, "script timeout");
                        }
                        _ee => {
                            dbg!(_ee);
                            assert!(false);
                        }
                    },
                }
            }

            #[test]
            fn exec_async2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let eval_ret = wdc
                    .exec_async(
                        "var cb = arguments[arguments.length-1]; cb(0xcafe);",
                        vec![],
                    )
                    .expect("exec_sync");

                assert_eq!(eval_ret, b"51966");
            }

            #[test]
            fn perform_actions1() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let mut actg = ActionGroup::default();
                {
                    let act1 = actg.add_key_act("key-act-id");

                    act1.add_subact().keydown().tab();
                    act1.add_subact().keyup().tab();
                    act1.add_subact().keydown().tab();

                    act1.add_subact().keydown().unicode("X");
                    act1.add_subact().keydown().enter();
                }

                let _eval_ret = wdc.perform_actions(actg).expect("perform actions");
            }

            #[test]
            fn perform_actions2() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let mut actg = ActionGroup::default();
                {
                    let act1 = actg.add_pointer_act("ptr-act-id");

                    act1.add_subact()
                        .ptr_down()
                        .back_button()
                        .width(123)
                        .height(456)
                        .pressure(0.99)
                        .azimuth_angle(180.123);
                }

                let _eval_ret = wdc.perform_actions(actg).expect("perform actions");
            }

            #[test]
            fn perform_actions3() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let mut actg = ActionGroup::default();
                {
                    let act1 = actg.add_wheel_act("whl-act-id");
                    act1.add_subact()
                        .duration(11)
                        .start_at(12, 13)
                        .scroll_amt(14, 15)
                        .origin_viewport();
                }
                let _eval_ret = wdc.perform_actions(actg).expect("perform actions");
            }

            #[test]
            fn perform_actions4() {
                let wdc = wdc::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");

                let mut actg = ActionGroup::default();
                {
                    let act1 = actg.add_pointer_act("back-to-index");
                    act1.add_subact().ptr_down().back_button();
                    let act2 = actg.add_wheel_act("scroll-down-to-somewhere");
                    act2.add_subact().scroll_amt(14, 15);
                    let act3 = actg.add_key_act("focus-ele-and-enter");
                    act3.add_subact().keydown().tab();
                    act3.add_subact().keydown().enter();
                }

                let _eval_ret = wdc.perform_actions(actg).expect("perform actions");
            }
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

    #[allow(unused)]
    fn pbody_file(prefix: &str, req: &str) -> String {
        format!("wdctmp/{}-{}.pbody", prefix, req)
    }
}
