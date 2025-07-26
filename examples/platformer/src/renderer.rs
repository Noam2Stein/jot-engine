use super::*;

pub type Renderer = Renderer2D<1000, Colored, SVec2P, PosCamera2D>;

pub type RenderInput<'a> = RenderInput2D<'a, Colored, SVec2P, PosCamera2D>;

pub type Quad = Quad2D<Colored, SVec2P>;
