use glsp::{compile, prelude::*};

use crate::{
    api::{self, KeyPressed},
    gamelog::GameLog,
    *,
};

pub struct GlspInterpreter {
    pub runtime: glsp::Runtime,
}

impl GlspInterpreter {
    pub fn new() -> Self {
        // Sandboxed builder forbids access to filesystem
        let builder = RuntimeBuilder::new().sandboxed(false);
        GlspInterpreter {
            runtime: builder.build(),
        }
    }

    pub fn call_init(&self) {
        let cb_init = match glsp::global::<_, Val>("main:init") {
            Ok(Val::GFn(update)) => update,
            Ok(val) => {
                panic!("Invalid init callback:\n{:}", val);
            }
            Err(e) => {
                panic!("Cannot compile glsp code:\n{:}", e);
            }
        };
        let _: Val = match glsp::call(&cb_init, ()) {
            Ok(val) => val,
            Err(glsp_err) => {
                panic!("{:}", &glsp_err);
            }
        };
    }

    /// Calls the (main:update) GameLisp function.
    /// Panics if the function does not exist,
    /// or if the glsp code is syntactically incorrect
    pub fn call_update(&self) {
        let cb_update = match glsp::global("main:update") {
            Ok(Val::GFn(update)) => update,
            Ok(val) => {
                panic!("Invalid update callback:\n{:}", val);
            }
            Err(e) => {
                panic!("Cannot compile glsp code:\n{:}", e);
            }
        };
        let _: Val = match glsp::call(&cb_update, ()) {
            Ok(val) => val,
            Err(glsp_err) => {
                panic!("{:}", &glsp_err);
            }
        };
    }

    /// Initial run to setup the API and global variables/classes
    pub fn setup(&self) {
        self.runtime.run(|| {
            // Release: bundle the glsp code
            #[cfg(feature = "compiler")]
            let res = glsp::load_compiled(compile!["./game/main.glsp"])?;
            // Dev: dynamically load the code
            #[cfg(not(feature = "compiler"))]
            let res = glsp::load("./game/main.glsp")?;

            // internals
            glsp::add_rglobal(api::CommandQueue::new());
            glsp::add_rglobal(KeyPressed::new());

            // log
            glsp::add_rglobal(GameLog::new());
            glsp::bind_rfn("msg", &|s: String| {
                GameLog::borrow_mut().add(s);
            })?;

            // constants & globals
            glsp::bind_global("ctx:key", "")?;
            glsp::bind_global(":width", WIDTH)?;
            glsp::bind_global(":height", HEIGHT)?;
            glsp::bind_global(":bg-color", RGB::named(BG_COLOR))?;

            // log
            glsp::bind_rfn("log:add", &GameLog::add)?;
            glsp::bind_rfn("log:get", &GameLog::get_messages)?;

            // api
            Map::bind_map()?;
            World::bind_world()?;
            api::bind_api()?;
            gui::bind_gui()?;

            // colors
            glsp::bind_rfn("Color", &api::rgb_color)?;

            // rng
            glsp::bind_rfn("RNG", &RandomNumberGenerator::new)?;
            glsp::RClassBuilder::<RandomNumberGenerator>::new()
                .met("roll-dice", &RandomNumberGenerator::roll_dice)
                .met("range", &RandomNumberGenerator::range::<i32>)
                .build();

            // Call the `(defn main:init)` function
            self.call_init();

            glsp::eval(&res, None)?;
            Ok(())
        });
    }

    pub fn tick(&self, ctx: &mut BTerm) {
        self.runtime.run(|| {
            // Quick check:
            // If we fail to retrieve the ctx:key global, it most likely means
            // that there a GLSP syntax error in our code
            match glsp::global::<_, String>("ctx:key") {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                    panic!();
                }
            }
            // Update the ctx:key global
            if let Some(key) = ctx.key {
                // convert VirtualKeyCode to StrKeyCode
                let key: StrKeyCode = FromPrimitive::from_i32(key as i32).unwrap();
                glsp::set_global("ctx:key", key.to_string().to_lowercase())?;
                KeyPressed::borrow_mut().0.replace(key);
            } else {
                glsp::set_global("ctx:key", "")?;
                KeyPressed::borrow_mut().0.take();
            }

            // Call the `(defn main:update)` function
            self.call_update();

            // Execute all deferred commands
            let mut queue = api::CommandQueue::borrow_mut();
            for command in queue.0.iter() {
                match command {
                    GlspCommand::Cls => {
                        ctx.set_active_console(CONSOLE_BG);
                        ctx.cls_bg(RGB::named(GREY15));
                        ctx.set_active_console(CONSOLE_CHARS);
                        ctx.cls_bg(RGB::named(GREY15));
                        ctx.set_active_console(CONSOLE_TEXT);
                        ctx.cls_bg(RGB::named(GREY15));
                    }
                    GlspCommand::SetConsole { id } => ctx.set_active_console(*id),
                    GlspCommand::SetChar {
                        x,
                        y,
                        glyph,
                        fg,
                        bg,
                        console,
                    } => {
                        let console = *console;
                        if ctx.active_console != console {
                            ctx.set_active_console(console);
                        }
                        ctx.set(*x, *y, *fg, *bg, *glyph);
                    }
                    GlspCommand::Exit => ctx.quit(),

                    GlspCommand::SetScanlines(scanlines) => {
                        ctx.post_scanlines = *scanlines;
                    }
                    GlspCommand::SetBurnColor(color) => {
                        ctx.with_post_scanlines(true);
                        ctx.screen_burn_color(*color);
                    }
                    GlspCommand::Print {
                        x,
                        y,
                        output,
                        fg,
                        bg,
                    } => {
                        ctx.print_color(*x, *y, *fg, *bg, output);
                    }
                };
            }
            queue.0.clear();

            glsp::gc();
            Ok(())
        });
    }
}
