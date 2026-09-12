use luzure_ecs::Registry;
use luzure_render::{MeshInstance, MeshRender, RenderScene, render::RenderError};
use luzure_world::Transform;

pub(super) fn extract_meshes(registry: &Registry, scene: &mut RenderScene) -> Result<(), RenderError> {
    for (_, transform, mesh) in registry.query_pair::<Transform, MeshRender>() {
        scene.push_batch(mesh.mesh(), &[MeshInstance::new(*transform.matrix())])?;
    }

    Ok(())
}
