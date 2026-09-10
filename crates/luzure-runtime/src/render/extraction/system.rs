use luzure_ecs::Registry;
use luzure_render::{RenderScene, render::RenderError};

pub type RenderExtractSystem = fn(&Registry, &mut RenderScene) -> Result<(), RenderError>;
