use std::boxed::Box;

use crate::jsprint::JSprint;

pub type CommandFn = Box<dyn Fn(&mut JSprint, &str)>;
