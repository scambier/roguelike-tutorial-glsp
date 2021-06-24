use std::sync::Mutex;

use glsp::Val;
use lazy_static::lazy_static;
use rltk::{GameState, Rltk};
mod glsp_interpreter;
use glsp_interpreter::*;

lazy_static! {
    pub static ref QUEUE: Mutex<Vec<GlspCommand>> = Mutex::new(vec![]);
}

struct State {
    interpreter: GlspInterpreter,
    code: String,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.interpreter.runtime.run(|| {
            let update = match glsp::global::<_, Val>("run") {
                Ok(Val::GFn(update)) => update,
                Ok(val) => {
                    let msg = format!("Invalid update value {}", val);
                    panic!("{:}", msg);
                }
                Err(e) => {
                    let msg = "run is not defined";
                    panic!("{:}", e);
                }
            };
            let _: Val = match glsp::call(&update, ()) {
                Ok(val) => val,
                Err(glsp_err) => {
                    panic!("{:}", &glsp_err);
                }
            };

            glsp::gc();
            Ok(())
        });

        for command in QUEUE.lock().unwrap().iter() {
            match command {
                GlspCommand::Cls => ctx.cls(),
                GlspCommand::Print { x, y, s } => ctx.print(*x, *y, s),
            }
        }
        QUEUE.lock().unwrap().clear();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let code = String::from(
        r#"
        (let x 0)
        (defn run ()
            (cls)
            (print x 1 "Hello RLTK World")
            (inc! x)
        )
        "#,
    );

    let interpreter = GlspInterpreter::new();
    interpreter.runtime.run(|| {
        glsp::bind_rfn("cls", &cls)?;
        glsp::bind_rfn("print", &print)?;

        // parse the code
        let vals = glsp::parse_all(&code, Some("game"))?;

        // initial evaluation
        glsp::eval_multi(&vals, None)?;
        Ok(())
    });
    let gs = State { interpreter, code };
    rltk::main_loop(context, gs)
}
