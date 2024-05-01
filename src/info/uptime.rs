use std::path::Path;

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use mlua::prelude::*;

use crate::{cli::Inject, errors};

use super::kernel::Kernel;

pub struct Uptime(pub DateTime<Utc>);

impl Uptime {
    pub fn new(k: &Kernel) -> Self {
        let uptime_seconds;
        match k.name.as_str() {
            "Linux" | "Windows" | "MINIX" => {
                // Since `crate::sysinfo::SystemExt::get_uptime()` gets uptime
                // from /proc/uptime, we should check that it exists and have a
                // fallback.
                if Path::new("/proc/uptime").exists() {
                    uptime_seconds = sysinfo::System::uptime() as i64;
                } else {
                    // `crate::sysinfo::SystemExt::get_boot_time()` doesn't
                    // appear to rely on /proc/uptime, so we should be able to
                    // use it here.
                    let boot_time = sysinfo::System::boot_time() as i64;
                    let now_time = Utc::now().timestamp();
                    uptime_seconds = boot_time - now_time;
                }
            }
            // Unknown OSes should have already exit(1)'d by now, this is just
            // to satisfy the compiler.
            _ => {
                uptime_seconds = 0;
            }
        }
        Uptime(Utc.timestamp_opt(uptime_seconds, 0).unwrap())
    }
}

impl Inject for Uptime {
    fn inject(&self, lua: &mut Lua) {
        let globals = lua.globals();

        match lua.create_table() {
            Ok(t) => {
                match t.set("days", self.0.ordinal0()) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
                match t.set("hours", self.0.hour()) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
                match t.set("minutes", self.0.minute()) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
                match t.set("seconds", self.0.second()) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
                match globals.set("uptime", t) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
            }
            Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
        }
    }
}
