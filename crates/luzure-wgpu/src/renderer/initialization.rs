use luzure_render::render::RenderError;
use wgpu::{DeviceDescriptor, Instance, RequestAdapterOptions};

use std::{future::Future, pin::Pin, rc::Rc};

use crate::surface::WgpuSurfaceTarget;

use super::WgpuRendererState;

pub(super) type WgpuRendererInitialization = Pin<Box<dyn Future<Output = Result<WgpuRendererState, RenderError>>>>;

pub(super) fn initialize(instance: Instance, target: Rc<WgpuSurfaceTarget>) -> WgpuRendererInitialization {
    Box::pin(async move {
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(target.surface()), ..Default::default()
        }).await.map_err(|_| RenderError::AdapterRequest)?;

        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            label: Some("luzure-wgpu device"), ..Default::default()
        }).await.map_err(|_| RenderError::DeviceRequest)?;

        Ok(WgpuRendererState::new(adapter, device, queue))
    })
}
