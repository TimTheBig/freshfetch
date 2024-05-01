use mlua::prelude::*;

use crate::{cli::Inject, errors};

use super::utils::get_system;

#[derive(Clone, Debug)]
pub struct Memory {
    pub max: u64,
    pub used: u64,
}

impl Memory {
    pub fn new() -> Self {
        let system = get_system();
        Memory {
            max: system.total_memory(),
            used: system.used_memory(),
        }
    }
}

impl Inject for Memory {
    fn inject(&self, lua: &mut Lua) {
        let globals = lua.globals();
        match lua.create_table() {
            Ok(t) => {
                match t.set("max", self.max) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match t.set("used", self.used) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match globals.set("memory", t) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
            }
            Err(e) => {
                errors::handle(&format!("{}{}", errors::LUA, e));
                panic!();
            }
        }
    }
}
