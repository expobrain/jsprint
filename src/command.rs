use std::boxed::Box;

use jsprint::JSprint;

pub type CommandFnDef = Fn(&mut JSprint, &str);
pub type CommandFn = Box<CommandFnDef>;
