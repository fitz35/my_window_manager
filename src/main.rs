use std::collections::HashMap;


use bar::status_bar;
use keybindings::raw_key_bindings;

use penrose::core::bindings::parse_keybindings_with_xmodmap;
use penrose::core::{Config, WindowManager};
use penrose::extensions::hooks::manage::FloatingCentered;
use penrose::extensions::hooks::{add_named_scratchpads, NamedScratchPad};
use penrose::x::query::ClassName;
use penrose::Result;
use penrose::x11rb::RustConn;

use simplelog::{ LevelFilter, SimpleLogger };


mod styles;
mod hooks;
mod keybindings;
mod bar;



fn main() -> Result<()>{
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let conn = RustConn::new()?;
    let (nsp, _) = NamedScratchPad::new(
        "terminal",
        "st -c StScratchpad",
        ClassName("StScratchpad"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(Config::default(), key_bindings, HashMap::new(), conn)?;
    let wm = add_named_scratchpads(status_bar().add_to(wm), vec![nsp]);

    wm.run()
}
