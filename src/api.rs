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

#[derive(Debug)]
pub enum GlspCommand {
    Cls,
    SetChar {
        x: i32,
        y: i32,
        glyph: FontCharType,
        fg: RGB,
        bg: RGB,
        console: usize,
    },
    SetConsole {
        id: usize,
    },
    Exit,
}

pub fn cls() {
    CommandQueue::borrow_mut().0.push(GlspCommand::Cls);
}

pub fn set_console(id: usize) {
    CommandQueue::borrow_mut()
        .0
        .push(GlspCommand::SetConsole { id });
}

pub fn set_char_glsp(x: i32, y: i32, glyph: Val, fg: &RGB, bg: &RGB) {
    let (console, glyph) = match glyph {
        Val::Int(g) => (0, g as u16),
        Val::Char(c) => (1, to_cp437(c)),
        _ => {
            panic!("invalid glyph")
        }
    };
    set_char(x, y, glyph, fg, bg, console);
}

pub fn set_char(x: i32, y: i32, glyph: FontCharType, fg: &RGB, bg: &RGB, console: usize) {
    let command = GlspCommand::SetChar {
        x,
        y,
        glyph: glyph as FontCharType,
        fg: *fg,
        bg: *bg,
        console,
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
        .met("init", &|x: i32, y: i32| Point { x, y })
        .prop_get("x", &|p: &Point| p.x)
        .prop_set("x", &|p: &mut Point, x: i32| p.x = x)
        .prop_get("y", &|p: &Point| p.y)
        .prop_set("y", &|p: &mut Point, y: i32| p.y = y)
        .build();
    Ok(())
}
