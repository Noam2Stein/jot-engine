use std::sync::Arc;

use super::*;

#[derive(Debug, Clone)]
pub struct GPUContext {
    pub device: Arc<GPUDevice>,
    pub queue: Arc<GPUQueue>,
}
