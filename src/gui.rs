use bracket_lib::prelude::*;
use glsp::{GResult, RGlobal};

use crate::{
    api::{set_char, set_console, CommandQueue, GlspCommand},
    utils::ss_idx,
    BG_COLOR, CONSOLE_BG, CONSOLE_TEXT,
};

pub fn bind_gui() -> GResult<()> {
    glsp::bind_rfn("draw-box", &draw_box)?;
    glsp::bind_rfn("draw-h-bar", &progress_bar_h)?;
    glsp::bind_rfn("print", &print)?;
    Ok(())
}

fn draw_box(x: i32, y: i32, w: i32, h: i32, fg: Option<&RGB>, bg: Option<&RGB>) {
    let top_left = ss_idx(96, 224);
    let top_right = ss_idx(104, 224);
    let bottom_left = ss_idx(96, 232);
    let bottom_right = ss_idx(104, 232);
    let top = ss_idx(56, 224);
    let bottom = ss_idx(56, 240);
    let left = ss_idx(48, 232);
    let right = ss_idx(64, 232);
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

fn print(x: i32, y: i32, output: String, fg: Option<&RGB>, bg: Option<&RGB>) {
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
        output,
        fg: *fg,
        bg: *bg,
    };
    set_console(CONSOLE_TEXT);
    CommandQueue::borrow_mut().0.push(command);
}

fn progress_bar_h(x: i32, y: i32, w: i32, completed: i32, max: i32, fg: &RGB, bg: &RGB) {
    let filled = (w as f32 * (completed as f32 / max as f32)) as i32;
    for i in x..(x + filled) {
        set_char(i, y, ss_idx(16, 160), fg, bg, CONSOLE_BG);
    }
    for i in (x + filled)..x + w {
        set_char(
            i,
            y,
            ss_idx(0, 208),
            &fg.lerp(RGB::named(BG_COLOR), 0.5),
            bg,
            CONSOLE_BG,
        );
    }
}
