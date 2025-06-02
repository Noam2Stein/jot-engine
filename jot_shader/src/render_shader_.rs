use super::*;

shader_interface! {
    pub render_shader:

    type Vertex;
    type Fragment;

    fn vertex_main(input: Self::Vertex) -> Self::Fragment;

    fn fragment_main(input: Self::Fragment) -> FVec4;
}
