use mlua::prelude::*;

use crate::{cli::Inject, errors};

pub struct Terminal {
    pub width: i32,
    pub height: i32,
}

impl Terminal {
    pub fn new() -> Self {
        let (w, h) = term_size::dimensions().expect("Failed to get terminal dimensions.");
        Terminal {
            width: w as i32,
            height: h as i32,
        }
    }
}

impl Inject for Terminal {
    fn inject(&self, lua: &mut Lua) {
        let globals = lua.globals();
        match lua.create_table() {
            Ok(t) => {
                match t.set("width", self.width) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err = e)),
                }
                match t.set("height", self.height) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err = e)),
                }
                match globals.set("terminal", t) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err = e)),
                }
            }
            Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err = e)),
        }
    }
}
