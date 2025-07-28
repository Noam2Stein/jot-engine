use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use image::{DynamicImage, EncodableLayout, ImageFormat};
use quote::quote;
use serde::{Deserialize, Serialize};

use super::*;

impl AssetType for GpuTexture<2> {
    type Metadata = ();

    fn type_path() -> proc_macro2::TokenStream {
        quote! { ::jot::gpu::GpuTexture<2> }
    }

    fn build(path: &std::path::Path, _meta: Option<Self::Metadata>, dst_path: &std::path::Path) {
        let image = load_image(path);

        let bytes = bitcode::serialize(&TextureFile {
            width: image.width(),
            height: image.height(),
            buf: image.to_rgba8().into_vec(),
        })
        .unwrap();

        File::create(dst_path).unwrap().write_all(&bytes).unwrap();
    }

    fn import(path: &std::path::Path, _meta: Option<Self::Metadata>, gpu: &Gpu) -> Self {
        let image = load_image(path);

        gpu.create_texture(&GpuTextureDesc {
            size: vec2p!(image.width(), image.height()),
            color_format: Some(GpuTextureFormat::Rgba8Unorm),
            color_data: Some(image.to_rgba8().as_bytes()),
            depth_format: None,
            depth_data: None,
            usages: GpuTextureUsages::COPY_SRC | GpuTextureUsages::TEXTURE_BINDING,
        })
    }

    fn load(path: &std::path::Path, gpu: &Gpu) -> Self {
        let bytes = File::open(path)
            .unwrap()
            .bytes()
            .map(|bytes| bytes.unwrap())
            .collect::<Vec<u8>>();

        let TextureFile { width, height, buf } = bitcode::deserialize(&bytes).unwrap();

        gpu.create_texture(&GpuTextureDesc {
            size: vec2p!(width, height),
            color_format: Some(GpuTextureFormat::Rgba8Unorm),
            color_data: Some(&buf),
            depth_format: None,
            depth_data: None,
            usages: GpuTextureUsages::COPY_SRC | GpuTextureUsages::TEXTURE_BINDING,
        })
    }
}

fn load_image(path: &std::path::Path) -> DynamicImage {
    let format = match path.extension().unwrap().to_str().unwrap() {
        "png" => ImageFormat::Png,
        "jpeg" => ImageFormat::Jpeg,

        _ => unreachable!(),
    };

    let r = File::open(path).unwrap();

    let image = image::load(BufReader::new(r), format).expect("failed to deserialize image");

    image
}

#[derive(Serialize, Deserialize)]
struct TextureFile {
    width: u32,
    height: u32,
    buf: Vec<u8>,
}
