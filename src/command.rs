use std::boxed::Box;

use crate::jsprint::JSprint;

pub type CommandFn = Box<Fn(&mut JSprint, &str)>;
