use super::*;

pub trait HotReload {
    fn hot_reload(&mut self, _gpu: &Gpu);
}
