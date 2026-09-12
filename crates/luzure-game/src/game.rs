pub(crate) mod metadata;

use metadata::GameMetadata;

pub trait Game {
    type Plugins;

    const METADATA: GameMetadata;

    fn plugins(&mut self) -> Self::Plugins;
}
