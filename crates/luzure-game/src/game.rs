pub mod metadata;

use metadata::GameMetadata;

pub trait Game {
    type Plugins;

    fn metadata(&self) -> GameMetadata;
    fn plugins(&mut self) -> Self::Plugins;
}
