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

#[cfg(feature = "bidi")]
#[allow(dead_code)]
mod bidi_cmds {

    #[cfg(feature = "firefox")]
    mod gecko {
        use wdc::GeckoDriver;

        type RendKind = GeckoDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 4444;
        const PBODY_FILENAME_PREFIX: &str = "gecko";

        mod w3c_compliance {
            use super::*;

            #[test]
            fn navi_to() {
                let mut wdc =
                    wdc::bidi::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.gen_ctx(1).expect("gen ctx");
                assert_eq!(wdc.ctxlist().len(), 1);
                // wdc.navi(wdc.ctxlist()[0], "http://w3.org/standards")
                wdc.navi(wdc.ctxlist()[0], "about:rights").expect("navi");
                let res = wdc.ctx_tree().expect("ctx tree");
                dbg!(&res);
            }
        }
    }

    #[cfg(feature = "chromium")]
    mod chrom {
        use wdc::ChromeDriver;

        type RendKind = ChromeDriver;
        const REND_HOST: &str = "127.0.0.1";
        const REND_PORT: u16 = 9515;
        const PBODY_FILENAME_PREFIX: &str = "chrom";

        mod w3c_compliance {
            use super::*;

            #[test]
            fn navi_to() {
                let mut wdc =
                    wdc::bidi::init::<RendKind>(REND_HOST, REND_PORT, 10).expect("init wdc");
                wdc.gen_ctx(1).expect("gen ctx");
                assert_eq!(wdc.ctxlist().len(), 1);
                // wdc.navi(wdc.ctxlist()[0], "http://w3.org/standards")
                wdc.navi(wdc.ctxlist()[0], "chrome://version")
                    .expect("navi");
                wdc.ctx_tree().expect("ctx tree");
                // sleep(Duration::from_secs(100));
            }
        }
    }

    // Auxiliary Functions //

    fn is_uuid(s: &str) -> bool {
        let re = regex::Regex::new(
            r"^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}$",
        )
        .unwrap();

        re.is_match(s)
    }

    fn is_uuid_nodash(s: &str) -> bool {
        let re = regex::Regex::new(
            r"^[0-9A-Fa-f]{8}[0-9A-Fa-f]{4}[0-9A-Fa-f]{4}[0-9A-Fa-f]{4}[0-9A-Fa-f]{12}$",
        )
        .unwrap();

        re.is_match(s)
    }

    fn pbody_file(prefix: &str, req: &str) -> String {
        format!("wdctmp/{}-{}.pbody", prefix, req)
    }
}
