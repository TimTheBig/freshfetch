use mlua::prelude::*;

use crate::{
    art::Art,
    cli::{Cli, Inject},
    info::Info,
    misc::Terminal,
};

pub struct Layout {
    pub art: Art,
    pub info: Info,
    pub terminal: Terminal,
}

impl Layout {
    pub fn new(args: &Cli) -> Self {
        let mut info = Info::new();
        Layout {
            art: Art::new(&mut info, &args),
            info,
            terminal: Terminal::new(),
        }
    }
}

impl Inject for Layout {
    fn prep(&mut self) {
        self.info.prep();
        self.art.prep();
        self.terminal.prep();
    }
    fn inject(&self, lua: &mut Lua) {
        self.art.inject(lua);
        self.terminal.inject(lua);
        self.info.inject(lua);
    }
}
