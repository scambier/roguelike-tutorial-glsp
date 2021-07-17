use glsp::prelude::*;

use crate::{*, api::{self, KeyPressed}};

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

            // constants & globals
            glsp::bind_global("ctx:key", "")?;
            glsp::bind_global(":width", WIDTH)?;
            glsp::bind_global(":height", HEIGHT)?;

            // api
            api::bind_api()?;
            Map::bind_map()?;
            World::bind_world()?;

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
}
