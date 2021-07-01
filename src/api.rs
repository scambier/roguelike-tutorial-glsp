use crate::keycodes::StrKeyCode;
use glsp::prelude::*;
use rltk::{VirtualKeyCode, RGB};
use std::{str::FromStr, usize};

fn to_i32(n: Num) -> i32 {
    match n {
        Num::Flo(n) => n as i32,
        Num::Int(n) => n,
    }
}

pub struct CommandQueue(pub Vec<GlspCommand>);
impl CommandQueue {
    pub fn new() -> Self {
        CommandQueue {
            0: Vec::<GlspCommand>::new(),
        }
    }
}
impl RGlobal for CommandQueue {}

pub struct KeyPressed(pub Option<VirtualKeyCode>);
impl KeyPressed {
    pub fn new() -> Self {
        KeyPressed { 0: None }
    }
}
impl RGlobal for KeyPressed {}

pub enum GlspCommand {
    Cls,
    Print {
        x: i32,
        y: i32,
        glyph: u16,
        fg: RGB,
        bg: RGB,
    },
    Exit,
}

pub fn cls() {
    CommandQueue::borrow_mut().0.push(GlspCommand::Cls);
}

pub fn set_char(x: i32, y: i32, char: char, fg: &RGB, bg: &RGB) {
    let glyph = rltk::to_cp437(char);
    let command = GlspCommand::Print {
        x,
        y,
        glyph,
        fg: *fg,
        bg: *bg,
    };

    // RGlobal queue
    CommandQueue::borrow_mut().0.push(command);
}

pub fn key_pressed(k: String) -> bool {
    let key: Result<StrKeyCode, _> = StrKeyCode::from_str(&k);
    let rkey = KeyPressed::borrow().0;
    match (key, rkey) {
        (Ok(_), None) => false,
        (Ok(key), Some(rkey)) => key as u32 == rkey as u32,
        (Err(_), None) => false,
        (Err(_), Some(_)) => false,
    }
}

pub fn rgb_color(r: Num, g: Num, b: Num) -> RGB {
    RGB {
        r: r.into_f32(),
        g: g.into_f32(),
        b: b.into_f32(),
    }
}

pub fn exit() {
    CommandQueue::borrow_mut().0.push(GlspCommand::Exit);
}

/// `(sized-arr "foo" 50)` will create an `(arr)` pre-filled
/// with 50 `"foo"`, indexed from 0 to 49
pub fn sized_arr(val: Val, size: Num) -> Vec<Val> {
    let size = to_i32(size) as usize;
    vec![val; size]
}
