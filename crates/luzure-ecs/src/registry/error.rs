use crate::Entity;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum RegistryError {
    #[error("entity {entity:?} does not exist")]
    EntityNotFound { entity: Entity },
}
