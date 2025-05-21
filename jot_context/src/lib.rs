use std::sync::Arc;

use wgpu::{Device, Queue, TextureFormat};
use winit::window::Window;

#[derive(Debug, Clone)]
pub struct Context {
    pub window: Arc<Window>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface_format: TextureFormat,
}
