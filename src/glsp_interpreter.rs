use glsp::prelude::*;

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
}
