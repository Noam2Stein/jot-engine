use jot::asset::*;

fn main() {
    let mut builder = AssetBuilder::default();

    builder.set_ext_default("png", "texture");
    builder.set_type::<jot::gpu::GpuTexture<2>>("texture");

    builder.build();
}
