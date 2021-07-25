use crate::{keycodes::StrKeyCode, utils::ss_idx, BG_COLOR,};
use bracket_lib::prelude::*;
use glsp::prelude::*;

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
    SetBgColor {
        x: i32,
        y: i32,
        bg: RGB,
    },
    SetConsole {
        id: usize,
    },
    SetScanlines(bool),
    SetBurnColor(RGB),
    Print {
        x: i32,
        y: i32,
        output: String,
        fg: RGB,
        bg: RGB,
    },
    Exit,
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

pub struct KeyPressed(pub Option<StrKeyCode>);

impl KeyPressed {
    pub fn new() -> Self {
        KeyPressed { 0: None }
    }
}

impl RGlobal for KeyPressed {}

pub fn bind_utils() -> GResult<()> {
    glsp::bind_rfn("cls", &cls)?;
    glsp::bind_rfn("set", &set_char_glsp)?;
    glsp::bind_rfn("set_bg", &set_bg_glsp)?;
    glsp::bind_rfn("exit", &exit)?;
    glsp::bind_rfn("ss-idx", &ss_idx)?;

    // Screen
    glsp::bind_rfn("ctx:scanlines!", &set_scanlines)?;
    glsp::bind_rfn("ctx:burn!", &set_burn_color)?;
    glsp::bind_rfn("ctx:console!", &set_console)?;

    Ok(())
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
        .prop_get("x", &|p: &Point| p.x)
        .prop_set("x", &|p: &mut Point, x: i32| p.x = x)
        .prop_get("y", &|p: &Point| p.y)
        .prop_set("y", &|p: &mut Point, y: i32| p.y = y)
        .build();
    glsp::bind_rfn("dist2d", &dist2d)?;
    Ok(())
}

/// Clears all consoles
pub fn cls() {
    CommandQueue::borrow_mut().0.push(GlspCommand::Cls);
}

/// Sets console for the next draw call
pub fn set_console(id: usize) {
    CommandQueue::borrow_mut()
        .0
        .push(GlspCommand::SetConsole { id });
}

/// Draws a char on screen (glsp fn)
pub fn set_char_glsp(
    x: i32,
    y: i32,
    glyph: Val,
    fg: &RGB,
    bg: Option<&RGB>,
    console: Option<usize>,
) {
    let bg = &match bg {
        Some(bg) => *bg,
        None => RGB::named(BG_COLOR),
    };
    let (glyph, console) = match (glyph, console) {
        (Val::Int(g), None) => (g as u16, 0),
        (Val::Int(g), Some(c)) => (g as u16, c),
        (Val::Char(g), None) => (to_cp437(g), 1),
        (Val::Char(g), Some(c)) => (to_cp437(g), c),
        _ => {
            panic!("invalid glyph")
        }
    };
    set_char(x, y, glyph, fg, bg, console);
}

pub fn set_bg_glsp(x: i32, y: i32, bg: &RGB) {
    CommandQueue::borrow_mut()
        .0
        .push(GlspCommand::SetBgColor { x, y, bg: *bg });
}

/// Draws a char on screen (general fn)
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

/// Returns if a specific key is pressed for the current frame
// pub fn is_key_pressed(k: String) -> bool {
//     let key: Result<StrKeyCode, _> = StrKeyCode::from_str(&k);
//     let rkey = KeyPressed::borrow().0;
//     match (key, rkey) {
//         (Ok(_), None) => false,
//         (Ok(key), Some(rkey)) => key as u32 == rkey as u32,
//         (Err(_), None) => false,
//         (Err(_), Some(_)) => false,
//     }
// }

/// Returns a color from 0-1 RBG values
pub fn rgb_color(r: Num, g: Num, b: Num) -> RGB {
    RGB {
        r: r.into_f32(),
        g: g.into_f32(),
        b: b.into_f32(),
    }
}

fn set_scanlines(set: bool) {
    CommandQueue::borrow_mut()
        .0
        .push(GlspCommand::SetScanlines(set));
}

fn set_burn_color(color: &RGB) {
    CommandQueue::borrow_mut()
        .0
        .push(GlspCommand::SetBurnColor(*color));
}

/// Exits the game
pub fn exit() {
    CommandQueue::borrow_mut().0.push(GlspCommand::Exit);
}

/// Converts a GLSP "Pos" component to a Rust <Point>
fn pos_to_point(pos: Val) -> GResult<Point> {
    match pos {
        Val::Obj(pos) => match (pos.get("x"), pos.get("y")) {
            (Ok(x), Ok(y)) => Ok(Point { x, y }),
            _ => bail!("This object does not have x and y fields"),
        },
        val => bail!("Expected an Obj, received a {}", val),
    }
}

/// Gets the distance between 2 "Pos" components
fn dist2d(start: Val, end: Val) -> GResult<f32> {
    match (pos_to_point(start), pos_to_point(end)) {
        (Ok(start), Ok(end)) => Ok(DistanceAlg::Pythagoras.distance2d(start, end)),
        (Ok(_), Err(e)) => bail!(e),
        (Err(e), Ok(_)) => bail!(e),
        (Err(e), Err(_)) => bail!(e),
    }
}
