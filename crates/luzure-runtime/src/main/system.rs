use luzure_ecs::Registry;
use luzure_input::input::InputState;

use crate::FrameTime;

pub type MainSystem = fn(&mut Registry, &InputState, &FrameTime);
