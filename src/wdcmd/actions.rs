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

use std::borrow::Cow;

// action group

///
/// A group contains all user-requested actions.
#[derive(Debug, Default)]
pub struct ActionGroup<'ag> {
    actions: Vec<ActionKind<'ag>>,
}

impl<'ag> ActionGroup<'ag> {
    pub fn add_act<'o>(&mut self, act: ActionKind<'o>)
    where
        'o: 'ag,
    {
        self.actions.push(act);
    }

    pub fn add_key_act<'me, 'i>(&'me mut self, id: &'i str) -> &'me mut AnyAction<'ag, KeySubAction>
    where
        'i: 'ag,
        'ag: 'me,
    {
        let mut act = AnyAction::<KeySubAction>::default();
        act.set_id(id);
        self.actions.push(ActionKind::Key(act));

        if let Some(ActionKind::Key(ref mut v)) = self.actions.last_mut() {
            v
        } else {
            panic!("bug")
        }
    }

    pub fn add_pointer_act<'me, 'i>(
        &'me mut self,
        id: &'i str,
    ) -> &'me mut AnyAction<'ag, PointerSubAction>
    where
        'i: 'ag,
        'ag: 'me,
    {
        let mut act = AnyAction::<PointerSubAction>::default();
        act.set_id(id);
        self.actions.push(ActionKind::Pointer(act));

        if let Some(ActionKind::Pointer(v)) = self.actions.last_mut() {
            v
        } else {
            panic!("bug")
        }
    }

    pub fn add_wheel_act<'me, 'i>(
        &'me mut self,
        id: &'i str,
    ) -> &'me mut AnyAction<'ag, WheelSubAction>
    where
        'i: 'ag,
        'ag: 'me,
    {
        let mut act = AnyAction::<WheelSubAction>::default();
        act.set_id(id);
        self.actions.push(ActionKind::Wheel(act));

        if let Some(ActionKind::Wheel(v)) = self.actions.last_mut() {
            v
        } else {
            panic!("bug")
        }
    }
}

#[derive(Debug)]
pub enum ActionKind<'ag> {
    None,
    Key(AnyAction<'ag, KeySubAction>),
    Pointer(AnyAction<'ag, PointerSubAction>),
    Wheel(AnyAction<'ag, WheelSubAction>),
}

// any action

#[derive(Debug)]
pub struct AnyAction<'ag, S> {
    id: Cow<'ag, str>,
    sub_acts: Vec<S>,
}

impl<S> Default for AnyAction<'_, S> {
    fn default() -> Self {
        Self {
            id: Cow::from(""),
            sub_acts: vec![],
        }
    }
}

impl<'ag, S: Default> AnyAction<'ag, S> {
    pub fn set_id<'i>(&mut self, arg: &'i str)
    where
        'i: 'ag,
    {
        self.id = Cow::from(arg)
    }

    // as long as borrow does not fit the needs, use move instead
    pub fn set_id_take(&mut self, arg: String) {
        self.id = Cow::from(arg)
    }

    pub fn done(&mut self) {}

    pub fn add_subact(&mut self) -> &mut S {
        let subact = S::default();
        self.sub_acts.push(subact);
        if let Some(r) = self.sub_acts.last_mut() {
            r
        } else {
            panic!("bug")
        }
    }
}

// key

#[derive(Debug)]
pub struct KeySubAction {
    subtype: KeySubActType,
    value: KbdValue,
}

impl Default for KeySubAction {
    fn default() -> Self {
        Self {
            subtype: KeySubActType::Pause,
            value: KbdValue::Unicode(""),
        }
    }
}

impl KeySubAction {
    pub fn keydown(&mut self) -> &mut Self {
        self.subtype = KeySubActType::KeyDown;
        self
    }

    pub fn keyup(&mut self) -> &mut Self {
        self.subtype = KeySubActType::KeyUp;
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        self.subtype = KeySubActType::Pause;
        self
    }

    pub fn backspace(&mut self) -> &mut Self {
        self.value = KbdValue::Backspace;
        self
    }

    pub fn tab(&mut self) -> &mut Self {
        self.value = KbdValue::Tab;
        self
    }

    pub fn enter(&mut self) -> &mut Self {
        self.value = KbdValue::Enter;
        self
    }

    pub fn left_ctrl(&mut self) -> &mut Self {
        self.value = KbdValue::LeftControl;
        self
    }

    pub fn right_ctrl(&mut self) -> &mut Self {
        self.value = KbdValue::RightControl;
        self
    }

    pub fn left_shift(&mut self) -> &mut Self {
        self.value = KbdValue::LeftShift;
        self
    }

    pub fn right_shift(&mut self) -> &mut Self {
        self.value = KbdValue::RightShift;
        self
    }

    pub fn left_alt(&mut self) -> &mut Self {
        self.value = KbdValue::LeftAlt;
        self
    }

    pub fn right_alt(&mut self) -> &mut Self {
        self.value = KbdValue::RightAlt;
        self
    }

    pub fn unicode(&mut self, c: &'static str) -> &mut Self {
        if c.len() < 4 {
            self.value = KbdValue::Unicode(c);
        } else {
            self.value = KbdValue::Unicode("?");
        }
        self
    }

    pub fn done(&mut self) -> &Self {
        self
    }
}

#[derive(Debug, PartialEq)]
pub enum KbdValue {
    Backspace,
    Tab,
    Enter,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    Unicode(&'static str),
}

#[derive(Debug, PartialEq)]
pub enum KeySubActType {
    KeyUp,
    KeyDown,
    Pause,
}

// pointer

#[derive(Debug)]
pub enum PointerSubActType {
    PointerUp,
    PointerDown,
    PointerMove,
    PointerCancel,
    Pause,
}

#[derive(Debug)]
pub struct PointerSubAction {
    subtype: PointerSubActType,
    button: u8,
    width: Option<u32>,
    height: Option<u32>,
    pressure: Option<f32>,     // [0,1]
    tan_pressure: Option<f32>, // [-1,1]
    twist: Option<u32>,
    tiltx: Option<u32>,
    tilty: Option<u32>,
    altitude_angle: Option<f32>,
    azimuth_angle: Option<f32>,
}

impl Default for PointerSubAction {
    fn default() -> Self {
        Self {
            subtype: PointerSubActType::Pause,
            button: 0,
            width: None,
            height: None,
            pressure: None,
            tan_pressure: None,
            twist: None,
            tiltx: None,
            tilty: None,
            altitude_angle: None,
            azimuth_angle: None,
        }
    }
}

impl PointerSubAction {
    pub fn ptr_up(&mut self) -> &mut Self {
        self.subtype = PointerSubActType::PointerUp;
        self
    }
    pub fn ptr_down(&mut self) -> &mut Self {
        self.subtype = PointerSubActType::PointerDown;
        self
    }
    pub fn ptr_move(&mut self) -> &mut Self {
        self.subtype = PointerSubActType::PointerMove;
        self
    }
    pub fn ptr_cancel(&mut self) -> &mut Self {
        self.subtype = PointerSubActType::PointerCancel;
        self
    }
    pub fn pause(&mut self) -> &mut Self {
        self.subtype = PointerSubActType::Pause;
        self
    }

    pub fn left_button(&mut self) -> &mut Self {
        self.button = 0;
        self
    }
    pub fn middle_button(&mut self) -> &mut Self {
        self.button = 1;
        self
    }
    pub fn right_button(&mut self) -> &mut Self {
        self.button = 2;
        self
    }
    pub fn back_button(&mut self) -> &mut Self {
        self.button = 3;
        self
    }
    pub fn forward_button(&mut self) -> &mut Self {
        self.button = 4;
        self
    }

    pub fn width(&mut self, arg: u32) -> &mut Self {
        self.width = Some(arg);
        self
    }

    pub fn height(&mut self, arg: u32) -> &mut Self {
        self.height = Some(arg);
        self
    }
    pub fn pressure(&mut self, arg: f32) -> &mut Self {
        self.pressure = Some(arg);
        self
    }
    pub fn azimuth_angle(&mut self, arg: f32) -> &mut Self {
        self.azimuth_angle = Some(arg);
        self
    }

    pub fn done(&mut self) {}
}

// wheel

#[derive(Debug)]
pub struct WheelSubAction {
    subtype: WheelSubActType,
    start_at: (u32, u32),
    scroll_amt: (u32, u32),
    duration: Option<u32>,
    origin: Option<u8>,
}

impl Default for WheelSubAction {
    fn default() -> Self {
        WheelSubAction {
            subtype: WheelSubActType::Scroll,
            start_at: (0, 0),
            scroll_amt: (0, 0),
            duration: None,
            origin: None,
        }
    }
}

impl WheelSubAction {
    pub fn duration(&mut self, t: u32) -> &mut Self {
        self.duration = Some(t);
        self
    }

    pub fn origin_viewport(&mut self) -> &mut Self {
        self.origin = Some(1);
        self
    }

    pub fn origin_pointer(&mut self) -> &mut Self {
        self.origin = Some(2);
        self
    }

    pub fn start_at(&mut self, x: u32, y: u32) -> &mut Self {
        self.start_at = (x, y);
        self
    }

    pub fn scroll_amt(&mut self, x: u32, y: u32) -> &mut Self {
        self.scroll_amt = (x, y);
        self
    }
}

#[derive(Debug)]
pub enum WheelSubActType {
    Scroll,
    Pause,
}

mod ser {
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    const INSIG_SNAME: &str = "-";
    const INSIG_SFLEN: usize = 1;

    use super::*;

    impl Serialize for ActionGroup<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            state.serialize_field("actions", &self.actions)?;

            state.end()
        }
    }

    impl Serialize for ActionKind<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            match self {
                ActionKind::None => {
                    state.serialize_field("type", "none")?;
                    // ...
                }
                ActionKind::Key(act) => {
                    state.serialize_field("type", "key")?;
                    state.serialize_field("id", &act.id)?;
                    state.serialize_field("actions", &act.sub_acts)?;
                }
                ActionKind::Wheel(act) => {
                    state.serialize_field("type", "wheel")?;
                    state.serialize_field("id", &act.id)?;
                    state.serialize_field("actions", &act.sub_acts)?;
                }
                ActionKind::Pointer(act) => {
                    state.serialize_field("type", "pointer")?;
                    state.serialize_field("id", &act.id)?;
                    // ...
                    state.serialize_field("actions", &act.sub_acts)?;
                } // _ => {}
            }

            state.end()
        }
    }

    impl Serialize for KeySubAction {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            match self.subtype {
                KeySubActType::KeyDown => {
                    state.serialize_field("type", "keyDown")?;
                }
                KeySubActType::KeyUp => {
                    state.serialize_field("type", "keyUp")?;
                }
                KeySubActType::Pause => {
                    state.serialize_field("type", "pause")?;
                } // _ => {}
            }

            match self.value {
                KbdValue::Backspace => {
                    state.serialize_field("value", "__U__E003")?;
                }
                KbdValue::Tab => {
                    state.serialize_field("value", "__U__E004")?;
                }
                KbdValue::Enter => {
                    // state.serialize_field("value", "\\uE007")?;
                    //    \----> can be solved by serde's extra feature,
                    state.serialize_field("value", "__U__E007")?;
                }
                KbdValue::LeftShift => {
                    state.serialize_field("value", "__U__E008")?;
                }
                KbdValue::LeftControl => {
                    state.serialize_field("value", "__U__E009")?;
                }

                KbdValue::Unicode(s) => {
                    state.serialize_field("value", s)?;
                }
                _ => {}
            }

            state.end()
        }
    }

    impl Serialize for WheelSubAction {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            match self.subtype {
                WheelSubActType::Scroll => state.serialize_field("type", "scroll")?,
                WheelSubActType::Pause => state.serialize_field("type", "pause")?,
                // _ => {}
            }
            if let Some(dura) = self.duration {
                state.serialize_field("duration", &dura)?;
            }

            if let Some(otype) = &self.origin {
                match otype {
                    1 => state.serialize_field("origin", "viewport")?,
                    2 => state.serialize_field("origin", "pointer")?,
                    _ => {}
                }
            }

            let start_x = self.start_at.0;
            let start_y = self.start_at.1;
            let amt_x = self.scroll_amt.0;
            let amt_y = self.scroll_amt.1;
            state.serialize_field("x", &start_x)?;
            state.serialize_field("y", &start_y)?;
            state.serialize_field("deltaX", &amt_x)?;
            state.serialize_field("deltaY", &amt_y)?;

            state.end()
        }
    }

    impl Serialize for PointerSubAction {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct(INSIG_SNAME, INSIG_SFLEN)?;

            match self.subtype {
                PointerSubActType::PointerUp => state.serialize_field("type", "pointerUp")?,
                PointerSubActType::PointerDown => state.serialize_field("type", "pointerDown")?,
                PointerSubActType::PointerMove => state.serialize_field("type", "pointerMove")?,
                PointerSubActType::PointerCancel => {
                    state.serialize_field("type", "pointerCancel")?
                }
                PointerSubActType::Pause => state.serialize_field("type", "pause")?,
                // _ => {}
            }

            state.serialize_field("button", &self.button)?;

            if let Some(v) = &self.width {
                state.serialize_field("width", v)?;
            }
            if let Some(v) = &self.height {
                state.serialize_field("height", v)?;
            }
            if let Some(v) = &self.pressure {
                state.serialize_field("pressure", v)?;
            }
            if let Some(v) = &self.tan_pressure {
                state.serialize_field("tangentialPressure", v)?;
            }
            if let Some(v) = &self.twist {
                state.serialize_field("twist", v)?;
            }
            if let Some(v) = &self.tiltx {
                state.serialize_field("tiltX", v)?;
            }
            if let Some(v) = &self.tilty {
                state.serialize_field("tiltY", v)?;
            }
            if let Some(v) = &self.altitude_angle {
                state.serialize_field("altitudeAngle", v)?;
            }
            if let Some(v) = &self.azimuth_angle {
                state.serialize_field("azimuthAngle", v)?;
            }

            state.end()
        }
    }
} // ser

#[cfg(test)]
mod utst {
    use super::*;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn _1() {
        let mut actg = ActionGroup::default();
        {
            let act1 = actg.add_key_act("key-act-id");

            act1.add_subact().keydown().tab();
            act1.add_subact().keyup().tab();
            act1.add_subact().keydown().tab();

            act1.add_subact().keydown().unicode("X");
            act1.add_subact().keydown().enter();
        }

        assert_eq!(actg.actions.len(), 1);

        dbgg!(&actg);

        if let Some(ActionKind::Key(r)) = actg.actions.last() {
            assert_eq!(r.id, "key-act-id");
            assert_eq!(r.sub_acts.len(), 5);
            assert_eq!(r.sub_acts[0].subtype, KeySubActType::KeyDown);
            assert_eq!(r.sub_acts[0].value, KbdValue::Tab);
            assert_eq!(r.sub_acts[1].subtype, KeySubActType::KeyUp);
            assert_eq!(r.sub_acts[1].value, KbdValue::Tab);
            assert_eq!(r.sub_acts[2].subtype, KeySubActType::KeyDown);
            assert_eq!(r.sub_acts[2].value, KbdValue::Tab);
            assert_eq!(r.sub_acts[3].subtype, KeySubActType::KeyDown);
            assert_eq!(r.sub_acts[3].value, KbdValue::Unicode("X"));
            assert_eq!(r.sub_acts[4].subtype, KeySubActType::KeyDown);
            assert_eq!(r.sub_acts[4].value, KbdValue::Enter);
        } else {
            assert!(false);
        }

        // ser

        assert_ser_tokens(
            &actg,
            &[
                Token::Struct { name: "-", len: 1 },
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                // seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("key"),
                Token::Str("id"),
                Token::Str("key-act-id"),
                Token::Str("actions"),
                Token::Seq { len: Some(5) },
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("__U__E004"),
                Token::StructEnd,
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyUp"),
                Token::Str("value"),
                Token::Str("__U__E004"),
                Token::StructEnd,
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("__U__E004"),
                Token::StructEnd,
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("X"),
                Token::StructEnd,
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("__U__E007"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        fn into_json_bytes(obj: &ActionGroup) -> Vec<u8> {
            serde_json::to_vec(obj).unwrap()
        }

        let bs = into_json_bytes(&actg);

        fn post_process_into_string(bs: &[u8]) -> String {
            let mut s = String::from_utf8_lossy(bs).to_string();
            s = s.replace("__U__", r"\u");
            s
        }

        assert_eq!(
            post_process_into_string(&bs),
            r#"{"actions":[{"type":"key","id":"key-act-id","actions":[{"type":"keyDown","value":"\uE004"},{"type":"keyUp","value":"\uE004"},{"type":"keyDown","value":"\uE004"},{"type":"keyDown","value":"X"},{"type":"keyDown","value":"\uE007"}]}]}"#
        );
    }

    #[test]
    fn _2() {
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

        dbgg!(&actg);

        // ser

        assert_ser_tokens(
            &actg,
            &[
                Token::Struct { name: "-", len: 1 },
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("pointer"),
                Token::Str("id"),
                Token::Str("ptr-act-id"),
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("pointerDown"),
                Token::Str("button"),
                Token::U8(3),
                Token::Str("width"),
                Token::U32(123),
                Token::Str("height"),
                Token::U32(456),
                Token::Str("pressure"),
                Token::F32(0.99),
                Token::Str("azimuthAngle"),
                Token::F32(180.123),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        let s = serde_json::to_string(&actg).unwrap();

        assert_eq!(
            &s,
            r#"{"actions":[{"type":"pointer","id":"ptr-act-id","actions":[{"type":"pointerDown","button":3,"width":123,"height":456,"pressure":0.99,"azimuthAngle":180.123}]}]}"#
        );
    }

    #[test]
    fn _3() {
        let mut actg = ActionGroup::default();
        {
            let act1 = actg.add_wheel_act("whl-act-id");
            act1.add_subact()
                .duration(11)
                .start_at(12, 13)
                .scroll_amt(14, 15)
                .origin_viewport();
        }

        dbgg!(&actg);

        // ser

        assert_ser_tokens(
            &actg,
            &[
                Token::Struct { name: "-", len: 1 },
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("wheel"),
                Token::Str("id"),
                Token::Str("whl-act-id"),
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("scroll"),
                Token::Str("duration"),
                Token::U32(11),
                Token::Str("origin"),
                Token::Str("viewport"),
                Token::Str("x"),
                Token::U32(12),
                Token::Str("y"),
                Token::U32(13),
                Token::Str("deltaX"),
                Token::U32(14),
                Token::Str("deltaY"),
                Token::U32(15),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        let s = serde_json::to_string(&actg).unwrap();

        // dbgg!(String::from_utf8_lossy(&bs));

        assert_eq!(
            &s,
            r#"{"actions":[{"type":"wheel","id":"whl-act-id","actions":[{"type":"scroll","duration":11,"origin":"viewport","x":12,"y":13,"deltaX":14,"deltaY":15}]}]}"#
        );
    }

    #[test]
    fn _4() {
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

        dbgg!(&actg);

        // ser

        assert_ser_tokens(
            &actg,
            &[
                Token::Struct { name: "-", len: 1 },
                Token::Str("actions"),
                Token::Seq { len: Some(3) },
                // seq ele
                // {"type":"pointer","id":"back-to-index","actions":[{"type":"pointerDown","button"
                // :3}]}
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("pointer"),
                Token::Str("id"),
                Token::Str("back-to-index"),
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("pointerDown"),
                Token::Str("button"),
                Token::U8(3),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                // seq ele
                // {"type"::"wheel","id":"scroll-down-to-somewhere","actions":[{"type":"scroll","x"
                // :0,"y":0,"deltaX":14,"deltaY":15}]}
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("wheel"),
                Token::Str("id"),
                Token::Str("scroll-down-to-somewhere"),
                Token::Str("actions"),
                Token::Seq { len: Some(1) },
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("scroll"),
                Token::Str("x"),
                Token::U32(0),
                Token::Str("y"),
                Token::U32(0),
                Token::Str("deltaX"),
                Token::U32(14),
                Token::Str("deltaY"),
                Token::U32(15),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                // seq ele
                // {"type":"key","id":"focus-ele-and-enter","actions":[{"type":"keyDown","value":"\
                // uE004"},{"type":"keyDown","value":"\uE007"}]}
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("key"),
                Token::Str("id"),
                Token::Str("focus-ele-and-enter"),
                Token::Str("actions"),
                Token::Seq { len: Some(2) },
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("__U__E004"),
                Token::StructEnd,
                // seq ele seq ele
                Token::Struct { name: "-", len: 1 },
                Token::Str("type"),
                Token::Str("keyDown"),
                Token::Str("value"),
                Token::Str("__U__E007"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        let mut s = serde_json::to_string(&actg).unwrap();

        s = s.replace("__U__", r"\u");

        assert_eq!(
            &s,
            r#"{"actions":[{"type":"pointer","id":"back-to-index","actions":[{"type":"pointerDown","button":3}]},{"type":"wheel","id":"scroll-down-to-somewhere","actions":[{"type":"scroll","x":0,"y":0,"deltaX":14,"deltaY":15}]},{"type":"key","id":"focus-ele-and-enter","actions":[{"type":"keyDown","value":"\uE004"},{"type":"keyDown","value":"\uE007"}]}]}"#
        );
    }
}
