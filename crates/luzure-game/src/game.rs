pub mod metadata;

use metadata::GameMetadata;

pub trait Game {
    fn metadata(&self) -> GameMetadata;
}
