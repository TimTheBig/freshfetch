use clap::Parser;
use cli::{Cli, Inject};
use mlua::prelude::*;

use assets::defaults::LAYOUT;
use assets::{ANSI, PRINT};
use layout::Layout;

use std::env::var;
use std::fs::read_to_string;
use std::path::Path;

mod art;
mod assets;
mod cli;
mod errors;
mod info;
mod layout;
mod misc;
mod utils;

fn main() {
    let args = Cli::parse();
    let mut ctx = Lua::new();
    match ctx.load(PRINT).exec() {
        Ok(_) => (),
        Err(e) => {
            errors::handle(&format!("{}{}", errors::LUA, e));
            panic!();
        }
    }
    match ctx.load(ANSI).exec() {
        Ok(_) => (),
        Err(e) => {
            errors::handle(&format!("{}{}", errors::LUA, e));
            panic!();
        }
    }

    let mut layout = Layout::new(&args);
    layout.prep();
    layout.inject(&mut ctx);

    let layout_file = Path::new("/home/")
        .join(var("USER").unwrap_or(String::new()))
        .join(".config/freshfetch/layout.lua");

    if layout_file.exists() {
        match read_to_string(&layout_file) {
            Ok(v) => {
                match ctx.load(&v).exec() {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match ctx.globals().get::<&str, String>("__freshfetch__") {
                    Ok(v) => print!("{}", v),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
            }
            Err(e) => {
                errors::handle(&format!(
                    "{}{file}{}{err}",
                    errors::io::READ.0,
                    errors::io::READ.1,
                    file = layout_file.to_string_lossy(),
                    err = e
                ));
                panic!();
            }
        }
    } else {
        match ctx.load(LAYOUT).exec() {
            Ok(_) => (),
            Err(e) => {
                errors::handle(&format!("{}{}", errors::LUA, e));
                panic!();
            }
        }
        match ctx.globals().get::<&str, String>("__freshfetch__") {
            Ok(v) => print!("{}", v),
            Err(e) => {
                errors::handle(&format!("{}{}", errors::LUA, e));
                panic!();
            }
        }
    }
}
