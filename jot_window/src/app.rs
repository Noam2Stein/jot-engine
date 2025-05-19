use std::sync::Arc;

use wgpu::{Device, Queue, SurfaceTexture, TextureFormat};
use winit::window::{Window, WindowAttributes};

use super::*;

pub trait App {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default()
    }

    fn new(_ctx: &AppContext) -> Self;

    fn update(&mut self, _delta_time: f64, _ctx: &AppContext) -> AppFlow {
        AppFlow::Continue
    }

    fn event(&mut self, _event: &Event, _ctx: &AppContext) -> AppFlow {
        match _event {
            Event::ExitRequested => AppFlow::Exit,
            _ => AppFlow::Continue,
        }
    }

    fn draw(&mut self, _frame: &mut SurfaceTexture, _ctx: &AppContext) {}
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppFlow {
    Continue,
    Exit,
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub window: Arc<Window>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface_format: TextureFormat,
}

impl From<&Event> for AppFlow {
    fn from(value: &Event) -> Self {
        match value {
            Event::ExitRequested => Self::Exit,
            _ => Self::Continue,
        }
    }
}
