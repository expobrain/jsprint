use jsprint::JSprint;
use std::boxed::Box;

pub type CommandFn = Box<Fn(&mut JSprint, &str)>;
