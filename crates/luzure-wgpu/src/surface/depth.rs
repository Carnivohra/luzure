use wgpu::{Device, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};

pub(crate) struct WgpuDepth {
    view: TextureView,
}

impl WgpuDepth {
    pub(crate) const FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub(crate) fn new(device: &Device, size: (u32, u32)) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("luzure-wgpu depth texture"),
            size: Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());

        Self { view }
    }

    pub(crate) const fn view(&self) -> &TextureView {
        &self.view
    }
}
