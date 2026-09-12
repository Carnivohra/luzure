use luzure_ecs::Registry;
use luzure_render::{MeshHandle, MeshInstance, MeshRender, RenderScene, render::RenderError};
use luzure_world::Transform;

pub(super) fn extract_meshes(registry: &Registry, scene: &mut RenderScene, instances: &mut Vec<(MeshHandle, MeshInstance)>)
    -> Result<(), RenderError>
{
    instances.clear();

    for (_, transform, mesh) in registry.query_pair::<Transform, MeshRender>() {
        instances.push((mesh.mesh(), MeshInstance::new(*transform.matrix())));
    }

    instances.sort_unstable_by_key(|(mesh, _)| mesh.value());

    for (mesh, instance) in instances.iter().copied() {
        scene.push_instance(mesh, instance)?;
    }

    Ok(())
}
