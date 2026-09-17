use luzure_ecs::Registry;
use luzure_render::{CameraMatrices, RenderView};

use crate::{Camera, render::{CameraView, render_target}};

pub(crate) fn extract_cameras(registry: &Registry, views: &mut Vec<RenderView>) {
    views.clear();

    for (_, view) in registry.query::<CameraView>() {
        let Some(camera) = registry.get::<Camera>(view.camera()) else {
            continue;
        };

        views.push(RenderView::new(
            render_target(view.target()),
            CameraMatrices::new(
                *camera.view(),
                *camera.projection(),
            ),
            view.viewport(),
            view.order(),
        ));
    }

    views.sort_unstable_by_key(|view| (view.target().value(), view.order()));
}
