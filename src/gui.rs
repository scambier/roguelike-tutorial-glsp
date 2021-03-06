use bracket_lib::prelude::*;
use glsp::{GResult, RGlobal, Val};

use crate::{
    api::{set_char, set_console, CommandQueue, GlspCommand},
    utils::ss_idx,
    BG_COLOR, CONSOLE_BG, CONSOLE_UI,
};

pub fn bind_gui() -> GResult<()> {
    glsp::bind_rfn("draw-box", &draw_box)?;
    glsp::bind_rfn("draw-h-bar", &progress_bar_h)?;
    glsp::bind_rfn("print", &print)?;
    Ok(())
}

fn draw_box(x: i32, y: i32, w: i32, h: i32, fg: Option<&RGB>, bg: Option<&RGB>) {
    let top_left = ss_idx(96, 352);
    let top_right = ss_idx(104, 352);
    let bottom_left = ss_idx(96, 360);
    let bottom_right = ss_idx(104, 360);
    let top = ss_idx(56, 352);
    let bottom = ss_idx(56, 368);
    let left = ss_idx(48, 360);
    let right = ss_idx(64, 360);
    let x2 = x + w - 1;
    let y2 = y + h - 1;
    let fg = &match fg {
        Some(fg) => *fg,
        None => RGB::named(WHITE),
    };
    let bg = &match bg {
        Some(bg) => *bg,
        None => RGB::named(BG_COLOR),
    };

    // corners
    for corner in [
        (x, y, top_left),
        (x, y2, bottom_left),
        (x2, y, top_right),
        (x2, y2, bottom_right),
    ] {
        set_char(corner.0, corner.1, corner.2, fg, bg, 0);
    }
    // top & bottom
    for i in (x + 1)..x2 {
        set_char(i, y, top, fg, bg, 0);
        set_char(i, y2, bottom, fg, bg, 0);
    }
    // left & right
    for j in (y + 1)..y2 {
        set_char(x, j, left, fg, bg, 0);
        set_char(x2, j, right, fg, bg, 0);
    }
}

fn print(x: i32, y: i32, output: Val, fg: Option<&RGB>, bg: Option<&RGB>) {
    let fg = &match fg {
        Some(fg) => *fg,
        None => RGB::named(WHITE),
    };
    let bg = &match bg {
        Some(bg) => *bg,
        None => RGB::named(BG_COLOR),
    };
    let command = GlspCommand::Print {
        x,
        y,
        output: output.to_string(),
        fg: *fg,
        bg: *bg,
    };
    set_console(CONSOLE_UI);
    CommandQueue::borrow_mut().0.push(command);
}

fn progress_bar_h(x: i32, y: i32, w: i32, completed: i32, max: i32, fg: &RGB, bg: &RGB) {
    let filled = (w as f32 * (completed as f32 / max as f32)) as i32;
    for i in x..(x + filled) {
        set_char(i, y, 178, fg, bg, CONSOLE_BG);
    }
    for i in (x + filled)..x + w {
        set_char(
            i,
            y,
            176,
            &fg.lerp(RGB::named(BG_COLOR), 0.5),
            bg,
            CONSOLE_BG,
        );
    }
}
