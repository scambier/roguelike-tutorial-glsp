use crate::keycodes::StrKeyCode;
use bracket_lib::prelude::*;
use glsp::prelude::*;
use std::str::FromStr;

pub struct CommandQueue(pub Vec<GlspCommand>);
impl CommandQueue {
    pub fn new() -> Self {
        CommandQueue {
            0: Vec::<GlspCommand>::new(),
        }
    }
}
impl RGlobal for CommandQueue {}

pub struct KeyPressed(pub Option<StrKeyCode>);
impl KeyPressed {
    pub fn new() -> Self {
        KeyPressed { 0: None }
    }
}
impl RGlobal for KeyPressed {}

pub enum GlspCommand {
    Cls,
    SetChar {
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
    let glyph = to_cp437(char);
    let command = GlspCommand::SetChar {
        x,
        y,
        glyph,
        fg: *fg,
        bg: *bg,
    };

    // RGlobal queue
    CommandQueue::borrow_mut().0.push(command);
}

pub fn is_key_pressed(k: String) -> bool {
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

pub fn bind_geometry() -> GResult<()> {
    // Rect
    glsp::bind_rfn("Rect", &Rect::with_size::<i32>)?;
    glsp::RClassBuilder::<Rect>::new()
        .met("intersect?", &Rect::intersect)
        .met("center", &|rect: &Rect| {
            let center = rect.center();
            vec![center.x, center.y]
        })
        .build();

    // Point
    glsp::bind_rfn("Point", &Point::new::<i32>)?;
    glsp::RClassBuilder::<Point>::new()
        .prop_get("x", &|r: &Point| r.x)
        .prop_set("x", &|r: &mut Point, x: i32| r.x = x)
        .prop_get("y", &|r: &Point| r.y)
        .prop_set("y", &|r: &mut Point, y: i32| r.y = y)
        .build();
    Ok(())
}
