use crate::Registry;

use std::time::Duration;

pub type System = fn(&mut Registry, Duration);
