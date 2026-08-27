use wgpu::{Adapter, Device, Queue};

pub(super) struct WgpuRendererState {
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl WgpuRendererState {
    pub(super) fn new(adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self {
            adapter,
            device,
            queue,
        }
    }

    pub(super) const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub(super) const fn device(&self) -> &Device {
        &self.device
    }

    pub(super) const fn queue(&self) -> &Queue {
        &self.queue
    }
}
